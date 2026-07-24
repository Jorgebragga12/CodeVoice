//! Importação assistida de projetos (SECURITY-MODEL.md §3).
//!
//! Lê **somente** os arquivos da allowlist e lista nomes de diretórios até 2 níveis de
//! profundidade, respeitando a denylist. Nunca escreve no banco — devolve um preview pro
//! frontend decidir o que fazer. Só deve ser chamado sob ação explícita do usuário (botão
//! "Pré-visualizar importação", separado do botão "Salvar" — nunca automático).

use std::path::Path;

use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

use crate::security::path_validation::{self, PathValidationError};

/// Limite de tamanho por arquivo lido pela importação assistida.
pub const MAX_FILE_SIZE_BYTES: u64 = 512 * 1024;

/// Profundidade máxima (em níveis de subdiretório a partir da raiz) percorrida pelo scanner.
const MAX_DEPTH: usize = 2;

/// Nomes de arquivo (case-insensitive) permitidos na importação assistida. `docker-compose*.yml`
/// é tratado à parte por ser um padrão, não um nome fixo (ver [`is_docker_compose_file`]).
const ALLOWED_FILENAMES: &[&str] = &[
    "claude.md",
    "readme.md",
    "package.json",
    "tsconfig.json",
    "cargo.toml",
    "pyproject.toml",
    "composer.json",
    "go.mod",
    ".nvmrc",
];

const DENYLISTED_DIR_NAMES: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    "target",
    ".next",
    "out",
    "coverage",
    "venv",
    ".venv",
    "__pycache__",
];

const DENYLISTED_NAME_SUBSTRINGS: &[&str] = &["secret", "credential", "token"];
const DENYLISTED_SUFFIXES: &[&str] = &[".pem", ".key", ".pfx", ".p12", ".crt"];

#[derive(Debug, Error)]
pub enum ScanError {
    #[error(transparent)]
    Path(#[from] PathValidationError),
    #[error("erro de I/O em {path}: {source}")]
    Io { path: String, source: std::io::Error },
}

/// Um arquivo lido pela importação assistida (só arquivos da allowlist chegam aqui).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ImportedFile {
    pub relative_path: String,
    pub size_bytes: i32,
    pub content: String,
}

/// Resultado da importação assistida: nada foi salvo, é só um preview do que *seria* lido.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ImportPreview {
    pub root: String,
    pub files: Vec<ImportedFile>,
    pub directories: Vec<String>,
}

fn is_docker_compose_file(lower_name: &str) -> bool {
    lower_name.starts_with("docker-compose") && lower_name.ends_with(".yml")
}

fn is_allowed_filename(name: &str) -> bool {
    let lower = name.to_lowercase();
    ALLOWED_FILENAMES.contains(&lower.as_str()) || is_docker_compose_file(&lower)
}

/// Diretório é recusado (nem listado no preview, nem percorrido) se o nome bater com a lista
/// técnica fixa (`.git`, `node_modules`, ...) OU com as mesmas heurísticas de nome sensível
/// (`.env*`, `id_rsa*`, substring `secret`/`credential`/`token`, sufixos de chave/certificado)
/// aplicadas a arquivos por [`is_denied_by_name`]. Sem isso, um diretório literalmente chamado
/// `secrets/` ou `.credentials/` não era barrado: seu nome aparecia no preview e ele continuava
/// sendo percorrido normalmente, expondo o conteúdo de qualquer arquivo allowlisted (README.md,
/// package.json, ...) que estivesse dentro dele.
fn is_denylisted_dir(name: &str) -> bool {
    DENYLISTED_DIR_NAMES.contains(&name.to_lowercase().as_str()) || is_denied_by_name(name)
}

/// Denylist absoluta (SECURITY-MODEL.md §3): nunca ler `.env*`, chaves/certificados privados,
/// nem arquivos cujo nome contenha `secret`/`credential`/`token`. Verificado *antes* (e
/// independentemente) da allowlist — defesa em profundidade.
fn is_denied_by_name(name: &str) -> bool {
    let lower = name.to_lowercase();

    if lower == ".env" || lower.starts_with(".env.") {
        return true;
    }
    if lower.starts_with("id_rsa") {
        return true;
    }
    if DENYLISTED_NAME_SUBSTRINGS.iter().any(|needle| lower.contains(needle)) {
        return true;
    }
    DENYLISTED_SUFFIXES.iter().any(|suffix| lower.ends_with(suffix))
}

