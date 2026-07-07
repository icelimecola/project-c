pub mod monitor;

use crate::models::{ClipKind, CreateClipPayload};

pub fn read_text() -> Result<String, String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|error| format!("Failed to access system clipboard: {error}"))?;

    clipboard
        .get_text()
        .map_err(|error| format!("Failed to read text from system clipboard: {error}"))
}

pub fn payload_from_text(
    folder_id: impl Into<String>,
    text: &str,
    source: impl Into<String>,
) -> CreateClipPayload {
    let trimmed_text = text.trim();

    CreateClipPayload {
        folder_id: folder_id.into(),
        title: title_from_text(trimmed_text),
        content: trimmed_text.into(),
        source: source.into(),
        source_app: None,
        mime_type: None,
        kind: kind_from_text(trimmed_text),
    }
}

fn title_from_text(text: &str) -> String {
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

fn kind_from_text(text: &str) -> ClipKind {
    let trimmed = text.trim();

    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        ClipKind::Link
    } else if trimmed.contains('\n') && (trimmed.contains('{') || trimmed.contains(';')) {
        ClipKind::Code
    } else {
        ClipKind::Text
    }
}
