use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
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

/// Mostra (criando na primeira vez) a janela compacta de gravação: sem borda, sempre no topo,
/// sem barra de tarefas (PRODUCT-SPEC §5.1).
pub fn show_recorder_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(RECORDER_WINDOW) {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    WebviewWindowBuilder::new(
        app,
        RECORDER_WINDOW,
        WebviewUrl::App("index.html?window=recorder".into()),
    )
    .title("Gravando — CodeVoice")
    .inner_size(320.0, 132.0)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn hide_recorder_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(RECORDER_WINDOW) {
        let _ = window.hide();
    }
}
