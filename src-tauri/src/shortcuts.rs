use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

const QUICK_ACCESS_SHORTCUT: &str = "ctrl+shift+space";
const QUICK_ACCESS_OPENED_EVENT: &str = "quick-access-opened";

pub fn register(app: &AppHandle) {
    let plugin = match tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts([QUICK_ACCESS_SHORTCUT])
    {
        Ok(builder) => builder
            .with_handler(|app, shortcut, event| {
                if event.state == ShortcutState::Pressed
                    && shortcut.matches(Modifiers::CONTROL | Modifiers::SHIFT, Code::Space)
                {
                    let _ = focus_main_window(app);
                }
            })
            .build(),
        Err(error) => {
            eprintln!("Failed to parse global shortcut {QUICK_ACCESS_SHORTCUT}: {error}");
            return;
        }
    };

    if let Err(error) = app.plugin(plugin) {
        eprintln!("Failed to register global shortcut {QUICK_ACCESS_SHORTCUT}: {error}");
    }
}

fn focus_main_window(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        window.show()?;
        window.unminimize()?;
        window.set_focus()?;
        window.emit(QUICK_ACCESS_OPENED_EVENT, ())?;
    }

    Ok(())
}
