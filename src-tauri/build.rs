use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    embed_prompt_library();
    tauri_build::build()
}

/// Embute a biblioteca de modelos (`templates/**/*.md`) no binário como uma lista de
/// `(slug, conteúdo)`.
///
/// Ler os arquivos em runtime exigiria empacotá-los como resource do Tauri e resolver o
/// diretório de recursos — mais superfície de fs para o que é, na prática, conteúdo estático que
/// não muda entre execuções. `include_str!` resolve tudo em tempo de compilação, sem dependência
/// nova e sem nenhum acesso a disco depois do build.
fn embed_prompt_library() {
    let templates_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("templates");
    println!("cargo:rerun-if-changed={}", templates_dir.display());

    let mut files = Vec::new();
    collect_markdown(&templates_dir, &templates_dir, &mut files);
    files.sort();

    let mut out = String::from(
        "// Gerado por build.rs a partir de templates/ — não editar à mão.\n\
         pub const EMBEDDED_TEMPLATES: &[(&str, &str)] = &[\n",
    );
    for (slug, path) in &files {
        // `rerun-if-changed` no diretório não detecta a edição de um arquivo já existente em
        // todos os sistemas; registrar cada arquivo garante o rebuild ao mexer num modelo.
        println!("cargo:rerun-if-changed={}", path.display());
        writeln!(
            out,
            "    ({:?}, include_str!({:?})),",
            slug,
            path.display().to_string()
        )
        .expect("falha ao montar a lista de modelos embutidos");
    }
    out.push_str("];\n");

    let dest = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR ausente"))
        .join("embedded_templates.rs");
    std::fs::write(&dest, out).expect("falha ao escrever a biblioteca de modelos embutida");
}

/// Percorre as pastas de categoria coletando `(categoria/arquivo, caminho)`.
///
/// O `README.md` da raiz é o índice da biblioteca para humanos, não um modelo — só entram
/// arquivos dentro de uma subpasta de categoria.
fn collect_markdown(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(root, &path, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        let slug = relative
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");
        if !slug.contains('/') {
            continue; // arquivo solto na raiz (README.md) — não é modelo
        }
        out.push((slug, path));
    }
}
