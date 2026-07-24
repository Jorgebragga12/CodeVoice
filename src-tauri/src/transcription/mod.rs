pub mod model_manager;
mod normalize;
mod whisper;

pub use normalize::normalize_peak;
pub use whisper::WhisperEngine;

use std::path::Path;

use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

/// Idioma alvo da transcrição. PT é o principal (PRODUCT-SPEC §5.3); mantido como campo para
/// não travar um futuro multi-idioma.
pub const DEFAULT_LANGUAGE: &str = "pt";

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TranscribeOptions {
    /// Código ISO do idioma (ex.: "pt").
    pub language: String,
    /// Prompt inicial dado ao Whisper para enviesar a grafia de termos técnicos — nomes de
    /// arquivos, comandos, tecnologias do projeto ativo (PRODUCT-SPEC §5.3). Sem isso o modelo
    /// tende a "aportuguesar" ou transcrever foneticamente "useEffect", "package.json" etc.
    pub initial_prompt: String,
}

impl Default for TranscribeOptions {
    fn default() -> Self {
        Self {
            language: DEFAULT_LANGUAGE.to_string(),
            initial_prompt: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Transcript {
    pub text: String,
    pub language: String,
    /// Quanto tempo o processamento levou (não a duração do áudio). Alimenta o benchmark e a
    /// coluna `transcriptions.duration_ms`.
    pub processing_ms: i32,
}

/// Estado do motor quanto ao modelo. A UI usa isso para decidir se precisa baixar antes de
/// transcrever (ADR-001).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum EngineStatus {
    ModelMissing,
    Downloading { percent: u8 },
    Ready,
}

#[derive(Debug, Error)]
pub enum TranscribeError {
    #[error("nenhuma fala detectada no áudio")]
    NoSpeech,

    #[error("o modelo de transcrição ainda não foi baixado")]
    ModelMissing,

    #[error("áudio inválido ou ilegível: {0}")]
    InvalidAudio(String),

    #[error("falha ao carregar o modelo: {0}")]
    ModelLoad(String),

    #[error("falha ao transcrever: {0}")]
    Processing(String),

    #[error("erro de I/O em {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// Recebe atualizações de progresso (0–100) durante uma transcrição.
///
/// É `Arc<... + 'static>` (não um `Box` com lifetime) por uma exigência concreta do whisper-rs:
/// `set_progress_callback_safe` requer `FnMut + 'static`. O callback precisa poder ser movido
/// para dentro do estado do whisper.cpp, que não tem relação de lifetime com o chamador. A
/// camada de commands satisfaz isso naturalmente capturando um `AppHandle` clonado (que é
/// `'static`).
pub type ProgressSink = std::sync::Arc<dyn Fn(u8) + Send + Sync>;

/// Contrato do motor de transcrição. A abstração é o coração do ADR-001: hoje há uma única
/// implementação (whisper.cpp via whisper-rs), mas a assinatura permite trocar por outra engine
/// — ou por uma API remota no futuro — sem tocar em quem consome (commands, UI).
pub trait TranscriptionEngine: Send + Sync {
    /// Transcreve um WAV mono 16 kHz. Deve normalizar volume, detectar silêncio (→ `NoSpeech`)
    /// e reportar progresso via `progress`.
    fn transcribe(
        &self,
        audio: &Path,
        opts: &TranscribeOptions,
        progress: ProgressSink,
    ) -> Result<Transcript, TranscribeError>;

    /// Se o modelo necessário está presente/pronto.
    fn status(&self) -> EngineStatus;
}
