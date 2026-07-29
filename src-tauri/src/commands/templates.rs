use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

use crate::domain::{NewGeneratedPrompt, NewPromptTemplate, PromptTemplate};
use crate::promptgen::library;
use crate::storage::{PromptRepo, PromptTemplateRepo, RecordingRepo, TranscriptionRepo};

use super::promptgen::{load_context, GenerationResult};

/// Categoria criada pelo "salvar como modelo". Fica separada das 18 pastas da biblioteca para
/// que os modelos do usuário não se percam no meio dos 117 embutidos.
pub const USER_CATEGORY: &str = "meus-modelos";

/// Rótulos legíveis das categorias. Os ids são nomes de pasta (`templates/`), que são estáveis;
/// o rótulo é apresentação e mora aqui, na borda IPC, não no domínio.
const CATEGORY_LABELS: &[(&str, &str)] = &[
    (USER_CATEGORY, "Meus modelos"),
    ("arquitetura-design", "Arquitetura e design"),
    ("backend-avancado", "Backend avançado"),
    ("dados-ia", "Dados e IA"),
    ("depuracao", "Depuração"),
    ("desenvolvimento", "Desenvolvimento"),
    ("devops-infra", "DevOps e infraestrutura"),
    ("documentacao-processo", "Documentação e processo"),
    ("escrita-conteudo", "Escrita e conteúdo"),
    ("especialidades", "Especialidades"),
    ("fala-rapida", "Fala rápida"),
    ("frontend-ui", "Frontend e UI"),
    ("ia-engenharia", "Engenharia de IA"),
    ("manutencao-legado", "Manutenção e legado"),
    ("mobile-desktop", "Mobile e desktop"),
    ("negocios-produto", "Negócios e produto"),
    ("pessoal-aprendizado", "Pessoal e aprendizado"),
    ("qualidade-seguranca", "Qualidade e segurança"),
    ("testes-avancados", "Testes avançados"),
];

fn label_for(category: &str) -> String {
    CATEGORY_LABELS
        .iter()
        .find(|(id, _)| *id == category)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| category.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TemplateCategory {
    pub id: String,
    pub label: String,
    pub count: i32,
}

#[tauri::command]
#[specta::specta]
pub fn list_template_categories(
    repo: tauri::State<'_, PromptTemplateRepo>,
) -> Result<Vec<TemplateCategory>, String> {
    let categories = repo.categories().map_err(|e| e.to_string())?;
    Ok(categories
        .into_iter()
        .map(|(id, count)| TemplateCategory { label: label_for(&id), id, count })
        .collect())
}

/// Lista os modelos de uma categoria, ou todos quando `category` é `None`.
#[tauri::command]
#[specta::specta]
pub fn list_prompt_templates(
    repo: tauri::State<'_, PromptTemplateRepo>,
    category: Option<String>,
) -> Result<Vec<PromptTemplate>, String> {
    match category {
        Some(category) => repo.list_by_category(&category),
        None => repo.list(),
    }
    .map_err(|e| e.to_string())
}

/// Salva o prompt atual (já com as edições do usuário) como modelo reutilizável.
#[tauri::command]
#[specta::specta]
pub fn save_prompt_as_template(
    app: AppHandle,
    prompt_id: i32,
    name: String,
    description: String,
) -> Result<PromptTemplate, String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("dê um nome ao modelo".into());
    }

    let prompt = app
        .state::<PromptRepo>()
        .get(prompt_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "prompt não encontrado".to_string())?;

    app.state::<PromptTemplateRepo>()
        .create(&NewPromptTemplate {
            name,
            mode: prompt.mode,
            category: USER_CATEGORY.to_string(),
            description: description.trim().to_string(),
            content: prompt.content,
            project_id: prompt.project_id,
        })
        .map_err(|e| e.to_string())
}

/// Exclui um modelo do usuário. Modelos da biblioteca embutida são recusados pelo repositório —
/// apagá-los só duraria até o próximo startup, quando o seed os traria de volta.
#[tauri::command]
#[specta::specta]
pub fn delete_prompt_template(
    repo: tauri::State<'_, PromptTemplateRepo>,
    template_id: i32,
) -> Result<(), String> {
    repo.delete_user_template(template_id).map_err(|e| e.to_string())
}

/// Gera um prompt usando um modelo como base, em vez dos geradores do modo.
///
/// Não passa pelo Claude CLI: o modelo **é** o prompt: o que falta é encaixar a fala e os dados do
/// projeto. Mandar ao LLM só arriscaria reescrever um texto que o usuário escolheu deliberadamente.
#[tauri::command]
#[specta::specta]
pub fn generate_prompt_from_template(
    app: AppHandle,
    transcription_id: i32,
    template_id: i32,
) -> Result<GenerationResult, String> {
    let template = app
        .state::<PromptTemplateRepo>()
        .get(template_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "modelo não encontrado".to_string())?;

    let transcription = app
        .state::<TranscriptionRepo>()
        .get(transcription_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "transcrição não encontrada".to_string())?;

    let recording = app
        .state::<RecordingRepo>()
        .get(transcription.recording_id)
        .ok()
        .flatten();
    let project_id = recording.as_ref().and_then(|r| r.project_id);
    let audio_duration_ms = recording.as_ref().map(|r| r.duration_ms).unwrap_or(0);

    let context = load_context(&app, project_id);
    let content = library::render(&template.content, &transcription.text, &context);

    let saved = app
        .state::<PromptRepo>()
        .create_with_history(
            &NewGeneratedPrompt {
                transcription_id: Some(transcription_id),
                project_id,
                mode: template.mode,
                generator: "template".to_string(),
                content,
            },
            audio_duration_ms,
            &transcription.text,
            Some(transcription.recording_id),
        )
        .map_err(|e| e.to_string())?;

    Ok(GenerationResult { prompt: saved, fallback_reason: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_library_category_has_a_label() {
        let mut categories: Vec<_> =
            library::builtins().into_iter().map(|t| t.category).collect();
        categories.sort();
        categories.dedup();

        for category in categories {
            assert_ne!(
                label_for(&category),
                category,
                "categoria {category} caiu no fallback: falta rótulo em CATEGORY_LABELS"
            );
        }
    }

    #[test]
    fn unknown_category_falls_back_to_its_id() {
        assert_eq!(label_for("categoria-nova"), "categoria-nova");
    }
}
