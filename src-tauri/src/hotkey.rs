use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::commands::recording;

pub const RECORDER_WINDOW: &str = "recorder";

/// Registra o atalho global que alterna a gravação.
///
/// Devolve `Err` quando o atalho já está tomado por outro programa. O chamador **não deve**
/// tratar isso como fatal: o app continua utilizável pelos botões da UI, e o usuário pode
/// escolher outra combinação nas Configurações (PRODUCT-SPEC §5.2).
pub fn register(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let app_for_handler = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            // Só reage ao pressionar. Sem este filtro o toggle dispararia duas vezes por
            // acionamento (uma no press e outra no release), start e stop se anulando.
            if event.state() != ShortcutState::Pressed {
                return;
            }

            let app = app_for_handler.clone();
            // A janela é mostrada antes de abrir o microfone: é o que dá a resposta visual
            // imediata exigida pelo critério de "< 300 ms", já que inicializar o device pode
            // levar algumas dezenas de ms.
            if let Err(err) = show_recorder_window(&app) {
                log::error!("falha ao abrir a janela de gravação: {err}");
            }

            if let Err(err) = recording::do_toggle(&app) {
                log::error!("falha ao alternar a gravação pelo atalho: {err}");
                let _ = tauri::Emitter::emit(&app, "recording:error", err);
            }
        })
        .map_err(|e| format!("não foi possível registrar o atalho \"{shortcut}\": {e}"))
}

pub fn unregister(app: &AppHandle, shortcut: &str) {
    let _ = app.global_shortcut().unregister(shortcut);
}

/// Mostra a janela compacta de gravação (declarada oculta em `tauri.conf.json`, criada no
/// startup). Só exibir/focar — nada de criar em runtime: além de a janela pré-criada abrir
/// instantânea (requisito < 300 ms), criar `WebviewWindow` em runtime com `WebviewUrl::App`
/// não resolve a URL contra o dev server em modo dev (carrega em branco). Deixar o Tauri
/// resolver a URL pela config, como faz com a janela principal, funciona em dev e produção.
pub fn show_recorder_window(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window(RECORDER_WINDOW)
        .ok_or_else(|| "janela de gravação não encontrada".to_string())?;
    window.show().map_err(|e| e.to_string())?;
    let _ = window.set_focus();
    Ok(())
}

pub fn hide_recorder_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(RECORDER_WINDOW) {
        let _ = window.hide();
    }
}
