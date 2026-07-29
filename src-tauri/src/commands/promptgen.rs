use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

use crate::domain::{GeneratedPrompt, NewGeneratedPrompt, PromptMode};
use crate::promptgen::{
    claude_cli, modes, DefaultPromptGenerator, GenerationInput, ProjectContext, PromptGenerator,
    RefineAction,
};
use crate::storage::{
    ProjectRepo, ProjectRuleRepo, PromptRepo, RecordingRepo, TranscriptionRepo,
};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PromptModeOption {
    pub id: PromptMode,
    pub label: String,
    pub description: String,
}

/// Resultado da geração para a UI: o prompt salvo + aviso de fallback, se houve.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GenerationResult {
    pub prompt: GeneratedPrompt,
    /// `Some` quando o Claude CLI falhou e o template foi usado — a UI mostra isso.
    pub fallback_reason: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub fn list_prompt_modes() -> Result<Vec<PromptModeOption>, String> {
    Ok(modes::MODES
        .iter()
        .map(|spec| PromptModeOption {
            id: spec.mode,
            label: spec.label.to_string(),
            description: spec.description.to_string(),
        })
        .collect())
}

/// `true` se o comando `claude` existe no PATH. A UI usa para avisar que a geração vai cair no
/// template antes mesmo de o usuário tentar.
#[tauri::command]
#[specta::specta]
pub fn claude_cli_available() -> Result<bool, String> {
    Ok(claude_cli::is_available())
}

/// Monta o contexto do projeto (stack, regras, proibições) para injetar no prompt.
pub(super) fn load_context(app: &AppHandle, project_id: Option<i32>) -> ProjectContext {
    let Some(project_id) = project_id else {
        return ProjectContext::default();
    };

    let Some(project) = app
        .state::<ProjectRepo>()
        .get(project_id)
        .ok()
        .flatten()
    else {
        return ProjectContext::default();
    };

    let rules = app
        .state::<ProjectRuleRepo>()
        .list_for_project(project_id)
        .unwrap_or_default();

    ProjectContext::from_project(&project, &rules)
}

/// Gera um prompt a partir de uma transcrição já salva e persiste o resultado.
///
/// Roda em `spawn_blocking` porque o caminho do Claude CLI é um processo externo que pode
/// levar dezenas de segundos — travaria a UI se rodasse no worker do Tauri.
#[tauri::command]
#[specta::specta]
pub async fn generate_prompt(
    app: AppHandle,
    transcription_id: i32,
    mode: PromptMode,
) -> Result<GenerationResult, String> {
    tauri::async_runtime::spawn_blocking(move || generate_blocking(app, transcription_id, mode))
        .await
        .map_err(|e| e.to_string())?
}

fn generate_blocking(
    app: AppHandle,
    transcription_id: i32,
    mode: PromptMode,
) -> Result<GenerationResult, String> {
    let transcription = app
        .state::<TranscriptionRepo>()
        .get(transcription_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "transcrição não encontrada".to_string())?;

    // A gravação dá o projeto ativo e a duração do áudio para o histórico.
    let recording = app
        .state::<RecordingRepo>()
        .get(transcription.recording_id)
        .ok()
        .flatten();
    let project_id = recording.as_ref().and_then(|r| r.project_id);
    let audio_duration_ms = recording.as_ref().map(|r| r.duration_ms).unwrap_or(0);

    let context = load_context(&app, project_id);

    let generated = DefaultPromptGenerator::new()
        .generate(&GenerationInput {
            transcript: transcription.text.clone(),
            mode,
            project: context,
            refine: None,
        })
        .map_err(|e| e.to_string())?;

    let saved = app
        .state::<PromptRepo>()
        .create_with_history(
            &NewGeneratedPrompt {
                transcription_id: Some(transcription_id),
                project_id,
                mode: mode.as_db_str().to_string(),
                generator: generated.source.as_db_str().to_string(),
                content: generated.content,
            },
            audio_duration_ms,
            &transcription.text,
            Some(transcription.recording_id),
        )
        .map_err(|e| e.to_string())?;

    Ok(GenerationResult {
        prompt: saved,
        fallback_reason: generated.fallback_reason,
    })
}

/// Refina um prompt existente (encurtar/detalhar/mais técnico/dividir em etapas).
///
/// Sem o Claude CLI não há refino determinístico possível — o gerador devolve o prompt intacto
/// com um aviso, em vez de fingir que alterou.
#[tauri::command]
#[specta::specta]
pub async fn refine_prompt(
    app: AppHandle,
    prompt_id: i32,
    action: RefineAction,
) -> Result<GenerationResult, String> {
    tauri::async_runtime::spawn_blocking(move || refine_blocking(app, prompt_id, action))
        .await
        .map_err(|e| e.to_string())?
}

fn refine_blocking(
    app: AppHandle,
    prompt_id: i32,
    action: RefineAction,
) -> Result<GenerationResult, String> {
    let repo = app.state::<PromptRepo>();
    let existing = repo
        .get(prompt_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "prompt não encontrado".to_string())?;

    let mode = PromptMode::from_db_str(&existing.mode).unwrap_or(PromptMode::Technical);
    let context = load_context(&app, existing.project_id);

    let generated = DefaultPromptGenerator::new()
        .generate(&GenerationInput {
            transcript: String::new(),
            mode,
            project: context,
            refine: Some((action, existing.content.clone())),
        })
        .map_err(|e| e.to_string())?;

    let updated = repo
        .update_content(prompt_id, &generated.content)
        .map_err(|e| e.to_string())?;

    Ok(GenerationResult {
        prompt: updated,
        fallback_reason: generated.fallback_reason,
    })
}

/// Salva o texto editado pelo usuário (a edição livre é da Fase 7, mas o command já existe
/// para o editor consumir).
#[tauri::command]
#[specta::specta]
pub fn update_prompt_content(
    repo: tauri::State<'_, PromptRepo>,
    prompt_id: i32,
    content: String,
) -> Result<GeneratedPrompt, String> {
    repo.update_content(prompt_id, &content)
        .map_err(|e| e.to_string())
}
