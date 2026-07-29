mod error;
mod history_repo;
mod migrations;
mod project_repo;
mod project_rule_repo;
mod prompt_repo;
mod recording_repo;
mod template_repo;
mod transcription_repo;

pub use error::StorageError;
pub use history_repo::HistoryRepo;
pub use project_repo::ProjectRepo;
pub use project_rule_repo::ProjectRuleRepo;
pub use prompt_repo::PromptRepo;
pub use recording_repo::RecordingRepo;
pub use template_repo::{BuiltinTemplate, PromptTemplateRepo};
pub use transcription_repo::TranscriptionRepo;

use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

/// Cria (se necessário) o diretório de dados, abre o banco em `<app_data_dir>/codevoice.db`,
/// aplica as migrations pendentes e devolve um pool pronto pra uso pelos repositórios.
pub fn init_pool(app_data_dir: &Path) -> Result<DbPool, StorageError> {
    std::fs::create_dir_all(app_data_dir).map_err(|source| StorageError::Io {
        path: app_data_dir.display().to_string(),
        source,
    })?;

    let db_path = app_data_dir.join("codevoice.db");
    let manager = SqliteConnectionManager::file(&db_path).with_init(|conn| {
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")
    });
    let pool = r2d2::Pool::new(manager)?;

    let mut conn = pool.get()?;
    migrations::apply_all(&mut conn)?;

    Ok(pool)
}

/// Banco de teste isolado por arquivo temporário. Não usa `:memory:` porque cada conexão do
/// pool r2d2 abriria seu **próprio** banco em memória (elas não compartilham estado), o que
/// quebraria qualquer teste que pegue mais de uma conexão do pool.
#[cfg(test)]
pub(crate) struct TestDb {
    pub pool: DbPool,
    _file: tempfile::NamedTempFile,
}

#[cfg(test)]
pub(crate) fn test_pool() -> TestDb {
    let file = tempfile::NamedTempFile::new().expect("failed to create temp db file");
    let manager = SqliteConnectionManager::file(file.path())
        .with_init(|conn| conn.execute_batch("PRAGMA foreign_keys = ON;"));
    let pool = r2d2::Pool::new(manager).expect("failed to build pool");
    let mut conn = pool.get().expect("failed to get connection");
    migrations::apply_all(&mut conn).expect("failed to apply migrations");
    drop(conn);
    TestDb { pool, _file: file }
}
