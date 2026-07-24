//! Ponta-a-ponta do critério de aceite da Fase 3 (MASTER-PLAN.md §3): cadastrar um projeto
//! real — o próprio CodeVoice — importando arquivos da allowlist (README.md, package.json)
//! com preview, sem tocar no banco de desenvolvimento do Jorge (usa um `app_data_dir`
//! temporário, do mesmo jeito que `init_pool` espera em produção).
//!
//! Nota: este repositório não tem um `CLAUDE.md` na raiz, então a cobertura desse arquivo
//! específico vem do teste unitário `projects::scanner::tests::reads_claude_md_readme_and_package_json`
//! (com um fixture sintético). Aqui validamos o pipeline completo — validação de path, scanner
//! e `ProjectRepo::create` — contra um diretório real, não um fixture criado pelo teste.

use std::path::Path;

use codevoice_lib::domain::NewProject;
use codevoice_lib::projects::scanner;
use codevoice_lib::security::path_validation;
use codevoice_lib::storage::{init_pool, ProjectRepo};

#[test]
fn registers_the_codevoice_repo_itself_via_the_assisted_import_pipeline() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent().expect("src-tauri deveria ter um diretório pai");

    // 1) Validação/canonicalização de path — o mesmo código que o command `create_project` usa.
    let canonical_root = path_validation::validate_project_root(repo_root.to_str().unwrap())
        .expect("a raiz do próprio repositório CodeVoice deveria validar");

    // 2) Importação assistida: preview sem gravar nada no banco.
    let preview = scanner::scan_project(canonical_root.to_str().unwrap())
        .expect("scan do próprio repositório não deveria falhar");

    let file_names: Vec<_> = preview.files.iter().map(|f| f.relative_path.as_str()).collect();
    assert!(file_names.contains(&"README.md"), "README.md deveria ter sido importado");
    assert!(file_names.contains(&"package.json"), "package.json deveria ter sido importado");
    assert!(
        !preview.directories.iter().any(|d| d.contains("node_modules")),
        "node_modules nunca deveria aparecer na listagem de diretórios (denylist)"
    );
    assert!(
        !preview.directories.iter().any(|d| d == ".git"),
        ".git nunca deveria aparecer na listagem de diretórios (denylist)"
    );

    // 3) Cadastro real do projeto usando o caminho já validado — fluxo completo ponta a ponta.
    let temp_app_data_dir = tempfile::tempdir().unwrap();
    let pool = init_pool(temp_app_data_dir.path()).expect("falha ao inicializar banco temporário");
    let repo = ProjectRepo::new(pool);

    let created = repo
        .create(&NewProject {
            name: "CodeVoice".into(),
            path: canonical_root.display().to_string(),
            description: "app de ditado para prompts do Claude Code".into(),
            stack: "Tauri 2, React, SQLite".into(),
            architecture: String::new(),
            dev_commands: "npm run tauri dev".into(),
            test_commands: "npm run test".into(),
            forbidden_tech: String::new(),
            database_info: "SQLite".into(),
            notes: String::new(),
        })
        .expect("cadastro do projeto deveria funcionar de ponta a ponta");

    assert_eq!(created.path, canonical_root.display().to_string());

    let fetched = repo.get(created.id).unwrap().expect("projeto recém-criado deveria existir");
    assert_eq!(fetched.name, "CodeVoice");
}
