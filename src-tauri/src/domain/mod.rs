use serde::{Deserialize, Serialize};
use specta::Type;

// IDs e durações usam i32, não i64: specta-typescript recusa exportar i64/BigInt por padrão
// (risco de precisão em JS acima de 2^53) e um app local nunca chega perto do teto de i32.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub description: String,
    pub stack: String,
    pub architecture: String,
    pub dev_commands: String,
    pub test_commands: String,
    pub forbidden_tech: String,
    pub database_info: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Campos aceitos ao criar um projeto. `path` já deve chegar validado/canonicalizado
/// pela camada de commands (ver SECURITY-MODEL.md §3 — validação entra na Fase 3).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewProject {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub stack: String,
    #[serde(default)]
    pub architecture: String,
    #[serde(default)]
    pub dev_commands: String,
    #[serde(default)]
    pub test_commands: String,
    #[serde(default)]
    pub forbidden_tech: String,
    #[serde(default)]
    pub database_info: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProjectUpdate {
    pub name: String,
    pub description: String,
    pub stack: String,
    pub architecture: String,
    pub dev_commands: String,
    pub test_commands: String,
    pub forbidden_tech: String,
    pub database_info: String,
    pub notes: String,
}

/// Regra de projeto injetada no contexto do prompt (DATABASE-SCHEMA.md `project_rules`).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProjectRule {
    pub id: i32,
    pub project_id: i32,
    pub rule: String,
    pub sort_order: i32,
    pub created_at: String,
}

/// Nova regra: `sort_order` não é aceito do cliente — o repositório sempre acrescenta ao
/// final da lista (maior `sort_order` do projeto + 1). Reordenar é uma operação à parte
/// (`ProjectRuleRepo::reorder`).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewProjectRule {
    pub project_id: i32,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProjectRuleUpdate {
    pub rule: String,
}

/// Os 10 modos do gerador de prompts (PRODUCT-SPEC.md §5.4). A representação em banco é o
/// `as_db_str()` — mantém o `CHECK` da coluna `mode` em `generated_prompts`/`prompt_history`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PromptMode {
    CleanTranscript,
    Quick,
    Technical,
    NewFeature,
    BugFix,
    Refactor,
    Planning,
    CodeReview,
    UiCreation,
    DbChange,
}

impl PromptMode {
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::CleanTranscript => "clean_transcript",
            Self::Quick => "quick",
            Self::Technical => "technical",
            Self::NewFeature => "new_feature",
            Self::BugFix => "bug_fix",
            Self::Refactor => "refactor",
            Self::Planning => "planning",
            Self::CodeReview => "code_review",
            Self::UiCreation => "ui_creation",
            Self::DbChange => "db_change",
        }
    }

    /// Inverso de `as_db_str`, para reconstruir o modo a partir de uma linha do banco.
    pub fn from_db_str(value: &str) -> Option<Self> {
        Some(match value {
            "clean_transcript" => Self::CleanTranscript,
            "quick" => Self::Quick,
            "technical" => Self::Technical,
            "new_feature" => Self::NewFeature,
            "bug_fix" => Self::BugFix,
            "refactor" => Self::Refactor,
            "planning" => Self::Planning,
            "code_review" => Self::CodeReview,
            "ui_creation" => Self::UiCreation,
            "db_change" => Self::DbChange,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod prompt_mode_tests {
    use super::PromptMode;

    #[test]
    fn db_str_round_trips_for_every_mode() {
        for mode in [
            PromptMode::CleanTranscript,
            PromptMode::Quick,
            PromptMode::Technical,
            PromptMode::NewFeature,
            PromptMode::BugFix,
            PromptMode::Refactor,
            PromptMode::Planning,
            PromptMode::CodeReview,
            PromptMode::UiCreation,
            PromptMode::DbChange,
        ] {
            assert_eq!(PromptMode::from_db_str(mode.as_db_str()), Some(mode));
        }
    }

    #[test]
    fn unknown_db_value_is_none() {
        assert_eq!(PromptMode::from_db_str("modo_inventado"), None);
    }
}

/// Quem gerou o prompt (coluna `generated_prompts.generator`). Nome evita colidir com a
/// trait `PromptGenerator` que a Fase 6 vai definir em `promptgen/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PromptSource {
    ClaudeCli,
    Template,
}

impl PromptSource {
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::ClaudeCli => "claude_cli",
            Self::Template => "template",
        }
    }
}

