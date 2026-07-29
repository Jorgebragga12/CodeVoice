//! Imprime um prompt gerado pelo `TemplateGenerator`, para inspeção humana da qualidade da
//! saída (o `cargo test` verifica estrutura, mas não substitui ler o texto).
//!
//! Uso: `cargo run --example show_prompt [modo]` dentro de `src-tauri`.
//! Ex.: `cargo run --example show_prompt technical`

use codevoice_lib::domain::PromptMode;
use codevoice_lib::promptgen::{templates, ProjectContext};

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "technical".into());
    let mode = match arg.as_str() {
        "clean" => PromptMode::CleanTranscript,
        "quick" => PromptMode::Quick,
        "technical" => PromptMode::Technical,
        "feature" => PromptMode::NewFeature,
        "bug" => PromptMode::BugFix,
        "refactor" => PromptMode::Refactor,
        "planning" => PromptMode::Planning,
        "review" => PromptMode::CodeReview,
        "ui" => PromptMode::UiCreation,
        "db" => PromptMode::DbChange,
        other => {
            eprintln!("modo desconhecido: {other}");
            std::process::exit(1);
        }
    };

    let ctx = ProjectContext {
        name: "CodeVoice".into(),
        path: "C:\\Users\\Jorge Braga\\Documents\\CLAUDETE\\CodeVoice".into(),
        description: "App desktop que transforma fala em prompts técnicos".into(),
        stack: "Tauri 2, React 19, TypeScript, Rust, SQLite".into(),
        architecture: "Frontend burro; lógica no Rust atrás de traits".into(),
        dev_commands: "npm run tauri dev".into(),
        test_commands: "cargo test && npm run test".into(),
        forbidden_tech: "jQuery, Electron".into(),
        database_info: "SQLite via rusqlite".into(),
        notes: String::new(),
        rules: vec![
            "Nunca commitar segredos".into(),
            "Rodar lint e testes ao final de cada fase".into(),
        ],
    };

    let transcript = "então assim eu preciso né criar uma tela de login usando React \
                      com validação de email e salvar no banco SQLite tipo assim, \
                      e tem que ter tratamento de erro quando a senha estiver errada";

    println!("{}", templates::generate(mode, transcript, &ctx));
}
