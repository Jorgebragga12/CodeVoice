use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use specta::Type;

use super::TranscribeError;

/// Um modelo Whisper disponível para download. `sha256` e `size` são fixados no binário
/// (obtidos dos ponteiros LFS do HuggingFace em 24/07/2026) — é o que permite verificar a
/// integridade do arquivo baixado contra adulteração/corrupção (SECURITY-MODEL §2).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
pub struct ModelInfo {
    /// Identificador estável usado em settings/URL (ex.: "large-v3-turbo").
    pub id: &'static str,
    /// Nome amigável para a UI.
    pub label: &'static str,
    pub filename: &'static str,
    pub url: &'static str,
    pub sha256: &'static str,
    pub size_bytes: u64,
}

/// Catálogo de modelos. `large-v3-turbo` é o padrão travado pelo ADR-001; os demais são
/// alternativas (o q5_0 é ~1/3 do tamanho com qualidade quase idêntica; small/medium para
/// hardware mais fraco).
pub const MODELS: &[ModelInfo] = &[
    ModelInfo {
        id: "large-v3-turbo",
        label: "Large v3 Turbo (padrão, melhor qualidade) — 1.6 GB",
        filename: "ggml-large-v3-turbo.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo.bin",
        sha256: "1fc70f774d38eb169993ac391eea357ef47c88757ef72ee5943879b7e8e2bc69",
        size_bytes: 1_624_555_275,
    },
    ModelInfo {
        id: "large-v3-turbo-q5_0",
        label: "Large v3 Turbo quantizado (rápido, quase igual) — 574 MB",
        filename: "ggml-large-v3-turbo-q5_0.bin",
        url:
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo-q5_0.bin",
        sha256: "394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2",
        size_bytes: 574_041_195,
    },
    ModelInfo {
        id: "medium",
        label: "Medium — 1.5 GB",
        filename: "ggml-medium.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        sha256: "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208",
        size_bytes: 1_533_763_059,
    },
    ModelInfo {
        id: "small",
        label: "Small (leve, hardware modesto) — 488 MB",
        filename: "ggml-small.bin",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        sha256: "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b",
        size_bytes: 487_601_967,
    },
];

pub const DEFAULT_MODEL_ID: &str = "large-v3-turbo";

pub fn find_model(id: &str) -> Option<&'static ModelInfo> {
    MODELS.iter().find(|m| m.id == id)
}

pub fn default_model() -> &'static ModelInfo {
    find_model(DEFAULT_MODEL_ID).expect("modelo padrão precisa existir no catálogo")
}

pub fn models_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("models")
}

pub fn model_path(app_data_dir: &Path, model: &ModelInfo) -> PathBuf {
    models_dir(app_data_dir).join(model.filename)
}

/// Checagem rápida (sem hashear): o arquivo existe e tem o tamanho esperado. Bom o suficiente
/// para decidir se precisa baixar; a verificação forte de SHA-256 acontece ao final do download.
pub fn is_downloaded(app_data_dir: &Path, model: &ModelInfo) -> bool {
    std::fs::metadata(model_path(app_data_dir, model))
        .map(|m| m.len() == model.size_bytes)
        .unwrap_or(false)
}

