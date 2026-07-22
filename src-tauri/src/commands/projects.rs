use tauri::State;

use crate::domain::{NewProject, Project, ProjectUpdate};
use crate::storage::{ProjectRepo, StorageError};

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

#[tauri::command]
#[specta::specta]
pub fn create_project(repo: State<'_, ProjectRepo>, input: NewProject) -> Result<Project, String> {
    repo.create(&input).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn update_project(
    repo: State<'_, ProjectRepo>,
    id: i32,
    input: ProjectUpdate,
) -> Result<Project, String> {
    repo.update(id, &input).map_err(err_to_string)
}

#[tauri::command]
#[specta::specta]
pub fn delete_project(repo: State<'_, ProjectRepo>, id: i32) -> Result<(), String> {
    repo.delete(id).map_err(err_to_string)
}
