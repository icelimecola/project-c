use crate::models::{Clip, Folder};

#[tauri::command]
pub fn list_folders(app: tauri::AppHandle) -> Result<Vec<Folder>, String> {
    crate::db::clips::list_folders(&app)
}

#[tauri::command]
pub fn list_clips(app: tauri::AppHandle) -> Result<Vec<Clip>, String> {
    crate::db::clips::list_clips(&app)
}
