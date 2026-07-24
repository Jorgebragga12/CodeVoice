mod capture;
pub mod devices;
mod error;
mod resample;
mod session;
pub mod tmp_files;

pub use devices::{list_input_devices, AudioDevice};
pub use error::AudioError;
pub use resample::TARGET_SAMPLE_RATE;
pub use session::{RecorderState, RecordingSession, MAX_RECORDING};

use std::path::{Path, PathBuf};

use capture::{start_capture, CaptureCommand, CaptureHandle};

/// Resultado de uma gravação encerrada com sucesso.
#[derive(Debug, Clone)]
pub struct FinishedRecording {
    pub path: PathBuf,
    pub duration_ms: i32,
    pub device_name: String,
}

/// Orquestra o ciclo de gravação: junta a máquina de estados (`RecordingSession`) com a thread
/// de captura (`CaptureHandle`), garantindo que os dois nunca fiquem fora de sincronia.
///
/// Fica no estado gerenciado do Tauri atrás de um `Mutex`.
#[derive(Default)]
pub struct Recorder {
    session: RecordingSession,
    handle: Option<CaptureHandle>,
    current_path: Option<PathBuf>,
    device_name: String,
}

impl Recorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> RecorderState {
        self.session.state()
    }

    pub fn is_recording(&self) -> bool {
        self.session.is_recording()
    }

    pub fn elapsed_ms(&self) -> i32 {
        clamp_ms(self.session.elapsed().as_millis())
    }

    /// `true` quando a gravação passou dos 10 min e o chamador deve encerrá-la.
    pub fn reached_limit(&self) -> bool {
        self.session.reached_limit()
    }

    pub fn current_path(&self) -> Option<&Path> {
        self.current_path.as_deref()
    }

    pub fn start(&mut self, device_name: Option<String>, tmp_dir: &Path) -> Result<(), AudioError> {
        // Valida a transição ANTES de tocar no microfone: se já existe gravação em andamento,
        // abrir um segundo stream deixaria dois writers disputando o mesmo estado.
        if self.session.is_recording() {
            return Err(AudioError::Session(session::SessionError::AlreadyRecording));
        }

        let output = tmp_files::new_recording_path(tmp_dir);
        let handle = start_capture(device_name.clone(), output.clone())?;

        self.session.reset();
        self.session.start()?;
        self.handle = Some(handle);
        self.current_path = Some(output);
        self.device_name = device_name.unwrap_or_else(|| "(padrão do sistema)".to_string());

        Ok(())
    }

    /// Encerra a gravação preservando o arquivo.
    pub fn stop(&mut self) -> Result<FinishedRecording, AudioError> {
        let duration = self.session.stop()?;
        let path = self
            .current_path
            .take()
            .ok_or_else(|| AudioError::Stream("gravação sem arquivo associado".into()))?;

        if let Some(handle) = self.handle.take() {
            handle.finish(CaptureCommand::Stop)?;
        }

        Ok(FinishedRecording {
            path,
            duration_ms: clamp_ms(duration.as_millis()),
            device_name: std::mem::take(&mut self.device_name),
        })
    }

    /// Cancela a gravação e **apaga o arquivo** — o áudio nunca fica no disco (PRODUCT-SPEC §5.2).
    pub fn cancel(&mut self) -> Result<(), AudioError> {
        self.session.cancel()?;
        let path = self.current_path.take();

        if let Some(handle) = self.handle.take() {
            // Um erro ao encerrar o stream não pode impedir a exclusão do arquivo: o ponto do
            // cancelamento é justamente não deixar áudio para trás.
            let _ = handle.finish(CaptureCommand::Cancel);
        }

        if let Some(path) = path {
            let _ = std::fs::remove_file(path);
        }
        self.device_name.clear();

        Ok(())
    }
}

/// Converte millis (`u128`) para o `i32` usado no domínio/IPC. O teto de 10 min cabe folgado em
/// `i32`, então o saturating só existe para não haver caminho de overflow silencioso.
fn clamp_ms(millis: u128) -> i32 {
    i32::try_from(millis).unwrap_or(i32::MAX)
}

/// Ponto de entrada só para o exemplo `smoke_capture` (teste manual de hardware). Exposto aqui,
/// e não como parte da API normal, porque abrir o device diretamente sem passar pela máquina de
/// estados só faz sentido nesse contexto de diagnóstico.
#[doc(hidden)]
pub fn __smoke_start_capture(output: PathBuf) -> Result<SmokeHandle, AudioError> {
    Ok(SmokeHandle(start_capture(None, output)?))
}

#[doc(hidden)]
pub struct SmokeHandle(CaptureHandle);

impl SmokeHandle {
    pub fn stop(self) -> Result<u32, AudioError> {
        self.0.finish(CaptureCommand::Stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_recorder_is_idle_and_has_no_file() {
        let recorder = Recorder::new();
        assert_eq!(recorder.state(), RecorderState::Idle);
        assert!(!recorder.is_recording());
        assert!(recorder.current_path().is_none());
        assert_eq!(recorder.elapsed_ms(), 0);
    }

    #[test]
    fn stopping_an_idle_recorder_is_an_error_not_a_panic() {
        let mut recorder = Recorder::new();
        assert!(recorder.stop().is_err());
    }

    #[test]
    fn cancelling_an_idle_recorder_is_an_error_not_a_panic() {
        let mut recorder = Recorder::new();
        assert!(recorder.cancel().is_err());
    }

    #[test]
    fn clamp_ms_saturates_instead_of_overflowing() {
        assert_eq!(clamp_ms(1_500), 1_500);
        assert_eq!(clamp_ms(u128::MAX), i32::MAX);
    }
}
