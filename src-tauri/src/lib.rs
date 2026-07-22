use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

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
    tauri::Builder::default()
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