fn sha256_of(path: &Path) -> Result<String, TranscribeError> {
    let mut file = std::fs::File::open(path).map_err(|source| TranscribeError::Io {
        path: path.display().to_string(),
        source,
    })?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1 << 16];
    loop {
        let n = file.read(&mut buf).map_err(|source| TranscribeError::Io {
            path: path.display().to_string(),
            source,
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Baixa `model` para o diretório de modelos, reportando progresso em porcentagem (0–100), e
/// **verifica o SHA-256** antes de considerar o arquivo válido.
///
/// Estratégia contra corrupção (SECURITY-MODEL §2 "modelo adulterado"): baixa para `<arquivo>.part`,
/// confere o hash, e só então renomeia para o nome final. Um download interrompido nunca deixa
/// um `.bin` inválido no lugar do bom — no máximo um `.part` órfão, que o próximo download
/// sobrescreve. Hash divergente → apaga e erro claro (a UI oferece tentar de novo).
pub fn download(
    app_data_dir: &Path,
    model: &ModelInfo,
    on_progress: &(dyn Fn(u8) + Send + Sync),
) -> Result<(), TranscribeError> {
    let dir = models_dir(app_data_dir);
    std::fs::create_dir_all(&dir).map_err(|source| TranscribeError::Io {
        path: dir.display().to_string(),
        source,
    })?;

    let final_path = dir.join(model.filename);
    let part_path = dir.join(format!("{}.part", model.filename));

    let client = reqwest::blocking::Client::builder()
        .timeout(None) // downloads grandes; o próprio stream detecta queda de conexão
        .build()
        .map_err(|e| TranscribeError::Processing(e.to_string()))?;

    let mut response = client
        .get(model.url)
        .send()
        .map_err(|e| TranscribeError::Processing(format!("falha ao conectar: {e}")))?
        .error_for_status()
        .map_err(|e| TranscribeError::Processing(format!("resposta inválida do servidor: {e}")))?;

    let total = response.content_length().unwrap_or(model.size_bytes).max(1);

    let mut out = std::fs::File::create(&part_path).map_err(|source| TranscribeError::Io {
        path: part_path.display().to_string(),
        source,
    })?;

    let mut downloaded: u64 = 0;
    let mut last_percent: u8 = 0;
    let mut buf = [0u8; 1 << 16];
    on_progress(0);

    loop {
        let n = response
            .read(&mut buf)
            .map_err(|e| TranscribeError::Processing(e.to_string()))?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n])
            .map_err(|source| TranscribeError::Io {
                path: part_path.display().to_string(),
                source,
            })?;
        downloaded += n as u64;

        let percent = ((downloaded.min(total) * 100) / total) as u8;
        if percent != last_percent {
            last_percent = percent;
            on_progress(percent);
        }
    }
    out.flush().map_err(|source| TranscribeError::Io {
        path: part_path.display().to_string(),
        source,
    })?;
    drop(out);

    // Verificação de integridade: o arquivo baixado tem que bater exatamente com o hash fixado.
    let actual = sha256_of(&part_path)?;
    if !actual.eq_ignore_ascii_case(model.sha256) {
        let _ = std::fs::remove_file(&part_path);
        return Err(TranscribeError::Processing(format!(
            "verificação de integridade falhou para {} (o arquivo pode estar corrompido; tente baixar de novo)",
            model.label
        )));
    }

    std::fs::rename(&part_path, &final_path).map_err(|source| TranscribeError::Io {
        path: final_path.display().to_string(),
        source,
    })?;
    on_progress(100);
    Ok(())
}

/// Apaga um modelo baixado (usado quando o usuário troca de modelo ou quer liberar espaço).
pub fn delete_model(app_data_dir: &Path, model: &ModelInfo) -> Result<(), TranscribeError> {
    let path = model_path(app_data_dir, model);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|source| TranscribeError::Io {
            path: path.display().to_string(),
            source,
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_is_in_catalog_and_is_large_v3_turbo() {
        assert_eq!(default_model().id, "large-v3-turbo");
    }

    #[test]
    fn every_model_has_a_plausible_pinned_hash_and_size() {
        for m in MODELS {
            assert_eq!(m.sha256.len(), 64, "sha256 de {} não tem 64 hex", m.id);
            assert!(m.sha256.chars().all(|c| c.is_ascii_hexdigit()));
            assert!(m.size_bytes > 100_000_000, "tamanho suspeito para {}", m.id);
            assert!(m.url.starts_with("https://"), "URL de {} não é https", m.id);
        }
    }

    #[test]
    fn model_ids_are_unique() {
        for (i, a) in MODELS.iter().enumerate() {
            for b in &MODELS[i + 1..] {
                assert_ne!(a.id, b.id, "id duplicado: {}", a.id);
            }
        }
    }

    #[test]
    fn is_downloaded_false_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!is_downloaded(dir.path(), default_model()));
    }

    #[test]
    fn is_downloaded_checks_size_not_just_presence() {
        let dir = tempfile::tempdir().unwrap();
        let model = default_model();
        std::fs::create_dir_all(models_dir(dir.path())).unwrap();
        // Arquivo com nome certo mas tamanho errado (download truncado) não conta como pronto.
        std::fs::write(model_path(dir.path(), model), b"tamanho errado").unwrap();
        assert!(!is_downloaded(dir.path(), model));
    }

    #[test]
    fn sha256_matches_known_vector() {
        // SHA-256 de "abc" (vetor de teste padrão) — prova que nosso hasher/hex estão corretos.
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("abc.txt");
        std::fs::write(&f, b"abc").unwrap();
        assert_eq!(
            sha256_of(&f).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
