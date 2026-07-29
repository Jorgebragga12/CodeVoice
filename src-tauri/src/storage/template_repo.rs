use rusqlite::{params, OptionalExtension, Row};

use crate::domain::{NewPromptTemplate, PromptTemplate};

use super::{DbPool, StorageError};

#[derive(Clone)]
pub struct PromptTemplateRepo {
    pool: DbPool,
}

const SELECT_COLUMNS: &str = "id, name, mode, category, description, content, source, slug, \
     project_id, created_at, updated_at";

fn row_to_template(row: &Row<'_>) -> rusqlite::Result<PromptTemplate> {
    Ok(PromptTemplate {
        id: row.get("id")?,
        name: row.get("name")?,
        mode: row.get("mode")?,
        category: row.get("category")?,
        description: row.get("description")?,
        content: row.get("content")?,
        source: row.get("source")?,
        slug: row.get("slug")?,
        project_id: row.get("project_id")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

/// Linha da biblioteca embutida, já parseada a partir do arquivo markdown.
pub struct BuiltinTemplate {
    pub slug: String,
    pub name: String,
    pub mode: String,
    pub category: String,
    pub description: String,
    pub content: String,
}

impl PromptTemplateRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Substitui **toda** a biblioteca embutida numa transação.
    ///
    /// Apagar e reinserir mantém os modelos do binário como a fonte da verdade a cada versão do
    /// app (incluindo remoções e renomeações), sem precisar de versionamento próprio. O `WHERE
    /// source = 'builtin'` é o que garante que nada do usuário seja tocado.
    pub fn replace_builtins(&self, items: &[BuiltinTemplate]) -> Result<usize, StorageError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        tx.execute("DELETE FROM prompt_templates WHERE source = 'builtin'", [])?;

        {
            let mut stmt = tx.prepare(
                "INSERT INTO prompt_templates \
                 (name, mode, category, description, content, source, slug) \
                 VALUES (?1, ?2, ?3, ?4, ?5, 'builtin', ?6)",
            )?;
            for item in items {
                stmt.execute(params![
                    item.name,
                    item.mode,
                    item.category,
                    item.description,
                    item.content,
                    item.slug,
                ])?;
            }
        }

        tx.commit()?;
        Ok(items.len())
    }

    /// Modelos do usuário primeiro (são poucos e é o que ele acabou de criar), depois a
    /// biblioteca, agrupada por categoria.
    pub fn list(&self) -> Result<Vec<PromptTemplate>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM prompt_templates \
             ORDER BY source ASC, category ASC, name ASC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], row_to_template)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    pub fn list_by_category(&self, category: &str) -> Result<Vec<PromptTemplate>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM prompt_templates WHERE category = ?1 ORDER BY name ASC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![category], row_to_template)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    /// Categorias existentes com a contagem de modelos, para montar a navegação da biblioteca.
    pub fn categories(&self) -> Result<Vec<(String, i32)>, StorageError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT category, COUNT(*) FROM prompt_templates GROUP BY category ORDER BY category",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
    }

    pub fn get(&self, id: i32) -> Result<Option<PromptTemplate>, StorageError> {
        let conn = self.pool.get()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM prompt_templates WHERE id = ?1");
        conn.query_row(&sql, params![id], row_to_template)
            .optional()
            .map_err(Into::into)
    }

    pub fn create(&self, input: &NewPromptTemplate) -> Result<PromptTemplate, StorageError> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO prompt_templates \
             (name, mode, category, description, content, source, slug, project_id) \
             VALUES (?1, ?2, ?3, ?4, ?5, 'user', NULL, ?6)",
            params![
                input.name,
                input.mode,
                input.category,
                input.description,
                input.content,
                input.project_id,
            ],
        )?;
        let id = conn.last_insert_rowid() as i32;
        drop(conn);
        self.get(id)?
            .ok_or_else(|| StorageError::NotFound(format!("prompt_template {id}")))
    }

    /// Exclui um modelo do usuário. Modelos embutidos são intocáveis: apagar um só teria efeito
    /// até o próximo startup (o seed o traria de volta), então recusar é mais honesto que
    /// aparentar sucesso.
    pub fn delete_user_template(&self, id: i32) -> Result<(), StorageError> {
        let conn = self.pool.get()?;
        let changed = conn.execute(
            "DELETE FROM prompt_templates WHERE id = ?1 AND source = 'user'",
            params![id],
        )?;
        if changed == 0 {
            return Err(StorageError::NotFound(format!(
                "modelo {id} não existe ou é da biblioteca embutida (não pode ser excluído)"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::test_pool;

    fn builtin(slug: &str, category: &str, name: &str) -> BuiltinTemplate {
        BuiltinTemplate {
            slug: slug.into(),
            name: name.into(),
            mode: "bug_fix".into(),
            category: category.into(),
            description: "quando quebrou agora".into(),
            content: "Corrija o erro em <<SUA FALA>>".into(),
        }
    }

    fn user_template() -> NewPromptTemplate {
        NewPromptTemplate {
            name: "Meu modelo".into(),
            mode: "technical".into(),
            category: "meus".into(),
            description: "uso pessoal".into(),
            content: "# Prompt\n<<SUA FALA>>".into(),
            project_id: None,
        }
    }

    #[test]
    fn seeds_the_builtin_library() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());

        let count = repo
            .replace_builtins(&[
                builtin("depuracao/erro-agora", "depuracao", "Erro agora"),
                builtin("depuracao/ta-lento", "depuracao", "Tá lento"),
            ])
            .unwrap();

        assert_eq!(count, 2);
        let listed = repo.list().unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].source, "builtin");
        assert_eq!(listed[0].slug.as_deref(), Some("depuracao/erro-agora"));
    }

    #[test]
    fn reseeding_replaces_instead_of_duplicating() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        let items = [builtin("depuracao/erro-agora", "depuracao", "Erro agora")];

        repo.replace_builtins(&items).unwrap();
        repo.replace_builtins(&items).unwrap();

        assert_eq!(repo.list().unwrap().len(), 1, "o slug único impediria o duplicado");
    }

    #[test]
    fn reseeding_drops_templates_removed_from_the_binary() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());

        repo.replace_builtins(&[
            builtin("depuracao/erro-agora", "depuracao", "Erro agora"),
            builtin("depuracao/aposentado", "depuracao", "Aposentado"),
        ])
        .unwrap();
        repo.replace_builtins(&[builtin("depuracao/erro-agora", "depuracao", "Erro agora")])
            .unwrap();

        let slugs: Vec<_> = repo.list().unwrap().into_iter().filter_map(|t| t.slug).collect();
        assert_eq!(slugs, vec!["depuracao/erro-agora"]);
    }

    /// O seed roda em todo startup; se ele apagasse os modelos do usuário, o "salvar como
    /// modelo" seria inútil na prática.
    #[test]
    fn reseeding_never_touches_user_templates() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        let mine = repo.create(&user_template()).unwrap();

        repo.replace_builtins(&[builtin("depuracao/erro-agora", "depuracao", "Erro agora")])
            .unwrap();

        let still_there = repo.get(mine.id).unwrap().expect("modelo do usuário sumiu");
        assert_eq!(still_there.content, mine.content);
    }

    #[test]
    fn user_templates_are_never_marked_as_builtin() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());

        let created = repo.create(&user_template()).unwrap();

        assert_eq!(created.source, "user");
        assert!(created.slug.is_none());
    }

    #[test]
    fn lists_by_category_and_counts_them() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        repo.replace_builtins(&[
            builtin("depuracao/a", "depuracao", "A"),
            builtin("depuracao/b", "depuracao", "B"),
            builtin("testes-avancados/c", "testes-avancados", "C"),
        ])
        .unwrap();

        assert_eq!(repo.list_by_category("depuracao").unwrap().len(), 2);
        let categories = repo.categories().unwrap();
        assert_eq!(
            categories,
            vec![("depuracao".to_string(), 2), ("testes-avancados".to_string(), 1)]
        );
    }

    #[test]
    fn deletes_user_template() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        let created = repo.create(&user_template()).unwrap();

        repo.delete_user_template(created.id).unwrap();

        assert!(repo.get(created.id).unwrap().is_none());
    }

    #[test]
    fn refuses_to_delete_a_builtin_template() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        repo.replace_builtins(&[builtin("depuracao/erro-agora", "depuracao", "Erro agora")])
            .unwrap();
        let builtin_id = repo.list().unwrap()[0].id;

        assert!(matches!(
            repo.delete_user_template(builtin_id),
            Err(StorageError::NotFound(_))
        ));
        assert!(repo.get(builtin_id).unwrap().is_some(), "o embutido tem que continuar lá");
    }

    /// Fecha o circuito parser → migration 003 → repositório com a biblioteca real, não com
    /// dados de teste: é o que prova que os 117 modelos versionados cabem no esquema (o `CHECK`
    /// de `mode` incluído) e chegam ao banco com o cabeçalho já removido.
    #[test]
    fn seeds_the_real_library_of_117_templates() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());

        let count = repo
            .replace_builtins(&crate::promptgen::library::builtins())
            .expect("a biblioteca real precisa passar pelos CHECKs do esquema");
        assert_eq!(count, 117);

        assert_eq!(repo.categories().unwrap().len(), 18);

        let stored = repo.list().unwrap();
        for template in &stored {
            assert!(!template.content.contains("> Modo:"), "{} com metadado no corpo", template.name);
            assert!(!template.description.is_empty(), "{} sem descrição", template.name);
        }

        let sample = stored
            .iter()
            .find(|t| t.slug.as_deref() == Some("depuracao/erro-agora"))
            .expect("modelo de referência sumiu da biblioteca");
        assert_eq!(sample.mode, "bug_fix");
        assert_eq!(sample.category, "depuracao");
        assert!(sample.content.contains("<<SUA FALA>>"));
    }

    #[test]
    fn rejects_mode_outside_the_schema_check() {
        let db = test_pool();
        let repo = PromptTemplateRepo::new(db.pool.clone());
        let mut bad = user_template();
        bad.mode = "modo_inventado".into();

        assert!(repo.create(&bad).is_err());
    }
}
