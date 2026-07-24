use crate::projects::scanner::{self, ImportPreview};
use crate::security::path_validation;

/// Valida e canonicaliza um caminho digitado pelo usuário, sem tocar no banco. Usado pelo
/// frontend pra dar feedback imediato (ex.: ao sair do campo de caminho) antes de tentar
/// importar ou salvar o projeto.
#[tauri::command]
#[specta::specta]
pub fn validate_project_path(path: String) -> Result<String, String> {
    path_validation::validate_project_root(&path)
        .map(|canonical| canonical.display().to_string())
        .map_err(|e| e.to_string())
}

/// Importação assistida (SECURITY-MODEL.md §3): só lê arquivos da allowlist + lista nomes de
/// diretórios (2 níveis). Não salva nada — devolve um preview pra UI mostrar antes do usuário
/// confirmar. Só deve ser chamado sob ação explícita (botão "Pré-visualizar importação").
#[tauri::command]
#[specta::specta]
pub fn preview_project_import(path: String) -> Result<ImportPreview, String> {
    scanner::scan_project(&path).map_err(|e| e.to_string())
}
