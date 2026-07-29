use crate::domain::PromptMode;

use super::context::ProjectContext;
use super::modes::{spec_for, Section};

/// Hesitações comuns da fala em PT que não agregam nada ao prompt.
const FILLERS: &[&str] = &[
    " né ", " tipo assim ", " tipo, ", " ãã ", " ahn ", " hmm ", " então assim ",
];

/// Limpeza determinística da transcrição: remove hesitações, colapsa espaços e garante
/// pontuação final. **Não reescreve** o conteúdo — sem LLM não há como reformular sem risco de
/// inventar; o objetivo aqui é não estragar, não embelezar.
pub fn clean_transcript(raw: &str) -> String {
    let mut text = format!(" {} ", raw.trim());

    for filler in FILLERS {
        // Case-insensitive: a transcrição pode vir capitalizada no início de frase.
        let mut idx = 0;
        while let Some(found) = text[idx..].to_lowercase().find(filler) {
            let start = idx + found;
            let end = start + filler.len();
            text.replace_range(start..end, " ");
            idx = start;
        }
    }

    let mut cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");

    if !cleaned.is_empty() {
        // Capitaliza a primeira letra.
        let mut chars = cleaned.chars();
        if let Some(first) = chars.next() {
            cleaned = first.to_uppercase().collect::<String>() + chars.as_str();
        }
        // Fecha com pontuação se o Whisper não fechou.
        if !cleaned.ends_with(['.', '!', '?', ':']) {
            cleaned.push('.');
        }
    }

    cleaned
}

/// Gera o prompt de forma determinística, sem LLM.
///
/// É o fallback quando o `claude` CLI não está disponível (offline, não instalado, não logado)
/// — e o único caminho do modo "Transcrição limpa", que por definição não deve passar por um
/// modelo. O prompt sai estruturado com as seções do modo; as que dependem de interpretação
/// ficam com instruções para o agente preencher, em vez de conteúdo inventado.
pub fn generate(mode: PromptMode, transcript: &str, ctx: &ProjectContext) -> String {
    let spec = spec_for(mode);
    let cleaned = clean_transcript(transcript);

    // Transcrição limpa é texto corrido, não prompt.
    if spec.sections.is_empty() {
        return cleaned;
    }

    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", spec.label));

    for section in spec.sections {
        let body = render_section(*section, &cleaned, ctx);
        if body.trim().is_empty() {
            continue; // omite seções vazias (PRODUCT-SPEC §5.4)
        }
        out.push_str(&format!("## {}\n{}\n\n", section.title(), body.trim_end()));
    }

    out.trim_end().to_string()
}