fn relative_path(root: &Path, full: &Path) -> String {
    full.strip_prefix(root).unwrap_or(full).to_string_lossy().replace('\\', "/")
}

/// Detecta se `file` corresponde a um arquivo com mais de um "hard link" — outro nome de
/// caminho, em qualquer lugar do mesmo volume, apontando pros mesmos dados físicos no disco
/// (ex.: `std::fs::hard_link`, `mklink /H` no Windows, `ln` no Unix). Criar um hardlink NÃO
/// exige privilégio elevado nem Modo Desenvolvedor.
///
/// Isso é diferente de um symlink: não há nenhum "destino" pro SO reportar, então
/// `DirEntry::file_type().is_symlink()` retorna `false` (corretamente — não é reparse point) e
/// `ensure_within_root` nunca é acionado. Um hardlink criado sob um nome allowlisted (ex.:
/// `README.md`) dentro da raiz do projeto, mas fisicamente apontando para um arquivo fora dela
/// (`.env`, chave privada, etc.), tem seu conteúdo lido e devolvido no preview sem qualquer
/// checagem pegar isso — a denylist por nome (`is_denied_by_name`) só vê o nome do link
/// escolhido pelo atacante, nunca o nome/local do arquivo físico de origem.
///
/// Como não existe forma portátil de enumerar todos os nomes que apontam pro mesmo arquivo
/// (para confirmar que todos ficam dentro da raiz), qualquer arquivo com mais de 1 link é
/// tratado como não confiável e tem sua leitura recusada — mesma postura adotada para um
/// symlink que escapa da raiz.
///
/// `std::os::windows::fs::MetadataExt::number_of_links` cobriria isso de forma mais simples,
/// mas continua atrás da feature instável `windows_by_handle` em toolchains stable — por isso
/// chamamos `GetFileInformationByHandle` diretamente via `windows-sys`.
#[cfg(windows)]
fn has_multiple_hard_links(file: &std::fs::File) -> bool {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    };

    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    // SAFETY: `file.as_raw_handle()` é um HANDLE válido e aberto (o `File` ainda vive nesse
    // escopo, então o handle não foi fechado) e `info` é um buffer de saída do tamanho exato
    // esperado pela API, alocado logo acima.
    let ok = unsafe {
        GetFileInformationByHandle(file.as_raw_handle() as _, &mut info as *mut _)
    };
    ok != 0 && info.nNumberOfLinks > 1
}

#[cfg(unix)]
fn has_multiple_hard_links(file: &std::fs::File) -> bool {
    use std::os::unix::fs::MetadataExt;
    file.metadata().map(|m| m.nlink() > 1).unwrap_or(false)
}

#[cfg(not(any(windows, unix)))]
fn has_multiple_hard_links(_file: &std::fs::File) -> bool {
    false
}

/// Executa a importação assistida: valida/canonicaliza `root_raw` (rejeita travessia, UNC,
/// symlink-fora-do-projeto etc. — ver `security::path_validation`), então percorre até
/// `MAX_DEPTH` níveis de subdiretórios lendo somente arquivos da allowlist e listando nomes de
/// diretórios (pulando os da denylist). Não escreve nada no banco.
pub fn scan_project(root_raw: &str) -> Result<ImportPreview, ScanError> {
    let root = path_validation::validate_project_root(root_raw)?;

    // A raiz precisa ser legível; erros em subdiretórios mais fundo (permissão negada, etc.)
    // são ignorados silenciosamente por `walk` — uma subpasta problemática não deve derrubar
    // o preview inteiro.
    std::fs::read_dir(&root)
        .map_err(|source| ScanError::Io { path: root.display().to_string(), source })?;

    let mut files = Vec::new();
    let mut directories = Vec::new();
    walk(&root, &root, 1, &mut files, &mut directories);

    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    directories.sort();

    Ok(ImportPreview { root: root.display().to_string(), files, directories })
}

