use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

/// Teto de segurança por gravação (PRODUCT-SPEC §5.2). Atingi-lo encerra a gravação como se o
/// usuário tivesse parado — o áudio até ali é preservado, não descartado.
pub const MAX_RECORDING: Duration = Duration::from_secs(10 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum RecorderState {
    Idle,
    Recording,
    Stopped,
    Cancelled,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("já existe uma gravação em andamento")]
    AlreadyRecording,
    #[error("nenhuma gravação em andamento")]
    NotRecording,
}

/// Ciclo de vida de uma gravação: `Idle → Recording → (Stopped | Cancelled)`.
///
/// Deliberadamente **sem nenhuma dependência de áudio ou de I/O** — só transições e tempo. É o
/// que permite testar as regras do ciclo (e o limite de 10 min) sem microfone, já que a captura
/// real do cpal não é exercitável em ambiente sem hardware/CI.
#[derive(Debug)]
pub struct RecordingSession {
    state: RecorderState,
    started_at: Option<Instant>,
    /// Congelado no `stop`/`cancel` para que `elapsed()` continue devolvendo a duração da
    /// gravação depois de encerrada, em vez de um cronômetro que segue correndo.
    finished_elapsed: Option<Duration>,
}

impl Default for RecordingSession {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordingSession {
    pub fn new() -> Self {
        Self {
            state: RecorderState::Idle,
            started_at: None,
            finished_elapsed: None,
        }
    }

    pub fn state(&self) -> RecorderState {
        self.state
    }

    pub fn is_recording(&self) -> bool {
        self.state == RecorderState::Recording
    }

    pub fn start(&mut self) -> Result<(), SessionError> {
        if self.state == RecorderState::Recording {
            return Err(SessionError::AlreadyRecording);
        }
        self.state = RecorderState::Recording;
        self.started_at = Some(Instant::now());
        self.finished_elapsed = None;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<Duration, SessionError> {
        self.finish(RecorderState::Stopped)
    }

    pub fn cancel(&mut self) -> Result<Duration, SessionError> {
        self.finish(RecorderState::Cancelled)
    }

    fn finish(&mut self, next: RecorderState) -> Result<Duration, SessionError> {
        if self.state != RecorderState::Recording {
            return Err(SessionError::NotRecording);
        }
        let elapsed = self.elapsed();
        self.state = next;
        self.finished_elapsed = Some(elapsed);
        self.started_at = None;
        Ok(elapsed)
    }

    /// Tempo decorrido: cronômetro vivo enquanto grava, valor congelado depois de encerrada.
    pub fn elapsed(&self) -> Duration {
        if let Some(started) = self.started_at {
            started.elapsed()
        } else {
            self.finished_elapsed.unwrap_or_default()
        }
    }

    /// `true` quando a gravação passou do teto de segurança e deve ser encerrada pelo chamador.
    pub fn reached_limit(&self) -> bool {
        self.is_recording() && self.elapsed() >= MAX_RECORDING
    }

    /// Volta pro estado inicial, permitindo uma nova gravação após stop/cancel.
    pub fn reset(&mut self) {
        self.state = RecorderState::Idle;
        self.started_at = None;
        self.finished_elapsed = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_idle() {
        let session = RecordingSession::new();
        assert_eq!(session.state(), RecorderState::Idle);
        assert!(!session.is_recording());
    }

    #[test]
    fn idle_to_recording_to_stopped() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        assert_eq!(session.state(), RecorderState::Recording);
        session.stop().unwrap();
        assert_eq!(session.state(), RecorderState::Stopped);
    }

    #[test]
    fn idle_to_recording_to_cancelled() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        session.cancel().unwrap();
        assert_eq!(session.state(), RecorderState::Cancelled);
    }

    #[test]
    fn cannot_start_twice() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        assert_eq!(session.start(), Err(SessionError::AlreadyRecording));
        // O erro não pode ter derrubado a gravação em andamento.
        assert_eq!(session.state(), RecorderState::Recording);
    }

    #[test]
    fn cannot_stop_or_cancel_when_idle() {
        let mut session = RecordingSession::new();
        assert_eq!(session.stop(), Err(SessionError::NotRecording));
        assert_eq!(session.cancel(), Err(SessionError::NotRecording));
    }

    #[test]
    fn cannot_stop_twice() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        session.stop().unwrap();
        assert_eq!(session.stop(), Err(SessionError::NotRecording));
        assert_eq!(session.state(), RecorderState::Stopped);
    }

    #[test]
    fn reset_allows_a_new_recording_after_stop() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        session.stop().unwrap();
        session.reset();
        assert_eq!(session.state(), RecorderState::Idle);
        session.start().unwrap();
        assert_eq!(session.state(), RecorderState::Recording);
    }

    #[test]
    fn elapsed_freezes_after_stop() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        std::thread::sleep(Duration::from_millis(20));
        let at_stop = session.stop().unwrap();
        std::thread::sleep(Duration::from_millis(20));
        // Congelado: não pode continuar correndo depois de encerrada.
        assert_eq!(session.elapsed(), at_stop);
        assert!(at_stop >= Duration::from_millis(20));
    }

    #[test]
    fn elapsed_is_zero_before_starting() {
        let session = RecordingSession::new();
        assert_eq!(session.elapsed(), Duration::ZERO);
    }

    #[test]
    fn limit_is_not_reached_right_after_starting() {
        let mut session = RecordingSession::new();
        session.start().unwrap();
        assert!(!session.reached_limit());
    }

    #[test]
    fn limit_never_triggers_when_not_recording() {
        let session = RecordingSession::new();
        assert!(!session.reached_limit());
    }
}
