use crate::models::{Clip, ClipKind, Folder};

#[tauri::command]
pub fn list_folders() -> Vec<Folder> {
    vec![
        Folder {
            id: "inbox".into(),
            name: "Inbox".into(),
            hotkey: "Ctrl Alt V".into(),
            count: 42,
            color: "#3a8f74".into(),
        },
        Folder {
            id: "code".into(),
            name: "Code".into(),
            hotkey: "Ctrl Alt 1".into(),
            count: 18,
            color: "#4d6fb3".into(),
        },
        Folder {
            id: "links".into(),
            name: "Links".into(),
            hotkey: "Ctrl Alt 2".into(),
            count: 12,
            color: "#c45f42".into(),
        },
        Folder {
            id: "writing".into(),
            name: "Writing".into(),
            hotkey: "Ctrl Alt 3".into(),
            count: 9,
            color: "#8a6f2a".into(),
        },
        Folder {
            id: "support".into(),
            name: "Support".into(),
            hotkey: "Ctrl Alt 4".into(),
            count: 6,
            color: "#7c5c9f".into(),
        },
    ]
}

#[tauri::command]
pub fn list_clips() -> Vec<Clip> {
    vec![
        Clip {
            id: 1,
            folder_id: "inbox".into(),
            title: "SQLite migration note".into(),
            content: "Keep clip content local by default. Store text in SQLite, index searchable text with FTS5, and keep binary payloads in an app-managed data folder.".into(),
            source: "Notes".into(),
            time: "2m".into(),
            pinned: true,
            kind: ClipKind::Text,
        },
        Clip {
            id: 2,
            folder_id: "code".into(),
            title: "Tauri command shape".into(),
            content: "#[tauri::command]\nfn list_clips(folder_id: String, query: Option<String>) -> Vec<ClipSummary> {\n    // read from SQLite and return summaries\n}".into(),
            source: "Cursor".into(),
            time: "11m".into(),
            pinned: true,
            kind: ClipKind::Code,
        },
        Clip {
            id: 3,
            folder_id: "links".into(),
            title: "Tauri architecture".into(),
            content: "https://tauri.app/concept/architecture/".into(),
            source: "Arc".into(),
            time: "24m".into(),
            pinned: false,
            kind: ClipKind::Link,
        },
        Clip {
            id: 4,
            folder_id: "writing".into(),
            title: "Project positioning".into(),
            content: "A keyboard-first clipboard manager for people who live in folders, snippets, and repeated context switching.".into(),
            source: "ChatGPT".into(),
            time: "36m".into(),
            pinned: true,
            kind: ClipKind::Text,
        },
        Clip {
            id: 5,
            folder_id: "support".into(),
            title: "Reply template".into(),
            content: "Thanks for the report. I reproduced the issue and will ship a fix in the next patch. For now, the workaround is to disable direct paste and use copy-only mode.".into(),
            source: "Mail".into(),
            time: "1h".into(),
            pinned: false,
            kind: ClipKind::Text,
        },
        Clip {
            id: 6,
            folder_id: "code".into(),
            title: "Search query".into(),
            content: "select id, title, content_text from clips where folder_id = ? order by pinned desc, used_at desc limit 80;".into(),
            source: "TablePlus".into(),
            time: "2h".into(),
            pinned: false,
            kind: ClipKind::Code,
        },
        Clip {
            id: 7,
            folder_id: "inbox".into(),
            title: "Privacy baseline".into(),
            content: "No account, no cloud sync, no telemetry. The app database and attachments should live in one user-visible local data location.".into(),
            source: "Obsidian".into(),
            time: "3h".into(),
            pinned: false,
            kind: ClipKind::Text,
        },
    ]
}
