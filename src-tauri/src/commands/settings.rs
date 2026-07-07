use crate::models::AppSettings;

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    crate::db::settings::get_settings(&app)
}

#[tauri::command]
pub fn set_clipboard_monitor_enabled(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<AppSettings, String> {
    crate::db::settings::set_clipboard_monitor_enabled(&app, enabled)
}
