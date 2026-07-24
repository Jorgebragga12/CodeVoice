use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::audio::{list_input_devices, AudioDevice, Recorder, RecorderState};
use crate::domain::{NewRecording, Recording};
use crate::settings::{RecordingSettings, SettingsRepo};
use crate::storage::RecordingRepo;

/// Estado gerenciado da gravação. O `Mutex` serializa start/stop/cancel: sem ele, dois cliques
/// rápidos (ou o atalho global disparando junto com o botão) poderiam abrir dois streams no
/// mesmo device.
pub struct RecorderHandle {
    pub inner: Mutex<Recorder>,
    pub tmp_dir: PathBuf,
    /// Projeto selecionado na UI. Fica no backend porque o atalho global precisa saber a que
    /// projeto associar a gravação **sem** depender de uma ida e volta ao frontend — que pode
    /// estar minimizado na bandeja quando o atalho é acionado.
    pub active_project: Mutex<Option<i32>>,
}

impl RecorderHandle {
    pub fn new(tmp_dir: PathBuf) -> Self {
        Self {
            inner: Mutex::new(Recorder::new()),
            tmp_dir,
            active_project: Mutex::new(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct RecordingStatus {
    pub state: RecorderState,
    pub elapsed_ms: i32,
}

/// Um `Mutex` envenenado significa que alguma thread deu panic segurando o lock — o estado do
/// recorder é indeterminado. Reportamos como erro em vez de propagar o panic e derrubar o app.
fn lock_error() -> String {
    "estado da gravação ficou inconsistente; reinicie o CodeVoice".to_string()
}

// ---------------------------------------------------------------------------------------------
// Lógica compartilhada entre os commands (chamados pela UI) e o atalho global (chamado pelo SO).
// Manter isso aqui, e não duplicado nos dois caminhos, é o que garante que gravar pelo botão e
// gravar pelo atalho façam exatamente a mesma coisa.
// ---------------------------------------------------------------------------------------------

pub fn do_start(app: &AppHandle) -> Result<(), String> {
    let handle = app.state::<RecorderHandle>();
    let microphone = app
        .state::<SettingsRepo>()
        .get_recording_settings()
        .map_err(|e| e.to_string())?
        .microphone;

    {
        let mut recorder = handle.inner.lock().map_err(|_| lock_error())?;
        recorder
            .start(microphone, &handle.tmp_dir)
            .map_err(|e| e.to_string())?;
    }

    let _ = app.emit("recording:started", ());
    Ok(())
}

/// Encerra a gravação e persiste os metadados. O WAV **continua no disco** — quem o apaga é a
/// transcrição (Fase 5), depois de consumi-lo.
pub fn do_stop(app: &AppHandle) -> Result<Recording, String> {
    let handle = app.state::<RecorderHandle>();

    let finished = {
        let mut recorder = handle.inner.lock().map_err(|_| lock_error())?;
        recorder.stop().map_err(|e| e.to_string())?
    };
    let project_id = *handle.active_project.lock().map_err(|_| lock_error())?;

    let recording = app
        .state::<RecordingRepo>()
        .create(&NewRecording {
            project_id,
            duration_ms: finished.duration_ms,
            device_name: finished.device_name,
            audio_path: Some(finished.path.display().to_string()),
            audio_kept: false,
            status: "recorded".into(),
        })
        .map_err(|e| e.to_string())?;

    let _ = app.emit("recording:stopped", recording.clone());
    Ok(recording)
}

/// Cancela e apaga o WAV.
///
/// Não grava nada em `recordings` de propósito: cancelar significa "isso não aconteceu", e uma
/// linha registrando que o usuário falou 5 s e desistiu seria metadado sobre o comportamento
/// dele sem nenhum valor de uso — contra o espírito do PRODUCT-SPEC §6.
pub fn do_cancel(app: &AppHandle) -> Result<(), String> {
    let handle = app.state::<RecorderHandle>();
    {
        let mut recorder = handle.inner.lock().map_err(|_| lock_error())?;
        recorder.cancel().map_err(|e| e.to_string())?;
    }
    let _ = app.emit("recording:cancelled", ());
    Ok(())
}

/// Alterna gravando/parado. É o que o atalho global dispara.
pub fn do_toggle(app: &AppHandle) -> Result<(), String> {
    let is_recording = {
        let handle = app.state::<RecorderHandle>();
        let recorder = handle.inner.lock().map_err(|_| lock_error())?;
        recorder.is_recording()
    };

    if is_recording {
        do_stop(app).map(|_| ())
    } else {
        do_start(app)
    }
}

// ---------------------------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    list_input_devices().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn get_recording_settings(repo: State<'_, SettingsRepo>) -> Result<RecordingSettings, String> {
    repo.get_recording_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn save_recording_settings(
    repo: State<'_, SettingsRepo>,
    settings: RecordingSettings,
) -> Result<(), String> {
    repo.save_recording_settings(&settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn recording_status(handle: State<'_, RecorderHandle>) -> Result<RecordingStatus, String> {
    let recorder = handle.inner.lock().map_err(|_| lock_error())?;
    Ok(RecordingStatus {
        state: recorder.state(),
        elapsed_ms: recorder.elapsed_ms(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn set_active_project(
    handle: State<'_, RecorderHandle>,
    project_id: Option<i32>,
) -> Result<(), String> {
    let mut active = handle.active_project.lock().map_err(|_| lock_error())?;
    *active = project_id;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn start_recording(app: AppHandle) -> Result<(), String> {
    do_start(&app)
}

#[tauri::command]
#[specta::specta]
pub fn stop_recording(app: AppHandle) -> Result<Recording, String> {
    do_stop(&app)
}

#[tauri::command]
#[specta::specta]
pub fn cancel_recording(app: AppHandle) -> Result<(), String> {
    do_cancel(&app)
}
