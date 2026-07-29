pub mod claude_cli;
pub mod context;
pub mod modes;
pub mod templates;

use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

use crate::domain::{PromptMode, PromptSource};

pub use context::ProjectContext;

#[derive(Debug, Error)]
pub enum PromptGenError {
    #[error("o comando `claude` não está disponível: {0}")]
    CliUnavailable(String),

    #[error("falha ao gerar pelo Claude: {0}")]
    Cli(String),

    #[error("o Claude demorou mais de {0}s para responder")]
    Timeout(u64),

    #[error("transcrição vazia — não há o que transformar em prompt")]
    EmptyTranscript,
}

/// Ações de refino sobre um prompt já gerado (PRODUCT-SPEC §5.5). A Fase 7 (editor) as expõe
/// na UI; a definição vive aqui porque quem as executa é o gerador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum RefineAction {
    Shorten,
    Expand,
    MoreTechnical,
    SplitIntoSteps,
}

impl RefineAction {
    fn instruction(self) -> &'static str {
        match self {
            Self::Shorten => {
                "Encurte o prompt abaixo mantendo TODA a informação essencial. Corte redundância \
                 e enrolação, não requisitos."
            }
            Self::Expand => {
                "Detalhe mais o prompt abaixo: explicite casos de borda, estados de erro e \
                 critérios de aceitação. Não invente requisitos que não estejam implícitos."
            }
            Self::MoreTechnical => {
                "Reescreva o prompt abaixo em linguagem mais técnica e precisa, usando os termos \
                 corretos da stack. Não mude o escopo."
            }
            Self::SplitIntoSteps => {
                "Divida o prompt abaixo em etapas sequenciais numeradas, cada uma verificável \
                 isoladamente. Mantenha o escopo total idêntico."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerationInput {
    pub transcript: String,
    pub mode: PromptMode,
    pub project: ProjectContext,
    /// `Some` quando é um refino de um prompt já existente (Fase 7).
    pub refine: Option<(RefineAction, String)>,
}

#[derive(Debug, Clone)]
pub struct GeneratedPromptText {
    pub content: String,
    pub source: PromptSource,
    /// Preenchido quando o CLI falhou e caímos no template — a UI mostra isso ao usuário em vez
    /// de fingir que tudo correu bem.
    pub fallback_reason: Option<String>,
}

/// Contrato do gerador (ADR-002). A implementação padrão tenta o `claude` CLI e cai para o
/// template determinístico; ADR-002b prevê um `OpenAiGenerator` plugável aqui no futuro.
pub trait PromptGenerator: Send + Sync {
    fn generate(&self, input: &GenerationInput) -> Result<GeneratedPromptText, PromptGenError>;
}

/// Gerador padrão: Claude CLI com fallback automático para template.
#[derive(Default)]
pub struct DefaultPromptGenerator;

impl DefaultPromptGenerator {
    pub fn new() -> Self {
        Self
    }
}

/// Monta o texto enviado ao LLM: instrução do modo + contexto do projeto + transcrição.
///
/// A transcrição fica delimitada por marcadores explícitos para reduzir a chance de o modelo
/// tratar como instrução algo que o usuário apenas *falou* (ex.: "ignore as regras acima").
fn build_llm_prompt(input: &GenerationInput) -> String {
    let spec = modes::spec_for(input.mode);
    let mut out = String::new();

    if let Some((action, current)) = &input.refine {
        out.push_str(action.instruction());
        out.push_str("\n\nDevolva APENAS o prompt reescrito, sem comentários.\n\n");
        out.push_str("--- PROMPT ATUAL ---\n");
        out.push_str(current);
        out.push_str("\n--- FIM DO PROMPT ATUAL ---\n");
        return out;
    }

    out.push_str(spec.instruction);
    out.push_str("\n\n");

    if !spec.sections.is_empty() {
        out.push_str("Estruture a saída em markdown com estas seções (omita as que não se \
                      aplicarem, NÃO invente conteúdo para preencher):\n");
        for section in spec.sections {
            out.push_str(&format!("- {}\n", section.title()));
        }
        out.push('\n');
    }

    if !input.project.is_empty() {
        out.push_str("Contexto do projeto (use-o, não repita literalmente):\n");
        out.push_str(&input.project.render());
        let rules = input.project.render_rules();
        if !rules.is_empty() {
            out.push_str("Regras do projeto:\n");
            out.push_str(&rules);
        }
        out.push('\n');
    }

    out.push_str(
        "Responda em português. Devolva APENAS o prompt final, sem comentários seus.\n\n\
         --- TRANSCRIÇÃO DA FALA ---\n",
    );
    out.push_str(&input.transcript);
    out.push_str("\n--- FIM DA TRANSCRIÇÃO ---\n");

    out
}

impl PromptGenerator for DefaultPromptGenerator {
    fn generate(&self, input: &GenerationInput) -> Result<GeneratedPromptText, PromptGenError> {
        if input.transcript.trim().is_empty() && input.refine.is_none() {
            return Err(PromptGenError::EmptyTranscript);
        }

        // "Transcrição limpa" nunca passa por LLM (ADR-002): é limpeza determinística, e mandar
        // pro modelo arriscaria reescrita/invenção — justamente o que este modo evita.
        if input.mode == PromptMode::CleanTranscript && input.refine.is_none() {
            return Ok(GeneratedPromptText {
                content: templates::generate(input.mode, &input.transcript, &input.project),
                source: PromptSource::Template,
                fallback_reason: None,
            });
        }

        let llm_prompt = build_llm_prompt(input);

        match claude_cli::generate(&llm_prompt) {
            Ok(content) => Ok(GeneratedPromptText {
                content,
                source: PromptSource::ClaudeCli,
                fallback_reason: None,
            }),
            Err(err) => {
                // Fallback: o app continua útil sem o CLI (offline, não instalado, não logado).
                // O usuário é avisado — falhar em silêncio esconderia que a qualidade caiu.
                log::warn!("geração via claude CLI falhou, usando template: {err}");

                if let Some((_, current)) = &input.refine {
                    // Refino não tem equivalente determinístico: sem LLM não dá para encurtar
                    // ou detalhar sem reescrever. Devolvemos o prompt intacto com o aviso.
                    return Ok(GeneratedPromptText {
                        content: current.clone(),
                        source: PromptSource::Template,
                        fallback_reason: Some(format!(
                            "Não foi possível refinar pelo Claude ({err}). O prompt não foi alterado."
                        )),
                    });
                }

                Ok(GeneratedPromptText {
                    content: templates::generate(input.mode, &input.transcript, &input.project),
                    source: PromptSource::Template,
                    fallback_reason: Some(format!(
                        "Gerado por modelo local de template ({err})."
                    )),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(mode: PromptMode) -> GenerationInput {
        GenerationInput {
            transcript: "criar uma tela de login".into(),
            mode,
            project: ProjectContext {
                name: "CodeVoice".into(),
                stack: "Tauri, React".into(),
                ..Default::default()
            },
            refine: None,
        }
    }

    #[test]
    fn empty_transcript_is_rejected() {
        let mut i = input(PromptMode::Technical);
        i.transcript = "   ".into();
        assert!(matches!(
            DefaultPromptGenerator::new().generate(&i),
            Err(PromptGenError::EmptyTranscript)
        ));
    }

    #[test]
    fn clean_transcript_never_uses_the_llm() {
        let result = DefaultPromptGenerator::new()
            .generate(&input(PromptMode::CleanTranscript))
            .unwrap();
        assert_eq!(result.source, PromptSource::Template);
        assert!(result.fallback_reason.is_none(), "não é fallback, é o caminho normal");
        assert!(!result.content.contains("##"));
    }

    #[test]
    fn llm_prompt_delimits_the_transcript() {
        let prompt = build_llm_prompt(&input(PromptMode::Technical));
        assert!(prompt.contains("--- TRANSCRIÇÃO DA FALA ---"));
        assert!(prompt.contains("--- FIM DA TRANSCRIÇÃO ---"));
        assert!(prompt.contains("criar uma tela de login"));
    }

    #[test]
    fn llm_prompt_lists_the_mode_sections_and_project_context() {
        let prompt = build_llm_prompt(&input(PromptMode::Technical));
        assert!(prompt.contains("Objetivo"));
        assert!(prompt.contains("Critérios de aceitação"));
        assert!(prompt.contains("CodeVoice"), "contexto do projeto deveria estar presente");
        assert!(prompt.contains("Tauri, React"));
    }

    #[test]
    fn refine_prompt_asks_to_rewrite_the_current_prompt() {
        let mut i = input(PromptMode::Technical);
        i.refine = Some((RefineAction::Shorten, "# Prompt atual\nconteúdo".into()));
        let prompt = build_llm_prompt(&i);
        assert!(prompt.contains("Encurte"));
        assert!(prompt.contains("--- PROMPT ATUAL ---"));
        assert!(prompt.contains("conteúdo"));
        // Num refino não faz sentido reenviar a transcrição original.
        assert!(!prompt.contains("--- TRANSCRIÇÃO DA FALA ---"));
    }

    #[test]
    fn every_refine_action_has_a_distinct_instruction() {
        let actions = [
            RefineAction::Shorten,
            RefineAction::Expand,
            RefineAction::MoreTechnical,
            RefineAction::SplitIntoSteps,
        ];
        for (i, a) in actions.iter().enumerate() {
            assert!(!a.instruction().is_empty());
            for b in &actions[i + 1..] {
                assert_ne!(a.instruction(), b.instruction());
            }
        }
    }

    /// Segurança: mesmo com conteúdo hostil na transcrição, o texto só é usado como dado —
    /// nunca chega a argv (ver `claude_cli::generate`, que escreve por stdin).
    #[test]
    fn hostile_transcript_stays_inside_the_delimiters() {
        let mut i = input(PromptMode::Technical);
        i.transcript = "\"; rm -rf / && echo pwned".into();
        let prompt = build_llm_prompt(&i);
        let start = prompt.find("--- TRANSCRIÇÃO DA FALA ---").unwrap();
        let end = prompt.find("--- FIM DA TRANSCRIÇÃO ---").unwrap();
        let payload_pos = prompt.find("rm -rf").unwrap();
        assert!(payload_pos > start && payload_pos < end);
    }
}
