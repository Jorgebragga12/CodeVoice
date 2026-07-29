use tauri::State;

use crate::domain::{
    NewProject, NewProjectRule, Project, ProjectRule, ProjectRuleUpdate, ProjectUpdate,
};
use crate::security::path_validation;
use crate::storage::{ProjectRepo, ProjectRuleRepo, StorageError};

fn err_to_string(err: StorageError) -> String {
    err.to_string()
}

#[tauri::command]
#[specta::specta]
pub fn list_projects(repo: State<'_, ProjectRepo>) -> Result<Vec<Project>, String> {
    repo.list().map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn get_project(repo: State<'_, ProjectRepo>, id: i32) -> Result<Option<Project>, String> {
    repo.get(id).map_err(err_to_string)
}

/// `input.path` chega como texto bruto do frontend — nunca confiar nele: valida e canonicaliza
/// (SECURITY-MODEL.md §3) antes de gravar, mesmo que a UI já tenha chamado `validate_project_path`
/// ou `preview_project_import` antes (defesa em profundidade, o backend nunca confia no frontend).
#[tauri::command]
#[specta::specta]
pub fn create_project(
    repo: State<'_, ProjectRepo>,
    mut input: NewProject,
) -> Result<Project, String> {
    let canonical =
        path_validation::validate_project_root(&input.path).map_err(|e| e.to_string())?;
    input.path = canonical.display().to_string();
    repo.create(&input).map_err(err_to_string)
}

/// `path` não é editável após a criação (ver `domain::ProjectUpdate` — não tem campo `path`),
/// então não há validação de caminho aqui.
#[tauri::command]
#[specta::specta]
pub fn update_project(
    repo: State<'_, ProjectRepo>,
    id: i32,
    input: ProjectUpdate,
) -> Result<Project, String> {
    repo.update(id, &input).map_err(err_to_string)
}

/// Exclusão real (sem lixeira — SECURITY-MODEL.md §5); a UI exige confirmação antes de chamar.
/// `prompt_history.project_id` usa `ON DELETE SET NULL`, então o histórico não fica órfão.
#[tauri::command]
#[specta::specta]
pub fn delete_project(repo: State<'_, ProjectRepo>, id: i32) -> Result<(), String> {
    repo.delete(id).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn list_project_rules(
    repo: State<'_, ProjectRuleRepo>,
    project_id: i32,
) -> Result<Vec<ProjectRule>, String> {
    repo.list_for_project(project_id).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn create_project_rule(
    repo: State<'_, ProjectRuleRepo>,
    input: NewProjectRule,
) -> Result<ProjectRule, String> {
    repo.create(&input).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn update_project_rule(
    repo: State<'_, ProjectRuleRepo>,
    id: i32,
    input: ProjectRuleUpdate,
) -> Result<ProjectRule, String> {
    repo.update(id, &input).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn delete_project_rule(repo: State<'_, ProjectRuleRepo>, id: i32) -> Result<(), String> {
    repo.delete(id).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn reorder_project_rules(
    repo: State<'_, ProjectRuleRepo>,
    project_id: i32,
    ordered_ids: Vec<i32>,
) -> Result<(), String> {
    repo.reorder(project_id, &ordered_ids)
        .map_err(err_to_string)
}
