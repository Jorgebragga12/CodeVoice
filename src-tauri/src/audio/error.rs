use thiserror::Error;

use super::session::SessionError;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("{0}")]
    Session(#[from] SessionError),

    #[error("nenhum microfone disponível")]
    NoInputDevice,

    #[error("microfone \"{0}\" não encontrado")]
    DeviceNotFound(String),

    #[error("não foi possível ler a configuração do microfone: {0}")]
    UnsupportedConfig(String),

    #[error("formato de áudio não suportado: {0}")]
    UnsupportedSampleFormat(String),

    #[error("falha ao acessar o microfone: {0}")]
    Stream(String),

    #[error("falha ao gravar o arquivo de áudio: {0}")]
    Wav(String),

    #[error("erro de I/O em {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
