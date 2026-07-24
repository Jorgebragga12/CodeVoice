use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::domain::{NewTranscription, Transcription};
use crate::settings::SettingsRepo;
use crate::storage::{RecordingRepo, TranscriptionRepo};
use crate::transcription::model_manager::{self, ModelInfo};
use crate::transcription::{
    EngineStatus, TranscribeError, TranscribeOptions, TranscriptionEngine, WhisperEngine,
};

/// Diretório de dados do app, guardado no estado para que os commands de transcrição saibam
/// onde ficam os modelos e o WAV temporário sem reconsultar o `AppHandle` toda hora.
pub struct AppDataDir(pub PathBuf);

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct WhisperModelOption {
    pub id: String,
    pub label: String,
    pub downloaded: bool,
    pub size_bytes: f64,
}

fn resolve_model(settings: &SettingsRepo) -> Result<&'static ModelInfo, String> {
    let id = settings
        .get_recording_settings()
        .map_err(|e| e.to_string())?
        .whisper_model;
    Ok(model_manager::find_model(&id).unwrap_or_else(model_manager::default_model))
}

#[tauri::command]
#[specta::specta]
pub fn list_whisper_models(
    app_data: State<'_, AppDataDir>,
    settings: State<'_, SettingsRepo>,
) -> Result<Vec<WhisperModelOption>, String> {
    let selected = settings
        .get_recording_settings()
        .map_err(|e| e.to_string())?
        .whisper_model;

    let _ = selected; // seleção é refletida em settings; aqui só listamos disponibilidade
    Ok(model_manager::MODELS
        .iter()
        .map(|m| WhisperModelOption {
            id: m.id.to_string(),
            label: m.label.to_string(),
            downloaded: model_manager::is_downloaded(&app_data.0, m),
            size_bytes: m.size_bytes as f64,
        })
        .collect())
}

/// Status do motor para o modelo atualmente selecionado: pronto ou modelo ausente.
#[tauri::command]
#[specta::specta]
pub fn transcription_status(
    app_data: State<'_, AppDataDir>,
    settings: State<'_, SettingsRepo>,
) -> Result<EngineStatus, String> {
    let model = resolve_model(&settings)?;
    let engine = WhisperEngine::new(app_data.0.clone(), model);
    Ok(engine.status())
}

/// Baixa o modelo selecionado, emitindo `model:download-progress` (0–100). Roda numa thread
/// dedicada porque o download é longo e não pode travar a UI; o resultado volta por evento
/// (`model:download-done` / `model:download-error`).
#[tauri::command]
#[specta::specta]
pub fn download_model(app: AppHandle) -> Result<(), String> {
    let app_data = app.state::<AppDataDir>().0.clone();
    let model = resolve_model(&app.state::<SettingsRepo>())?;

    std::thread::spawn(move || {
        let app_for_progress = app.clone();
        let on_progress = move |percent: u8| {
            let _ = app_for_progress.emit("model:download-progress", percent);
        };

        match model_manager::download(&app_data, model, &on_progress) {
            Ok(()) => {
                let _ = app.emit("model:download-done", model.id);
            }
            Err(err) => {
                let _ = app.emit("model:download-error", err.to_string());
            }
        }
    });

    Ok(())
}

/// Monta o `initial_prompt` do Whisper com o contexto técnico do projeto ativo — stack,
/// tecnologias — para enviesar a grafia de termos técnicos (PRODUCT-SPEC §5.3). Sem isso o
/// modelo transcreve "use effect" em vez de "useEffect", "pacote ponto json" etc.
fn build_initial_prompt(app: &AppHandle, project_id: Option<i32>) -> String {
    let base = "Transcrição técnica de programação. Preserve nomes de arquivos, comandos e \
                tecnologias exatamente (ex.: package.json, useEffect, Tauri, SQLite).";
    let Some(project_id) = project_id else {
        return base.to_string();
    };

    if let Some(project) = app
        .state::<crate::storage::ProjectRepo>()
        .get(project_id)
        .ok()
        .flatten()
    {
        let mut extra = String::new();
        if !project.stack.trim().is_empty() {
            extra.push_str(&format!(" Stack do projeto: {}.", project.stack.trim()));
        }
        return format!("{base}{extra}");
    }
    base.to_string()
}

/// Transcreve a gravação `recording_id`: lê o WAV, roda o Whisper (com progresso via
/// `transcription:progress`), grava em `transcriptions`, apaga o WAV se `keep_audio` estiver
/// desligado, e atualiza o status da gravação. Retorna a transcrição salva.
///
/// Roda numa thread (via spawn_blocking) porque o Whisper prende a CPU por segundos — travar o
/// worker do Tauri deixaria toda a UI sem resposta.
#[tauri::command]
#[specta::specta]
pub async fn transcribe_recording(
    app: AppHandle,
    recording_id: i32,
) -> Result<Transcription, String> {
    tauri::async_runtime::spawn_blocking(move || transcribe_blocking(app, recording_id))
        .await
        .map_err(|e| e.to_string())?
}

fn transcribe_blocking(app: AppHandle, recording_id: i32) -> Result<Transcription, String> {
    let recordings = app.state::<RecordingRepo>();
    let recording = recordings
        .get(recording_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "gravação não encontrada".to_string())?;

    let audio_path = recording
        .audio_path
        .clone()
        .ok_or_else(|| "o áudio desta gravação não está mais disponível".to_string())?;

    let settings = app.state::<SettingsRepo>();
    let recording_settings = settings.get_recording_settings().map_err(|e| e.to_string())?;
    let model = resolve_model(&settings)?;

    let app_data = app.state::<AppDataDir>().0.clone();
    let engine = WhisperEngine::new(app_data, model);

    if engine.status() != EngineStatus::Ready {
        return Err("o modelo de transcrição ainda não foi baixado".to_string());
    }

    let _ = recordings.set_status(recording_id, "transcribing");

    let opts = TranscribeOptions {
        language: crate::transcription::DEFAULT_LANGUAGE.to_string(),
        initial_prompt: build_initial_prompt(&app, recording.project_id),
    };

    let app_for_progress = app.clone();
    let progress: Arc<dyn Fn(u8) + Send + Sync> = Arc::new(move |percent: u8| {
        let _ = app_for_progress.emit("transcription:progress", percent);
    });

    let result = engine.transcribe(std::path::Path::new(&audio_path), &opts, progress);

    match result {
        Ok(transcript) => {
            let saved = app
                .state::<TranscriptionRepo>()
                .create(&NewTranscription {
                    recording_id,
                    text: transcript.text,
                    language: transcript.language,
                    engine: "whisper-rs".into(),
                    model_name: model.id.to_string(),
                    duration_ms: transcript.processing_ms,
                })
                .map_err(|e| e.to_string())?;

            let _ = recordings.set_status(recording_id, "transcribed");

            // Privacidade (PRODUCT-SPEC §5.2): o WAV existe só para virar texto. Feito isso, é
            // apagado — a menos que o usuário tenha ligado "manter áudio".
            if !recording_settings.keep_audio {
                let _ = std::fs::remove_file(&audio_path);
                let _ = recordings.clear_audio_path(recording_id);
            }

            Ok(saved)
        }
        Err(err) => {
            let _ = recordings.set_status(recording_id, "failed");
            // Áudio vazio/silêncio também não deve ficar no disco.
            if matches!(err, TranscribeError::NoSpeech) && !recording_settings.keep_audio {
                let _ = std::fs::remove_file(&audio_path);
                let _ = recordings.clear_audio_path(recording_id);
            }
            Err(err.to_string())
        }
    }
}
