use std::path::{Path, PathBuf};

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use super::model_manager::{self, ModelInfo};
use super::normalize::{normalize_peak, rms};
use super::{
    EngineStatus, ProgressSink, Transcript, TranscribeError, TranscribeOptions, TranscriptionEngine,
};

/// Abaixo deste RMS o áudio é tratado como silêncio e nem chega ao Whisper — que, sobre
/// silêncio, "alucina" frases (tipicamente "Legendas pela comunidade" e afins). Melhor um erro
/// claro de "nenhuma fala detectada" do que texto inventado.
const SILENCE_RMS_THRESHOLD: f32 = 0.0025; // ~-52 dBFS

/// O whisper.cpp/GGML despejam centenas de linhas de debug no stderr a cada transcrição.
/// `install_logging_hooks` redireciona esses logs para os hooks do whisper-rs — e como não
/// habilitamos as features `log_backend`/`tracing_backend`, isso efetivamente os silencia.
/// Chamado uma única vez (via `Once`) antes da primeira transcrição.
fn silence_whisper_logs_once() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(whisper_rs::install_logging_hooks);
}

/// Motor de transcrição baseado em whisper.cpp (via whisper-rs) — a implementação concreta do
/// ADR-001. O modelo é carregado do disco a cada transcrição num primeiro momento; se isso se
/// mostrar lento no benchmark da Fase 5, dá para cachear o `WhisperContext` carregado.
pub struct WhisperEngine {
    app_data_dir: PathBuf,
    model: &'static ModelInfo,
}

impl WhisperEngine {
    pub fn new(app_data_dir: PathBuf, model: &'static ModelInfo) -> Self {
        Self { app_data_dir, model }
    }

    fn model_path(&self) -> PathBuf {
        model_manager::model_path(&self.app_data_dir, self.model)
    }

    /// Lê o WAV (mono 16 kHz produzido pela Fase 4), normaliza o volume e devolve amostras f32
    /// em [-1, 1] prontas para o Whisper. Também devolve o RMS pós-normalização para a checagem
    /// de silêncio.
    fn load_samples(&self, audio: &Path) -> Result<(Vec<f32>, f32), TranscribeError> {
        let mut reader = hound::WavReader::open(audio).map_err(|e| match e {
            hound::Error::IoError(source) => TranscribeError::Io {
                path: audio.display().to_string(),
                source,
            },
            other => TranscribeError::InvalidAudio(other.to_string()),
        })?;

        let spec = reader.spec();
        if spec.channels != 1 || spec.sample_rate != crate::audio::TARGET_SAMPLE_RATE {
            return Err(TranscribeError::InvalidAudio(format!(
                "esperado mono 16 kHz, recebido {} canal(is) a {} Hz",
                spec.channels, spec.sample_rate
            )));
        }

        let mut samples: Vec<i16> = reader
            .samples::<i16>()
            .collect::<Result<_, _>>()
            .map_err(|e| TranscribeError::InvalidAudio(e.to_string()))?;

        if samples.is_empty() {
            return Err(TranscribeError::NoSpeech);
        }

        let gain = normalize_peak(&mut samples);
        log::info!("transcrição: ganho de normalização aplicado = {gain:.1}x");

        let level = rms(&samples);
        let f32_samples = samples.iter().map(|s| *s as f32 / i16::MAX as f32).collect();
        Ok((f32_samples, level))
    }
}

impl TranscriptionEngine for WhisperEngine {
    fn status(&self) -> EngineStatus {
        if model_manager::is_downloaded(&self.app_data_dir, self.model) {
            EngineStatus::Ready
        } else {
            EngineStatus::ModelMissing
        }
    }

    fn transcribe(
        &self,
        audio: &Path,
        opts: &TranscribeOptions,
        progress: ProgressSink,
    ) -> Result<Transcript, TranscribeError> {
        // (ver trait em mod.rs para a assinatura)
        if !model_manager::is_downloaded(&self.app_data_dir, self.model) {
            return Err(TranscribeError::ModelMissing);
        }

        silence_whisper_logs_once();
        let started = std::time::Instant::now();
        let (samples, level) = self.load_samples(audio)?;

        // Corta silêncio ANTES de carregar o modelo: evita gastar segundos de CPU só para o
        // Whisper devolver uma frase alucinada em cima de nada.
        if level < SILENCE_RMS_THRESHOLD {
            return Err(TranscribeError::NoSpeech);
        }

        let model_path = self.model_path();
        let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())
            .map_err(|e| TranscribeError::ModelLoad(e.to_string()))?;

        let mut state = ctx
            .create_state()
            .map_err(|e| TranscribeError::ModelLoad(e.to_string()))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        let threads = std::thread::available_parallelism()
            .map(|n| n.get() as i32)
            .unwrap_or(4)
            .max(1);
        params.set_n_threads(threads);
        params.set_translate(false); // queremos PT transcrito, não traduzido para inglês
        params.set_language(Some(&opts.language));
        params.set_no_context(true); // cada gravação é independente
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        if !opts.initial_prompt.is_empty() {
            params.set_initial_prompt(&opts.initial_prompt);
        }

        let progress_cb = progress.clone();
        params.set_progress_callback_safe(move |p: i32| {
            progress_cb(p.clamp(0, 100) as u8);
        });

        state
            .full(params, &samples)
            .map_err(|e| TranscribeError::Processing(e.to_string()))?;

        let mut text = String::new();
        for i in 0..state.full_n_segments() {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(chunk) = segment.to_str() {
                    text.push_str(chunk);
                }
            }
        }
        let text = text.trim().to_string();

        if text.is_empty() {
            return Err(TranscribeError::NoSpeech);
        }

        progress(100);
        Ok(Transcript {
            text,
            language: opts.language.clone(),
            processing_ms: started.elapsed().as_millis().min(i32::MAX as u128) as i32,
        })
    }
}