fn render_section(section: Section, transcript: &str, ctx: &ProjectContext) -> String {
    match section {
        Section::Objetivo => transcript.to_string(),

        Section::ContextoProjeto => ctx.render(),

        Section::Regras => {
            let mut body = ctx.render_rules();
            if !ctx.forbidden_tech.trim().is_empty() {
                body.push_str(&format!("- Não usar: {}\n", ctx.forbidden_tech.trim()));
            }
            body
        }

        Section::Restricoes => {
            let mut body = String::from("- Não alterar funcionalidades não relacionadas.\n");
            if !ctx.forbidden_tech.trim().is_empty() {
                body.push_str(&format!(
                    "- Tecnologias proibidas neste projeto: {}.\n",
                    ctx.forbidden_tech.trim()
                ));
            }
            body
        }

        Section::Validacoes => {
            let mut body = String::new();
            if !ctx.test_commands.trim().is_empty() {
                body.push_str(&format!(
                    "- Rodar os testes do projeto: {}\n",
                    ctx.test_commands.trim()
                ));
            }
            if !ctx.dev_commands.trim().is_empty() {
                body.push_str(&format!(
                    "- Verificar que a aplicação sobe: {}\n",
                    ctx.dev_commands.trim()
                ));
            }
            body.push_str("- Não declarar concluído sem executar a validação.\n");
            body
        }

        Section::FormatoRelatorio => String::from(
            "Ao final, relate: o que foi feito, arquivos criados/modificados, decisões \
             tomadas, e o que ficou pendente.\n",
        ),

        // Seções que dependem de interpretação: sem LLM, instruímos o agente a derivá-las do
        // objetivo em vez de inventar conteúdo (que seria pior que deixar em branco).
        Section::RequisitosFuncionais => {
            "Derivar do objetivo acima. Se algo estiver ambíguo, perguntar antes de implementar.\n"
                .to_string()
        }
        Section::RequisitosTecnicos => {
            if ctx.stack.trim().is_empty() && ctx.architecture.trim().is_empty() {
                String::new()
            } else {
                let mut body = String::new();
                if !ctx.stack.trim().is_empty() {
                    body.push_str(&format!("- Manter a stack do projeto: {}\n", ctx.stack.trim()));
                }
                if !ctx.architecture.trim().is_empty() {
                    body.push_str(&format!(
                        "- Respeitar a arquitetura existente: {}\n",
                        ctx.architecture.trim()
                    ));
                }
                body
            }
        }
        Section::ArquivosRelacionados => {
            "Localizar no projeto antes de editar; não assumir caminhos.\n".to_string()
        }
        Section::PlanoEsperado => {
            "Apresentar um plano curto antes de alterar código, e seguir esse plano.\n".to_string()
        }
        Section::CriteriosAceite => {
            "O objetivo acima está cumprido, os testes passam, e nada não relacionado foi \
             alterado.\n"
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> ProjectContext {
        ProjectContext {
            name: "CodeVoice".into(),
            path: "C:\\p".into(),
            stack: "Tauri, React".into(),
            test_commands: "cargo test".into(),
            forbidden_tech: "jQuery".into(),
            rules: vec!["Sempre rodar lint".into()],
            ..Default::default()
        }
    }

    #[test]
    fn cleans_fillers_and_punctuates() {
        let out = clean_transcript("então assim  eu quero né criar uma tela");
        assert!(!out.to_lowercase().contains(" né "));
        assert!(!out.to_lowercase().contains("então assim"));
        assert!(out.ends_with('.'));
        assert!(out.starts_with('E'), "primeira letra deveria ser maiúscula: {out}");
    }

    #[test]
    fn keeps_existing_final_punctuation() {
        assert!(clean_transcript("Isso funciona?").ends_with('?'));
    }

    #[test]
    fn clean_transcript_of_empty_input_is_empty() {
        assert_eq!(clean_transcript("   "), "");
    }

    #[test]
    fn preserves_technical_terms() {
        let out = clean_transcript("criar o package.json e usar useEffect");
        assert!(out.contains("package.json"));
        assert!(out.contains("useEffect"));
    }

    #[test]
    fn clean_mode_returns_plain_text_not_a_prompt() {
        let out = generate(PromptMode::CleanTranscript, "quero criar uma tela", &ctx());
        assert!(!out.contains("##"), "não deveria ter seções: {out}");
        assert!(out.contains("criar uma tela"));
    }

    #[test]
    fn technical_mode_has_headings_and_objective() {
        let out = generate(PromptMode::Technical, "criar tela de login", &ctx());
        assert!(out.starts_with("# Prompt técnico"));
        assert!(out.contains("## Objetivo"));
        assert!(out.contains("criar tela de login") || out.contains("Criar tela de login"));
        assert!(out.contains("## Critérios de aceitação"));
    }

    #[test]
    fn injects_project_context_and_rules() {
        let out = generate(PromptMode::Technical, "fazer algo", &ctx());
        assert!(out.contains("CodeVoice"), "faltou o nome do projeto");
        assert!(out.contains("Tauri, React"), "faltou a stack");
        assert!(out.contains("Sempre rodar lint"), "faltou a regra do projeto");
        assert!(out.contains("jQuery"), "faltou a tecnologia proibida");
        assert!(out.contains("cargo test"), "faltou o comando de teste");
    }

    #[test]
    fn omits_empty_sections_when_there_is_no_project() {
        let out = generate(PromptMode::Technical, "fazer algo", &ProjectContext::default());
        // Sem projeto não há contexto para renderizar — a seção não deve aparecer vazia.
        assert!(!out.contains("## Contexto do projeto"));
        // Mas o objetivo continua lá.
        assert!(out.contains("## Objetivo"));
    }

    #[test]
    fn every_mode_produces_non_empty_output() {
        let modes = [
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
        ];
        for mode in modes {
            let out = generate(mode, "preciso ajustar o login", &ctx());
            assert!(!out.trim().is_empty(), "modo {mode:?} gerou saída vazia");
        }
    }

    #[test]
    fn bug_fix_mode_asks_for_root_cause_context() {
        let out = generate(PromptMode::BugFix, "o botão não salva", &ctx());
        assert!(out.starts_with("# Correção de erro"));
        assert!(out.contains("## Validações"));
    }
}
