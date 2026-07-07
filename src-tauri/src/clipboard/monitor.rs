use std::{thread, time::Duration};

use tauri::{AppHandle, Emitter};

use crate::{db, models::Clip};

const CLIPBOARD_POLL_INTERVAL: Duration = Duration::from_millis(1500);
const CLIPS_CHANGED_EVENT: &str = "clips-changed";

pub fn start(app: AppHandle) {
    thread::spawn(move || {
        let mut last_hash: Option<String> = None;

        loop {
            if !monitor_enabled(&app) {
                thread::sleep(CLIPBOARD_POLL_INTERVAL);
                continue;
            }

            if let Some(clip) = capture_next_text_clip(&app, &mut last_hash) {
                if let Err(error) = app.emit(CLIPS_CHANGED_EVENT, clip) {
                    eprintln!("Failed to emit {CLIPS_CHANGED_EVENT}: {error}");
                }
            }

            thread::sleep(CLIPBOARD_POLL_INTERVAL);
        }
    });
}

fn monitor_enabled(app: &AppHandle) -> bool {
    match db::settings::clipboard_monitor_enabled(app) {
        Ok(enabled) => enabled,
        Err(error) => {
            eprintln!("Failed to read clipboard monitor setting: {error}");
            false
        }
    }
}

fn capture_next_text_clip(app: &AppHandle, last_hash: &mut Option<String>) -> Option<Clip> {
    let text = match super::read_text() {
        Ok(text) => text,
        Err(_) => return None,
    };
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return None;
    }

    let next_hash = db::content_hash(trimmed_text);
    if last_hash.as_deref() == Some(next_hash.as_str()) {
        return None;
    }

    *last_hash = Some(next_hash);

    match db::clips::create_clip(
        app,
        super::payload_from_text("inbox", trimmed_text, "Clipboard Monitor"),
    ) {
        Ok(clip) => Some(clip),
        Err(error) => {
            eprintln!("Failed to capture clipboard text: {error}");
            None
        }
    }
}