/// Entrada para `HistoryRepo::save_flow` — grava gravação + transcrição + prompt + histórico
/// numa única transação (DATABASE-SCHEMA.md §5).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewFlow {
    pub project_id: Option<i32>,
    pub duration_ms: i32,
    pub device_name: String,
    pub transcript_text: String,
    pub transcript_language: String,
    pub engine: String,
    pub model_name: String,
    pub transcribe_duration_ms: i32,
    pub mode: PromptMode,
    pub generator: PromptSource,
    pub prompt_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PromptHistoryEntry {
    pub id: i32,
    pub project_id: Option<i32>,
    pub mode: String,
    pub favorite: bool,
    pub audio_duration_ms: i32,
    pub status: String,
    pub created_at: String,
}

/// Metadados de uma gravação. `audio_path` é `None` depois que o WAV temporário é apagado
/// (o caso normal — ver `RecordingRepo::clear_audio_path`).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Recording {
    pub id: i32,
    pub project_id: Option<i32>,
    pub duration_ms: i32,
    pub device_name: String,
    pub audio_path: Option<String>,
    pub audio_kept: bool,
    /// `recorded` | `transcribing` | `transcribed` | `failed` | `cancelled` (CHECK no schema).
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewRecording {
    pub project_id: Option<i32>,
    pub duration_ms: i32,
    pub device_name: String,
    pub audio_path: Option<String>,
    pub audio_kept: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Transcription {
    pub id: i32,
    pub recording_id: i32,
    pub text: String,
    pub language: String,
    pub engine: String,
    pub model_name: String,
    /// Tempo de processamento (ms), não a duração do áudio.
    pub duration_ms: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewTranscription {
    pub recording_id: i32,
    pub text: String,
    pub language: String,
    pub engine: String,
    pub model_name: String,
    pub duration_ms: i32,
}

/// Prompt gerado. `content` é a versão atual (pode ter sido editada pelo usuário na Fase 7);
/// `original_content` guarda como saiu do gerador, para comparação/reversão.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GeneratedPrompt {
    pub id: i32,
    pub transcription_id: Option<i32>,
    pub project_id: Option<i32>,
    /// Valor de `PromptMode::as_db_str()`.
    pub mode: String,
    /// Valor de `PromptSource::as_db_str()` — `claude_cli` ou `template`.
    pub generator: String,
    pub content: String,
    pub original_content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewGeneratedPrompt {
    pub transcription_id: Option<i32>,
    pub project_id: Option<i32>,
    pub mode: String,
    pub generator: String,
    pub content: String,
}

/// Modelo de prompt reutilizável (`prompt_templates`). Vem de duas origens: a biblioteca
/// embutida no binário (`source = "builtin"`, com `slug` estável) e o "salvar como modelo" do
/// editor (`source = "user"`, `slug` nulo).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PromptTemplate {
    pub id: i32,
    pub name: String,
    /// Valor de `PromptMode::as_db_str()` (CHECK na coluna desde a migration 003).
    pub mode: String,
    /// Slug da pasta de origem em `templates/` (ex.: `depuracao`); vazio nos modelos do usuário.
    pub category: String,
    /// A linha "> Uso:" do modelo — o que ele serve para fazer, em uma frase.
    pub description: String,
    pub content: String,
    /// `builtin` (biblioteca embutida, somente leitura) ou `user`.
    pub source: String,
    pub slug: Option<String>,
    pub project_id: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

/// Modelo criado pelo usuário. `source`/`slug` não são aceitos do cliente: o repositório grava
/// sempre `user`/NULL, para que a UI não consiga forjar um modelo "embutido" — que é justamente
/// a classe que o seed do startup apaga e reescreve.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NewPromptTemplate {
    pub name: String,
    pub mode: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    pub content: String,
    pub project_id: Option<i32>,
}
