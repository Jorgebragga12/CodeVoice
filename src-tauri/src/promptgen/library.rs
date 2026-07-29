//! Biblioteca de modelos de prompt embutida no binário (`templates/`).
//!
//! Cada arquivo tem 4 linhas de cabeçalho que são **metadado**, não corpo de prompt:
//!
//! ```markdown
//! # Erro agora (colar stack trace)
//!
//! > Modo: `bug_fix` · Área: depuração
//! > Uso: quebrou agora, tenho a mensagem de erro na tela.
//!
//! ---
//!
//! Estou com um erro no projeto **[nome do projeto]**.
//! ```
//!
//! Gravar o arquivo inteiro em `content` faria todo prompt gerado começar com
//! "> Modo: bug_fix · Área: depuração" — daí o parser separar cabeçalho de corpo.
//!
//! A **categoria vem da pasta**, não do campo `Área`: são 22 valores distintos de `Área` para 18
//! pastas (`negocios-produto/` sozinha produz "negócios" e "produto"), e a pasta é o que casa com
//! a navegação do repositório e com o índice do README.

use crate::domain::PromptMode;
use crate::storage::BuiltinTemplate;

use super::context::ProjectContext;

include!(concat!(env!("OUT_DIR"), "/embedded_templates.rs"));

/// Marcador de onde entra a transcrição da fala, em todos os 117 modelos da biblioteca.
///
/// Convenção travada na Fase 7 sobre a alternativa `{{transcript}}` que o DATABASE-SCHEMA.md
/// documentava sem nunca ter virado código: `<<SUA FALA>>` já está nos arquivos versionados e é
/// legível para quem lê os modelos direto no repositório.
pub const SPEECH_MARKER: &str = "<<SUA FALA>>";

