use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{NewProjectRule, ProjectRule, ProjectRuleUpdate};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct ProjectRuleRepo {
    pool: DbPool,
}

fn row_to_rule(row: &Row<'_>) -> rusqlite::Result<ProjectRule> {
    Ok(ProjectRule {
        id: row.get("id")?,
        project_id: row.get("project_id")?,
        rule: row.get("rule")?,
        sort_order: row.get("sort_order")?,
        created_at: row.get("created_at")?,
    })
}

const SELECT_COLUMNS: &str = "id, project_id, rule, sort_order, created_at";

impl ProjectRuleRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Cria uma regra ao final da lista do projeto (maior `sort_order` existente + 1).
    pub fn create(&self, input: &NewProjectRule) -> Result<ProjectRule, StorageError> {
        let conn = self.pool.get()?;
        let next_order: i32 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM project_rules WHERE project_id = ?1",
            params![input.project_id],
            |row| row.get(0),
        )?;

        conn.execute(
            "INSERT INTO project_rules (project_id, rule, sort_order) VALUES (?1, ?2, ?3)",
            params![input.project_id, input.rule, next_order],
        )?;
        let id = conn.last_insert_rowid() as i32;
        drop(conn);
        self.get(id)?.ok_or_else(|| StorageError::NotFound(format!("project_rule {id}")))
    }

    pub fn get(&self, id: i32) -> Result<Option<ProjectRule>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM project_rules WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_rule).optional().map_err(Into::into)
    }

    pub fn list_for_project(&self, project_id: i32) -> Result<Vec<ProjectRule>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM project_rules WHERE project_id = ?1 \
             ORDER BY sort_order ASC, id ASC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![project_id], row_to_rule)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    /// Atualiza o texto da regra. `sort_order` não é editável por aqui — usar [`Self::reorder`].
    pub fn update(&self, id: i32, input: &ProjectRuleUpdate) -> Result<ProjectRule, StorageError> {
        let conn = self.pool.get()?;
        let changed =
            conn.execute("UPDATE project_rules SET rule = ?1 WHERE id = ?2", params![input.rule, id])?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("project_rule {id}")));
        }
        drop(conn);
        self.get(id)?.ok_or_else(|| StorageError::NotFound(format!("project_rule {id}")))
    }

    pub fn delete(&self, id: i32) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute("DELETE FROM project_rules WHERE id = ?1", params![id])?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!("project_rule {id}")));
        }
        Ok(())
    }

    /// Reordena as regras de um projeto: `ordered_ids[i]` recebe `sort_order = i`. Ids que não
    /// pertencem a `project_id` são ignorados (a cláusula `AND project_id = ?3` simplesmente
    /// não casa nenhuma linha para eles).
    pub fn reorder(&self, project_id: i32, ordered_ids: &[i32]) -> Result<(), StorageError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        for (index, id) in ordered_ids.iter().enumerate() {
            tx.execute(
                "UPDATE project_rules SET sort_order = ?1 WHERE id = ?2 AND project_id = ?3",
                params![index as i32, id, project_id],
            )?;
        }
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::NewProject;
    use crate::storage::{test_pool, ProjectRepo};

    fn setup_project(repo: &ProjectRepo) -> i32 {
        repo.create(&NewProject {
            name: "CodeVoice".into(),
            path: "C:\\projects\\codevoice".into(),
            description: String::new(),
            stack: String::new(),
            architecture: String::new(),
            dev_commands: String::new(),
            test_commands: String::new(),
            forbidden_tech: String::new(),
            database_info: String::new(),
            notes: String::new(),
        })
        .unwrap()
        .id
    }

    #[test]
    fn creates_rules_appending_sort_order() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);

        let first =
            rules.create(&NewProjectRule { project_id, rule: "Nunca usar i64 em IDs".into() }).unwrap();
        let second =
            rules.create(&NewProjectRule { project_id, rule: "Sempre validar path".into() }).unwrap();

        assert_eq!(first.sort_order, 0);
        assert_eq!(second.sort_order, 1);
    }

    #[test]
    fn lists_rules_ordered() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);

        rules.create(&NewProjectRule { project_id, rule: "regra A".into() }).unwrap();
        rules.create(&NewProjectRule { project_id, rule: "regra B".into() }).unwrap();

        let listed = rules.list_for_project(project_id).unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].rule, "regra A");
        assert_eq!(listed[1].rule, "regra B");
    }

    #[test]
    fn updates_rule_text() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);

        let created = rules.create(&NewProjectRule { project_id, rule: "original".into() }).unwrap();
        let updated =
            rules.update(created.id, &ProjectRuleUpdate { rule: "editada".into() }).unwrap();

        assert_eq!(updated.rule, "editada");
    }

    #[test]
    fn update_missing_rule_returns_not_found() {
        let db = test_pool();
        let rules = ProjectRuleRepo::new(db.pool.clone());

        let err = rules.update(9999, &ProjectRuleUpdate { rule: "x".into() });
        assert!(matches!(err, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn deletes_rule() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);

        let created = rules.create(&NewProjectRule { project_id, rule: "efêmera".into() }).unwrap();
        rules.delete(created.id).unwrap();

        assert!(rules.get(created.id).unwrap().is_none());
    }

    #[test]
    fn reorders_rules() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);

        let a = rules.create(&NewProjectRule { project_id, rule: "A".into() }).unwrap();
        let b = rules.create(&NewProjectRule { project_id, rule: "B".into() }).unwrap();
        let c = rules.create(&NewProjectRule { project_id, rule: "C".into() }).unwrap();

        rules.reorder(project_id, &[c.id, a.id, b.id]).unwrap();

        let listed = rules.list_for_project(project_id).unwrap();
        let names: Vec<_> = listed.iter().map(|r| r.rule.as_str()).collect();
        assert_eq!(names, vec!["C", "A", "B"]);
    }

    #[test]
    fn deleting_project_cascades_rules() {
        let db = test_pool();
        let projects = ProjectRepo::new(db.pool.clone());
        let rules = ProjectRuleRepo::new(db.pool.clone());
        let project_id = setup_project(&projects);
        rules.create(&NewProjectRule { project_id, rule: "alguma regra".into() }).unwrap();

        projects.delete(project_id).unwrap();

        assert!(rules.list_for_project(project_id).unwrap().is_empty());
    }
}
