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
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|error| format!("Failed to access system clipboard: {error}"))?;
    let text = clipboard
        .get_text()
        .map_err(|error| format!("Failed to read text from system clipboard: {error}"))?;
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return Err("Clipboard text is empty.".into());
    }

    crate::db::clips::create_clip(
        &app,
        CreateClipPayload {
            folder_id: folder_id.unwrap_or_else(|| "inbox".into()),
            title: title_from_clipboard_text(trimmed_text),
            content: trimmed_text.into(),
            source: "Clipboard".into(),
            kind: kind_from_clipboard_text(trimmed_text),
        },
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

fn title_from_clipboard_text(text: &str) -> String {
    let first_line = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("Clipboard text")
        .trim();
    let title: String = first_line.chars().take(48).collect();

    if first_line.chars().count() > 48 {
        format!("{title}...")
    } else {
        title
    }
}

fn kind_from_clipboard_text(text: &str) -> crate::models::ClipKind {
    let trimmed = text.trim();

    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        crate::models::ClipKind::Link
    } else if trimmed.contains('\n') && (trimmed.contains('{') || trimmed.contains(';')) {
        crate::models::ClipKind::Code
    } else {
        crate::models::ClipKind::Text
    }
}