/// Campos entre colchetes que o app consegue preencher com **dado** que já tem no banco.
///
/// Deliberadamente curta: dos 676 campos `[entre colchetes]` da biblioteca, a maioria é decisão
/// do usuário (`[N]`, `[valor]`, `[período]`), não dado. Preencher esses com chute seria pior que
/// deixar o literal visível pedindo preenchimento.
const AUTO_FIELDS: &[&str] = &["[nome do projeto]", "[comando de teste]"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedTemplate {
    pub name: String,
    pub mode: String,
    pub description: String,
    pub content: String,
}

/// Lê o cabeçalho e devolve o corpo do modelo. `None` quando o arquivo não segue o formato
/// (sem título, sem modo válido ou sem separador `---`) — melhor deixar de fora que importar
/// um modelo quebrado.
pub fn parse(markdown: &str) -> Option<ParsedTemplate> {
    let (header, body) = split_at_separator(markdown)?;

    let mut name = None;
    let mut mode = None;
    let mut description = String::new();

    for line in header.lines() {
        let line = line.trim();
        if let Some(title) = line.strip_prefix("# ") {
            name.get_or_insert_with(|| title.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("> Modo:") {
            mode = parse_mode(rest);
        } else if let Some(rest) = line.strip_prefix("> Uso:") {
            description = rest.trim().to_string();
        }
    }

    let body = body.trim();
    if body.is_empty() {
        return None;
    }

    Some(ParsedTemplate {
        name: name?,
        mode: mode?,
        description,
        content: body.to_string(),
    })
}

/// Corta no primeiro `---` isolado. O separador está sempre na linha 6 dos modelos atuais, mas
/// procurar em vez de fatiar por número de linha evita que um modelo novo com uma linha a mais
/// no cabeçalho entre no banco com o metadado colado no corpo.
fn split_at_separator(markdown: &str) -> Option<(&str, &str)> {
    let mut offset = 0;
    for line in markdown.split_inclusive('\n') {
        if line.trim() == "---" {
            return Some((&markdown[..offset], &markdown[offset + line.len()..]));
        }
        offset += line.len();
    }
    None
}

/// Extrai o modo de `` `bug_fix` · Área: depuração``, validando contra os 10 modos conhecidos.
fn parse_mode(rest: &str) -> Option<String> {
    let raw = rest.split('·').next()?.trim().trim_matches('`').trim();
    PromptMode::from_db_str(raw).map(|m| m.as_db_str().to_string())
}

/// Categoria = nome da pasta do slug (`depuracao/erro-agora` → `depuracao`).
fn category_of(slug: &str) -> String {
    slug.split('/').next().unwrap_or_default().to_string()
}

/// Todos os modelos embutidos, prontos para o seed. Arquivos fora do formato são ignorados com
/// aviso no log (o teste `every_embedded_template_parses` impede que isso passe despercebido).
pub fn builtins() -> Vec<BuiltinTemplate> {
    EMBEDDED_TEMPLATES
        .iter()
        .filter_map(|(slug, markdown)| {
            let parsed = parse(markdown).or_else(|| {
                log::warn!("modelo embutido fora do formato esperado, ignorado: {slug}");
                None
            })?;
            Some(BuiltinTemplate {
                slug: (*slug).to_string(),
                name: parsed.name,
                mode: parsed.mode,
                category: category_of(slug),
                description: parsed.description,
                content: parsed.content,
            })
        })
        .collect()
}

/// Aplica um modelo: coloca a transcrição no lugar do marcador e preenche os poucos campos que
/// são dado do projeto.
///
/// Campos sem dado correspondente ficam **literais**: `[nome do projeto]` sem projeto ativo
/// continua visível pedindo preenchimento, em vez de virar um buraco silencioso no prompt.
pub fn render(template: &str, transcript: &str, ctx: &ProjectContext) -> String {
    let mut out = if template.contains(SPEECH_MARKER) {
        template.replace(SPEECH_MARKER, transcript.trim())
    } else {
        // Modelo do usuário salvo a partir de um prompt pronto não tem marcador. Anexar a fala
        // ao final é preferível a descartá-la em silêncio.
        format!("{}\n\n{}", template.trim_end(), transcript.trim())
    };

    for field in AUTO_FIELDS {
        let value = match *field {
            "[nome do projeto]" => ctx.name.trim().to_string(),
            "[comando de teste]" => join_commands(&ctx.test_commands),
            _ => String::new(),
        };
        if !value.is_empty() {
            out = out.replace(field, &value);
        }
    }

    out
}

/// `projects.test_commands` guarda um comando por linha; dentro de uma frase ("Rodar: X.") a
/// quebra de linha estragaria o texto, então vira lista separada por vírgula.
fn join_commands(raw: &str) -> String {
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# Erro agora (colar stack trace)\n\n\
        > Modo: `bug_fix` · Área: depuração\n\
        > Uso: quebrou agora, tenho a mensagem de erro na tela.\n\n\
        ---\n\n\
        Estou com um erro no projeto **[nome do projeto]**.\n\n\
        <<SUA FALA>>\n\n\
        Rodar: [comando de teste].\n";

    fn ctx() -> ProjectContext {
        ProjectContext {
            name: "CodeVoice".into(),
            test_commands: "cargo test\nnpm run test".into(),
            ..Default::default()
        }
    }

    #[test]
    fn parses_header_into_metadata() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(parsed.name, "Erro agora (colar stack trace)");
        assert_eq!(parsed.mode, "bug_fix");
        assert_eq!(
            parsed.description,
            "quebrou agora, tenho a mensagem de erro na tela."
        );
    }

    /// O ponto principal do parser: o metadado não pode vazar para o corpo do prompt.
    #[test]
    fn body_excludes_the_header() {
        let parsed = parse(SAMPLE).unwrap();
        assert!(!parsed.content.contains("> Modo:"));
        assert!(!parsed.content.contains("> Uso:"));
        assert!(!parsed.content.starts_with('#'));
        assert!(parsed.content.starts_with("Estou com um erro"));
    }

    #[test]
    fn rejects_file_without_separator() {
        assert!(parse("# Só título\n\n> Modo: `quick`\n").is_none());
    }

    #[test]
    fn rejects_unknown_mode() {
        let bad = SAMPLE.replace("`bug_fix`", "`modo_inventado`");
        assert!(parse(&bad).is_none());
    }

    #[test]
    fn rejects_empty_body() {
        assert!(parse("# T\n\n> Modo: `quick`\n\n---\n\n   \n").is_none());
    }

    #[test]
    fn category_comes_from_the_folder() {
        assert_eq!(category_of("depuracao/erro-agora"), "depuracao");
        assert_eq!(
            category_of("negocios-produto/analise-concorrencia"),
            "negocios-produto"
        );
    }

    #[test]
    fn render_replaces_speech_and_project_data() {
        let out = render(&parse(SAMPLE).unwrap().content, "o login quebrou", &ctx());
        assert!(out.contains("o login quebrou"));
        assert!(!out.contains(SPEECH_MARKER));
        assert!(out.contains("**CodeVoice**"));
        assert!(out.contains("Rodar: cargo test, npm run test."));
    }

    /// Sem projeto ativo o literal tem que sobreviver: um `[nome do projeto]` visível é um
    /// lembrete de preencher; um vazio silencioso vira um prompt pela metade.
    #[test]
    fn render_keeps_literals_when_there_is_no_project_data() {
        let out = render(
            &parse(SAMPLE).unwrap().content,
            "algo",
            &ProjectContext::default(),
        );
        assert!(out.contains("[nome do projeto]"));
        assert!(out.contains("[comando de teste]"));
    }

    #[test]
    fn render_appends_speech_when_the_template_has_no_marker() {
        let out = render("# Modelo do usuário\n\nCorpo fixo.", "minha fala", &ctx());
        assert!(out.contains("Corpo fixo."));
        assert!(out.ends_with("minha fala"));
    }

    #[test]
    fn render_leaves_decision_fields_untouched() {
        let out = render("Repetir [N] vezes em [período].", "x", &ctx());
        assert!(out.contains("[N]"));
        assert!(out.contains("[período]"));
    }

    #[test]
    fn the_library_is_embedded_and_complete() {
        assert_eq!(
            EMBEDDED_TEMPLATES.len(),
            117,
            "a biblioteca versionada tem 117 modelos (o README da raiz não conta)"
        );
    }

    /// Se um modelo novo entrar com cabeçalho torto, ele sumiria da biblioteca em silêncio.
    #[test]
    fn every_embedded_template_parses() {
        for (slug, markdown) in EMBEDDED_TEMPLATES {
            let parsed = parse(markdown).unwrap_or_else(|| panic!("{slug} não parseou"));
            assert!(!parsed.name.trim().is_empty(), "{slug} sem título");
            assert!(
                !parsed.description.trim().is_empty(),
                "{slug} sem linha `> Uso:`"
            );
            assert!(
                PromptMode::from_db_str(&parsed.mode).is_some(),
                "{slug} com modo inválido: {}",
                parsed.mode
            );
            assert!(
                !parsed.content.contains("> Modo:"),
                "{slug} deixou metadado vazar para o corpo"
            );
            assert!(
                parsed.content.contains(SPEECH_MARKER),
                "{slug} não tem onde encaixar a fala"
            );
        }
    }

    #[test]
    fn builtins_cover_the_eighteen_categories() {
        let items = builtins();
        assert_eq!(items.len(), 117);

        let mut categories: Vec<_> = items.iter().map(|t| t.category.clone()).collect();
        categories.sort();
        categories.dedup();
        assert_eq!(
            categories.len(),
            18,
            "categorias encontradas: {categories:?}"
        );
        assert!(categories.contains(&"depuracao".to_string()));
    }

    #[test]
    fn builtin_slugs_are_unique() {
        let items = builtins();
        let mut slugs: Vec<_> = items.iter().map(|t| t.slug.clone()).collect();
        slugs.sort();
        let total = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), total, "slug duplicado na biblioteca");
    }
}
