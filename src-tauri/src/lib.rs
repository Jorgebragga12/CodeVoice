use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tauri_specta::{collect_commands, Builder as SpectaBuilder};

pub mod audio;
pub mod commands;
pub mod domain;
pub mod hotkey;
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
        commands::recording::list_audio_devices,
        commands::recording::get_recording_settings,
        commands::recording::save_recording_settings,
        commands::recording::recording_status,
        commands::recording::set_active_project,
        commands::recording::start_recording,
        commands::recording::stop_recording,
        commands::recording::cancel_recording,
        commands::window::show_recorder_window,
        commands::window::hide_recorder_window,
        commands::window::update_hotkey,
        commands::transcription::list_whisper_models,
        commands::transcription::transcription_status,
        commands::transcription::download_model,
        commands::transcription::transcribe_recording,
        commands::promptgen::list_prompt_modes,
        commands::promptgen::claude_cli_available,
        commands::promptgen::generate_prompt,
        commands::promptgen::refine_prompt,
        commands::promptgen::update_prompt_content,
        commands::templates::list_template_categories,
        commands::templates::list_prompt_templates,
        commands::templates::save_prompt_as_template,
        commands::templates::delete_prompt_template,
        commands::templates::generate_prompt_from_template,
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
            app.manage(storage::HistoryRepo::new(pool.clone()));
            app.manage(storage::RecordingRepo::new(pool.clone()));
            app.manage(storage::TranscriptionRepo::new(pool.clone()));
            app.manage(storage::PromptRepo::new(pool.clone()));
            app.manage(commands::transcription::AppDataDir(app_data_dir.clone()));

            // A biblioteca de modelos vive no binário; o banco é só o índice consultável dela.
            // Reescrever as linhas `builtin` a cada startup mantém as duas em sincronia após um
            // update do app (inclusive quando um modelo é renomeado ou removido), sem tocar nos
            // modelos que o usuário salvou.
            let template_repo = storage::PromptTemplateRepo::new(pool.clone());
            match template_repo.replace_builtins(&promptgen::library::builtins()) {
                Ok(count) => log::info!("biblioteca de modelos sincronizada: {count} modelos"),
                // Falhar aqui não justifica derrubar o app: o resto do fluxo (gravar,
                // transcrever, gerar) não depende da biblioteca.
                Err(err) => log::error!("falha ao sincronizar a biblioteca de modelos: {err}"),
            }
            app.manage(template_repo);

            let settings = settings::SettingsRepo::new(pool);
            let recording_settings = settings.get_recording_settings().unwrap_or_default();
            app.manage(settings);

            // WAVs sobrando aqui são de uma execução que morreu antes de processá-los: voz do
            // usuário que já deveria ter sido apagada (SECURITY-MODEL §2).
            let tmp_dir = audio::tmp_files::tmp_dir(&app_data_dir);
            let removed = audio::tmp_files::cleanup_orphans(&tmp_dir);
            if removed > 0 {
                log::info!("removidos {removed} arquivo(s) de áudio órfãos de sessões anteriores");
            }
            app.manage(commands::recording::RecorderHandle::new(tmp_dir));

            // Conflito de atalho não é fatal: o app segue usável pelos botões e o usuário troca
            // a combinação nas Configurações.
            if let Err(err) = hotkey::register(app.handle(), &recording_settings.hotkey) {
                log::warn!("{err}");
            }

            spawn_recording_limit_watchdog(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Vigia o teto de 10 min por gravação (PRODUCT-SPEC §5.2) no backend, não na UI.
///
/// Depender do frontend para isso deixaria o limite furado exatamente nos casos em que ele mais
/// importa: janela de gravação fechada, app minimizado na bandeja, ou webview travada — cenários
/// em que uma gravação esquecida encheria o disco em silêncio.
fn spawn_recording_limit_watchdog(app: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let reached = app
            .try_state::<commands::recording::RecorderHandle>()
            .and_then(|handle| handle.inner.lock().ok().map(|rec| rec.reached_limit()))
            .unwrap_or(false);

        if reached {
            log::info!("limite de gravação atingido; encerrando automaticamente");
            match commands::recording::do_stop(&app) {
                Ok(_) => {
                    let _ = tauri::Emitter::emit(&app, "recording:limit-reached", ());
                }
                Err(err) => log::error!("falha ao encerrar gravação no limite: {err}"),
            }
        }
    });
}
