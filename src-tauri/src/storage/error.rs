use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("erro de banco de dados: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("erro no pool de conexões: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("erro de I/O em {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("registro não encontrado: {0}")]
    NotFound(String),
}
