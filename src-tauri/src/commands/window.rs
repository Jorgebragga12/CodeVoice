use tauri::AppHandle;

use crate::hotkey;
use crate::settings::SettingsRepo;

/// Esconde a janela compacta de gravação. Chamado pela própria janela ao terminar/cancelar.
#[tauri::command]
#[specta::specta]
pub fn hide_recorder_window(app: AppHandle) -> Result<(), String> {
    hotkey::hide_recorder_window(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn show_recorder_window(app: AppHandle) -> Result<(), String> {
    hotkey::show_recorder_window(&app)
}

/// Troca o atalho global em tempo real: desregistra o antigo, tenta o novo e, se o novo estiver
/// tomado por outro programa, **restaura o anterior** para o usuário não ficar sem atalho
/// nenhum por causa de uma escolha inválida.
#[tauri::command]
#[specta::specta]
pub fn update_hotkey(
    app: AppHandle,
    settings: tauri::State<'_, SettingsRepo>,
    hotkey_combo: String,
) -> Result<(), String> {
    let mut current = settings
        .get_recording_settings()
        .map_err(|e| e.to_string())?;

    let previous = current.hotkey.clone();
    hotkey::unregister(&app, &previous);

    if let Err(err) = hotkey::register(&app, &hotkey_combo) {
        // Melhor esforço: se nem o antigo voltar, a UI ainda funciona pelos botões.
        let _ = hotkey::register(&app, &previous);
        return Err(err);
    }

    current.hotkey = hotkey_combo;
    settings
        .save_recording_settings(&current)
        .map_err(|e| e.to_string())
}
