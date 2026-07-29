use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{NewProject, Project, ProjectUpdate};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct ProjectRepo {
    pool: DbPool,
}

fn row_to_project(row: &Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get("id")?,
        name: row.get("name")?,
        path: row.get("path")?,
        description: row.get("description")?,
        stack: row.get("stack")?,
        architecture: row.get("architecture")?,
        dev_commands: row.get("dev_commands")?,
        test_commands: row.get("test_commands")?,
        forbidden_tech: row.get("forbidden_tech")?,
        database_info: row.get("database_info")?,
        notes: row.get("notes")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

const SELECT_COLUMNS: &str = "id, name, path, description, stack, architecture, dev_commands, \
     test_commands, forbidden_tech, database_info, notes, created_at, updated_at";

impl ProjectRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, input: &NewProject) -> Result<Project, StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO projects (name, path, description, stack, architecture, dev_commands, \
             test_commands, forbidden_tech, database_info, notes) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                input.name,
                input.path,
                input.description,
                input.stack,
                input.architecture,
                input.dev_commands,
                input.test_commands,
                input.forbidden_tech,
                input.database_info,
                input.notes,
            ],
        )?;
        let id = conn.last_insert_rowid() as i32;
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("project {id}")))
    }

    pub fn get(&self, id: i32) -> Result<Option<Project>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM projects WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_project)
            .optional()
            .map_err(Into::into)
    }

    pub fn list(&self) -> Result<Vec<Project>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM projects ORDER BY name COLLATE NOCASE");
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], row_to_project)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn update(&self, id: i32, input: &ProjectUpdate) -> Result<Project, StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute(
            "UPDATE projects SET name = ?1, description = ?2, stack = ?3, architecture = ?4, \
             dev_commands = ?5, test_commands = ?6, forbidden_tech = ?7, database_info = ?8, \
             notes = ?9, updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now') WHERE id = ?10",
            params![
                input.name,
                input.description,
                input.stack,
                input.architecture,
                input.dev_commands,
                input.test_commands,
                input.forbidden_tech,
                input.database_info,
                input.notes,
                id,
            ],
        )?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        drop(conn);
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("project {id}")))
    }

    pub fn delete(&self, id: i32) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::test_pool;

    fn sample() -> NewProject {
        NewProject {
            name: "CodeVoice".into(),
            path: "C:\\projects\\codevoice".into(),
            description: "app de ditado".into(),
            stack: "Tauri, React".into(),
            architecture: String::new(),
            dev_commands: "npm run tauri dev".into(),
            test_commands: "npm run test".into(),
            forbidden_tech: String::new(),
            database_info: "SQLite".into(),
            notes: String::new(),
        }
    }

    #[test]
    fn creates_and_gets_project() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());

        let created = repo.create(&sample()).unwrap();
        assert_eq!(created.name, "CodeVoice");
        assert!(created.id > 0);

        let fetched = repo.get(created.id).unwrap().unwrap();
        assert_eq!(fetched.path, "C:\\projects\\codevoice");
    }

    #[test]
    fn lists_projects_sorted_by_name() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());

        let mut b = sample();
        b.name = "Beta".into();
        b.path = "C:\\b".into();
        let mut a = sample();
        a.name = "Alpha".into();
        a.path = "C:\\a".into();

        repo.create(&b).unwrap();
        repo.create(&a).unwrap();

        let names: Vec<_> = repo.list().unwrap().into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["Alpha", "Beta"]);
    }

    #[test]
    fn updates_project() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());
        let created = repo.create(&sample()).unwrap();

        let updated = repo
            .update(
                created.id,
                &ProjectUpdate {
                    name: "CodeVoice Renomeado".into(),
                    description: "nova descrição".into(),
                    stack: created.stack.clone(),
                    architecture: created.architecture.clone(),
                    dev_commands: created.dev_commands.clone(),
                    test_commands: created.test_commands.clone(),
                    forbidden_tech: created.forbidden_tech.clone(),
                    database_info: created.database_info.clone(),
                    notes: created.notes.clone(),
                },
            )
            .unwrap();

        assert_eq!(updated.name, "CodeVoice Renomeado");
        assert_eq!(updated.description, "nova descrição");
        assert_ne!(updated.updated_at, "");
    }

    #[test]
    fn update_missing_project_returns_not_found() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());
        let err = repo.update(
            9999,
            &ProjectUpdate {
                name: "x".into(),
                description: String::new(),
                stack: String::new(),
                architecture: String::new(),
                dev_commands: String::new(),
                test_commands: String::new(),
                forbidden_tech: String::new(),
                database_info: String::new(),
                notes: String::new(),
            },
        );
        assert!(matches!(err, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn deletes_project() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());
        let created = repo.create(&sample()).unwrap();

        repo.delete(created.id).unwrap();

        assert!(repo.get(created.id).unwrap().is_none());
    }

    #[test]
    fn duplicate_path_is_rejected() {
        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());
        repo.create(&sample()).unwrap();

        let err = repo.create(&sample());
        assert!(err.is_err());
    }

    /// Critério de aceite da Fase 3: excluir projeto não pode deixar o histórico órfão — o
    /// schema usa `ON DELETE SET NULL` em `prompt_history.project_id` (DATABASE-SCHEMA.md §3),
    /// então a linha do histórico deve continuar existindo, só com `project_id = NULL`.
    #[test]
    fn deleting_project_sets_history_project_id_to_null_instead_of_orphaning_rows() {
        use crate::domain::{NewFlow, PromptMode, PromptSource};
        use crate::storage::HistoryRepo;

        let db = test_pool();
        let repo = ProjectRepo::new(db.pool.clone());
        let history = HistoryRepo::new(db.pool.clone());

        let project = repo.create(&sample()).unwrap();

        let flow = NewFlow {
            project_id: Some(project.id),
            duration_ms: 5_000,
            device_name: "Microfone padrão".into(),
            transcript_text: "criar tela de projetos".into(),
            transcript_language: "pt".into(),
            engine: "whisper-rs".into(),
            model_name: "large-v3-turbo".into(),
            transcribe_duration_ms: 200,
            mode: PromptMode::NewFeature,
            generator: PromptSource::Template,
            prompt_content: "Objetivo: criar tela de projetos.".into(),
        };
        let history_id = history.save_flow(&flow).unwrap();

        repo.delete(project.id).unwrap();

        let conn = db.pool.get().unwrap();
        let remaining_project_id: Option<i32> = conn
            .query_row(
                "SELECT project_id FROM prompt_history WHERE id = ?1",
                params![history_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(remaining_project_id, None);

        let row_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM prompt_history WHERE id = ?1",
                params![history_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            row_count, 1,
            "a linha do histórico não deveria ter sido removida"
        );
    }
}
