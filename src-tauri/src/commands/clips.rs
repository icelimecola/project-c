use crate::models::{Clip, CreateClipPayload, Folder};

#[tauri::command]
pub fn list_folders(app: tauri::AppHandle) -> Result<Vec<Folder>, String> {
    crate::db::clips::list_folders(&app)
}

#[tauri::command]
pub fn list_clips(app: tauri::AppHandle) -> Result<Vec<Clip>, String> {
    crate::db::clips::list_clips(&app)
}

#[tauri::command]
pub fn create_clip(app: tauri::AppHandle, payload: CreateClipPayload) -> Result<Clip, String> {
    crate::db::clips::create_clip(&app, payload)
}

#[tauri::command]
pub fn capture_clipboard_text(
    app: tauri::AppHandle,
    folder_id: Option<String>,
) -> Result<Clip, String> {
    let text = crate::clipboard::read_text()?;
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return Err("Clipboard text is empty.".into());
    }

    crate::db::clips::create_clip(
        &app,
        crate::clipboard::payload_from_text(
            folder_id.unwrap_or_else(|| "inbox".into()),
            trimmed_text,
            "Clipboard",
        ),
    )
}

#[tauri::command]
pub fn delete_clip(app: tauri::AppHandle, id: u32) -> Result<(), String> {
    crate::db::clips::delete_clip(&app, id)
}

#[tauri::command]
pub fn toggle_clip_pinned(app: tauri::AppHandle, id: u32) -> Result<Clip, String> {
    crate::db::clips::toggle_clip_pinned(&app, id)
}
