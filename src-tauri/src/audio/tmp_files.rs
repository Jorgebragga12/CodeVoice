use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Áudio temporário vive só aqui (SECURITY-MODEL §2, linha "Áudio temporário"). Tudo neste
/// diretório é descartável por definição: é o que permite a limpeza cega de órfãos no startup.
pub fn tmp_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("tmp")
}

/// Nome único por gravação. Timestamp sozinho não basta — duas gravações no mesmo milissegundo
/// (ou um relógio que andou pra trás) colidiriam, e uma gravação sobrescreveria a outra —, daí
/// o contador de processo como desempate.
pub fn new_recording_path(tmp_dir: &Path) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);

    tmp_dir.join(format!("rec-{millis}-{seq}.wav"))
}

/// Apaga WAVs deixados para trás por uma execução anterior (crash, queda de energia, kill).
///
/// Um `.wav` neste diretório sempre significa gravação que nunca completou o processamento —
/// áudio com voz do usuário que deveria ter sido apagado. Deixar acumular contraria a promessa
/// de privacidade do PRODUCT-SPEC §6, então limpamos na inicialização.
///
/// Devolve quantos arquivos foram removidos. Erros individuais (arquivo travado por outro
/// processo, por ex.) são ignorados de propósito: falhar aqui não pode impedir o app de subir.
pub fn cleanup_orphans(tmp_dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(tmp_dir) else {
        return 0; // diretório ainda não existe: nada a limpar
    };

    let mut removed = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let is_wav = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("wav"));
        if is_wav && std::fs::remove_file(&path).is_ok() {
            removed += 1;
        }
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_dir_is_a_subfolder_of_app_data() {
        let dir = tmp_dir(Path::new("C:\\dados"));
        assert!(dir.ends_with("tmp"));
        assert!(dir.starts_with("C:\\dados"));
    }

    #[test]
    fn recording_paths_are_unique_and_wav() {
        let dir = Path::new("C:\\tmp");
        let a = new_recording_path(dir);
        let b = new_recording_path(dir);
        assert_ne!(a, b, "duas gravações não podem colidir no mesmo caminho");
        assert_eq!(a.extension().unwrap(), "wav");
    }

    #[test]
    fn cleanup_removes_leftover_wav_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("rec-1.wav"), b"x").unwrap();
        std::fs::write(dir.path().join("rec-2.wav"), b"x").unwrap();

        assert_eq!(cleanup_orphans(dir.path()), 2);
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[test]
    fn cleanup_leaves_non_wav_files_alone() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("rec.wav"), b"x").unwrap();
        std::fs::write(dir.path().join("anotacao.txt"), b"x").unwrap();

        assert_eq!(cleanup_orphans(dir.path()), 1);
        assert!(dir.path().join("anotacao.txt").exists());
    }

    #[test]
    fn cleanup_is_case_insensitive_about_the_extension() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("rec.WAV"), b"x").unwrap();

        assert_eq!(cleanup_orphans(dir.path()), 1);
    }

    #[test]
    fn cleanup_on_a_missing_directory_is_harmless() {
        assert_eq!(cleanup_orphans(Path::new("C:\\nao-existe-xyz-123")), 0);
    }
}
