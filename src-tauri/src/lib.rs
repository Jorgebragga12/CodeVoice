use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tauri_specta::{collect_commands, Builder as SpectaBuilder};

pub mod audio;
pub mod commands;
pub mod domain;
pub mod projects;
pub mod promptgen;
pub mod security;
pub mod settings;
pub mod storage;
pub mod terminal;
pub mod transcription;

pub fn run() {
    let specta_builder = SpectaBuilder::<tauri::Wry>::new().commands(collect_commands![
        commands::projects::list_projects,
        commands::projects::get_project,
        commands::projects::create_project,
        commands::projects::update_project,
        commands::projects::delete_project,
        commands::projects::list_project_rules,
        commands::projects::create_project_rule,
        commands::projects::update_project_rule,
        commands::projects::delete_project_rule,
        commands::projects::reorder_project_rules,
        commands::scanner::validate_project_path,
        commands::scanner::preview_project_import,
    ]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(specta_typescript::Typescript::default(), "../src/ipc/bindings.ts")
        .expect("falha ao exportar bindings TypeScript");

    tauri::Builder::default()
        .invoke_handler(specta_builder.invoke_handler())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("codevoice".into()),
                    }),
                ])
                .level(log::LevelFilter::Info)
                .format(|out, message, record| {
                    let redacted = crate::security::log_filter::redact(&message.to_string());
                    out.finish(format_args!(
                        "[{} {}] {}",
                        record.level(),
                        record.target(),
                        redacted
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(move |app| {
            specta_builder.mount_events(app);

            let app_data_dir = app.path().app_data_dir().expect("sem app_data_dir resolvido");
            let pool = storage::init_pool(&app_data_dir).expect("falha ao inicializar o banco");
            app.manage(storage::ProjectRepo::new(pool.clone()));
            app.manage(storage::ProjectRuleRepo::new(pool.clone()));
            app.manage(storage::HistoryRepo::new(pool));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