/// Percorre `dir` (que está a `depth` níveis da raiz) coletando arquivos permitidos e nomes de
/// diretórios. `depth` do 1º nível de subpastas é 1; recursão para quando `depth == MAX_DEPTH`.
fn walk(
    root: &Path,
    dir: &Path,
    depth: usize,
    files: &mut Vec<ImportedFile>,
    directories: &mut Vec<String>,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Ok(file_type) = entry.file_type() else { continue };

        if file_type.is_dir() {
            if is_denylisted_dir(&name) {
                continue;
            }
            directories.push(relative_path(root, &entry.path()));

            if depth < MAX_DEPTH {
                // Diretório-symlink: só desce se o alvo continuar dentro da raiz do projeto.
                if file_type.is_symlink()
                    && path_validation::ensure_within_root(root, &entry.path()).is_err()
                {
                    continue;
                }
                walk(root, &entry.path(), depth + 1, files, directories);
            }
            continue;
        }

        if !file_type.is_file() && !file_type.is_symlink() {
            continue; // dispositivos/pipes/etc. — nunca importados
        }

        if is_denied_by_name(&name) || !is_allowed_filename(&name) {
            continue;
        }

        let real_path = if file_type.is_symlink() {
            match path_validation::ensure_within_root(root, &entry.path()) {
                Ok(resolved) => resolved,
                Err(_) => continue, // symlink escapando do projeto: rejeitado
            }
        } else {
            entry.path()
        };

        // Abre um único handle e reusa pra metadata, checagem de hardlink e leitura — evita
        // reabrir o arquivo (e uma pequena janela TOCTOU) entre essas três operações.
        let Ok(mut file) = std::fs::File::open(&real_path) else { continue };
        let Ok(metadata) = file.metadata() else { continue };
        if !metadata.is_file() || metadata.len() > MAX_FILE_SIZE_BYTES {
            continue;
        }
        if has_multiple_hard_links(&file) {
            continue; // hardlink disfarçado sob nome allowlisted: recusa (ver has_multiple_hard_links)
        }

        let mut bytes = Vec::new();
        let Ok(_) = std::io::Read::read_to_end(&mut file, &mut bytes) else { continue };
        // Não-UTF8 é tratado como binário/mídia e descartado (a allowlist só cobre texto).
        let Ok(content) = String::from_utf8(bytes) else { continue };

        files.push(ImportedFile {
            relative_path: relative_path(root, &entry.path()),
            size_bytes: metadata.len() as i32,
            content,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn ignores_env_file_at_root() {
        let dir = tempfile::tempdir().unwrap();
        write_file(&dir.path().join(".env"), "SECRET_KEY=supersecreto");
        write_file(&dir.path().join("README.md"), "# projeto");

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();
        let names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();

        assert!(names.contains(&"README.md"));
        assert!(!names.iter().any(|n| n.to_lowercase().contains(".env")));
    }

    #[test]
    fn ignores_files_with_secret_credential_or_token_in_the_name() {
        let dir = tempfile::tempdir().unwrap();
        write_file(&dir.path().join("secrets.json"), "{}");
        write_file(&dir.path().join("db-credential.txt"), "user/pass");
        write_file(&dir.path().join("package.json"), "{\"name\":\"codevoice\"}");

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();
        let names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();

        assert!(names.contains(&"package.json"));
        assert!(!names.contains(&"secrets.json"));
        assert!(!names.contains(&"db-credential.txt"));
    }

    #[test]
    fn rejects_path_with_traversal_segment() {
        let err = scan_project(r"C:\projects\..\Windows");
        assert!(matches!(err, Err(ScanError::Path(PathValidationError::Traversal))));
    }

    #[test]
    fn rejects_unc_path() {
        let err = scan_project(r"\\server\share\projeto");
        assert!(matches!(err, Err(ScanError::Path(PathValidationError::UnsupportedUnc(_)))));
    }

    #[test]
    fn ignores_file_larger_than_512kb() {
        let dir = tempfile::tempdir().unwrap();
        let big_content = "a".repeat(MAX_FILE_SIZE_BYTES as usize + 1024);
        write_file(&dir.path().join("package.json"), &big_content);
        write_file(&dir.path().join("README.md"), "# pequeno o suficiente");

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();
        let names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();

        assert!(!names.contains(&"package.json"), "arquivo > 512KB deveria ser ignorado");
        assert!(names.contains(&"README.md"));
    }

    #[test]
    fn rejects_symlink_pointing_outside_project_root() {
        let outside = tempfile::tempdir().unwrap();
        let outside_file = outside.path().join("segredo.md");
        fs::write(&outside_file, "não deveria ser lido").unwrap();

        let project = tempfile::tempdir().unwrap();
        let link_path = project.path().join("README.md");

        #[cfg(windows)]
        let created = std::os::windows::fs::symlink_file(&outside_file, &link_path).is_ok();
        #[cfg(not(windows))]
        let created = std::os::unix::fs::symlink(&outside_file, &link_path).is_ok();

        if !created {
            eprintln!(
                "aviso: sem privilégio para criar symlink neste ambiente — teste pulado, não é \
                 uma falha da lógica de rejeição"
            );
            return;
        }

        let preview = scan_project(project.path().to_str().unwrap()).unwrap();
        assert!(preview.files.is_empty(), "symlink apontando pra fora não deveria ser importado");
    }

    #[cfg(windows)]
    #[test]
    fn adversarial_junction_escapes_root_without_privilege() {
        // Diferente de symlinks, junctions NTFS (`mklink /J`) não exigem
        // SeCreateSymbolicLinkPrivilege / Modo Desenvolvedor — qualquer usuário padrão pode
        // criar uma. `walk()` só chama `ensure_within_root` quando `file_type.is_symlink()` é
        // verdadeiro; uma junction tem FILE_ATTRIBUTE_REPARSE_POINT mas com o reparse tag
        // IO_REPARSE_TAG_MOUNT_POINT (não IO_REPARSE_TAG_SYMLINK), então
        // `DirEntry::file_type().is_symlink()` retorna `false` para ela — o check é pulado.
        let outside = tempfile::tempdir().unwrap();
        let outside_file = outside.path().join("README.md");
        fs::write(&outside_file, "SEGREDO fora do projeto: nao deveria vazar").unwrap();

        let project = tempfile::tempdir().unwrap();
        let link_path = project.path().join("link_dir");

        let status = std::process::Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                link_path.to_str().unwrap(),
                outside.path().to_str().unwrap(),
            ])
            .status()
            .unwrap();

        if !status.success() {
            eprintln!("aviso: falha ao criar junction (mklink /J) — teste pulado");
            return;
        }

        // Confirma que o Rust std realmente NÃO marca a entrada da junction como symlink
        // (é essa a lacuna que o teste está explorando).
        let entry_is_symlink = fs::read_dir(project.path())
            .unwrap()
            .flatten()
            .find(|e| e.file_name() == "link_dir")
            .map(|e| e.file_type().unwrap().is_symlink())
            .unwrap();
        eprintln!(
            "DEBUG: DirEntry::file_type().is_symlink() para a junction = {entry_is_symlink}"
        );

        let preview = scan_project(project.path().to_str().unwrap()).unwrap();
        let names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();

        assert!(
            !names.iter().any(|n| n.replace('\\', "/") == "link_dir/README.md"),
            "BYPASS CONFIRMADO: scan_project leu {:?} através de uma directory junction, \
             vazando conteúdo de fora da raiz do projeto: {:?}",
            names,
            preview.files.iter().find(|f| f.relative_path.contains("README.md")).map(|f| &f.content)
        );
    }

    #[test]
    fn lists_directories_up_to_two_levels_and_skips_denylist() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src/components")).unwrap();
        fs::create_dir_all(dir.path().join("node_modules/whatever")).unwrap();

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();

        assert!(preview.directories.iter().any(|d| d == "src"));
        assert!(preview.directories.iter().any(|d| d == "src/components"));
        assert!(!preview.directories.iter().any(|d| d.contains("node_modules")));
    }

    /// REGRESSÃO (revisão adversarial): `is_denylisted_dir` só comparava o nome do diretório
    /// contra a lista técnica fixa (`.git`, `node_modules`, ...), nunca contra as heurísticas
    /// de nome sensível (`secret`/`credential`/`token`) usadas para arquivos. Um diretório
    /// chamado `secrets/` ou `.credentials/` tinha seu NOME exposto em `preview.directories` e
    /// continuava sendo percorrido normalmente, vazando o conteúdo de qualquer arquivo
    /// allowlisted (ex.: `package.json`, `README.md`) que estivesse dentro dele.
    #[test]
    fn regression_directory_with_sensitive_name_is_not_listed_nor_walked_into() {
        let dir = tempfile::tempdir().unwrap();
        write_file(&dir.path().join("secrets/package.json"), "{\"leaked\":true}");
        write_file(&dir.path().join(".credentials/README.md"), "# não deveria vazar");
        // Controle: um diretório comum continua listado e percorrido normalmente.
        write_file(&dir.path().join("src/README.md"), "# ok");

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();

        assert!(
            !preview.directories.iter().any(|d| d.to_lowercase().contains("secret")),
            "BYPASS CONFIRMADO: nome de diretório sensível apareceu no preview: {:?}",
            preview.directories
        );
        assert!(
            !preview.directories.iter().any(|d| d.to_lowercase().contains("credential")),
            "BYPASS CONFIRMADO: nome de diretório sensível apareceu no preview: {:?}",
            preview.directories
        );
        assert!(
            !preview.files.iter().any(|f| f.relative_path.starts_with("secrets/")),
            "BYPASS CONFIRMADO: arquivo dentro de diretório 'secrets/' foi lido: {:?}",
            preview.files.iter().map(|f| &f.relative_path).collect::<Vec<_>>()
        );
        assert!(
            !preview.files.iter().any(|f| f.relative_path.starts_with(".credentials/")),
            "BYPASS CONFIRMADO: arquivo dentro de diretório '.credentials/' foi lido: {:?}",
            preview.files.iter().map(|f| &f.relative_path).collect::<Vec<_>>()
        );
        assert!(preview.directories.iter().any(|d| d == "src"));
        assert!(preview.files.iter().any(|f| f.relative_path == "src/README.md"));
    }

    /// REGRESSÃO (revisão adversarial): um NTFS hardlink (`std::fs::hard_link`, sem privilégio
    /// elevado) criado dentro da raiz do projeto sob um nome allowlisted (`README.md`), mas
    /// fisicamente apontando pro mesmo arquivo que um `segredo.txt` FORA da raiz, não é um
    /// symlink (`DirEntry::file_type().is_symlink()` == `false`, corretamente — não é reparse
    /// point) — então nem o check `ensure_within_root` nem a denylist por nome (que só vê o
    /// nome escolhido pelo atacante, "README.md") pegavam isso. `scan_project` lia e devolvia o
    /// conteúdo do arquivo externo, integralmente, sob o nome `README.md`.
    #[test]
    fn regression_hardlink_disguised_under_allowed_name_is_not_read() {
        let outside = tempfile::tempdir().unwrap();
        let outside_secret = outside.path().join("segredo-privado.txt");
        fs::write(&outside_secret, "SECRET-HARDLINK-CONTENT-marker-77aa").unwrap();

        let project = tempfile::tempdir().unwrap();
        let link_path = project.path().join("README.md");

        std::fs::hard_link(&outside_secret, &link_path)
            .expect("hard_link não deveria exigir privilégio elevado no mesmo volume");

        // Confirma a premissa do ataque: a API do SO não marca isso como symlink (é um arquivo
        // comum do ponto de vista do `DirEntry`), então a defesa "symlink pra fora da raiz"
        // nunca dispara pra este caso.
        let entry_is_symlink = fs::read_dir(project.path())
            .unwrap()
            .flatten()
            .find(|e| e.file_name() == "README.md")
            .map(|e| e.file_type().unwrap().is_symlink())
            .unwrap();
        assert!(!entry_is_symlink, "premissa do teste furou: hardlink foi reportado como symlink");

        let preview = scan_project(project.path().to_str().unwrap()).unwrap();
        let leaked = preview.files.iter().find(|f| f.relative_path == "README.md");

        assert!(
            leaked.is_none(),
            "BYPASS CONFIRMADO: conteúdo de arquivo fora da raiz do projeto foi lido através \
             de hardlink disfarçado sob nome allowlisted: {leaked:?}"
        );
    }

    #[test]
    fn reads_claude_md_readme_and_package_json() {
        let dir = tempfile::tempdir().unwrap();
        write_file(&dir.path().join("CLAUDE.md"), "# instrucoes");
        write_file(&dir.path().join("README.md"), "# leia-me");
        write_file(&dir.path().join("package.json"), "{}");

        let preview = scan_project(dir.path().to_str().unwrap()).unwrap();
        let names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();

        assert_eq!(names.len(), 3);
        assert!(names.contains(&"CLAUDE.md"));
        assert!(names.contains(&"README.md"));
        assert!(names.contains(&"package.json"));
    }
}
