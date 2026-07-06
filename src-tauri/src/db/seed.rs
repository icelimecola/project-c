use rusqlite::{params, Connection};

use crate::models::ClipKind;

use super::{content, DbResult};

struct SeedFolder {
    id: &'static str,
    name: &'static str,
    hotkey: &'static str,
    color: &'static str,
    sort_order: i64,
}

struct SeedClip {
    id: i64,
    folder_id: &'static str,
    title: &'static str,
    content: &'static str,
    source: &'static str,
    time: &'static str,
    pinned: bool,
    kind: ClipKind,
}

pub fn ensure(conn: &Connection) -> DbResult<()> {
    let folder_count: i64 = conn
        .query_row("select count(*) from folders", [], |row| row.get(0))
        .map_err(|error| format!("Failed to count folders: {error}"))?;

    if folder_count > 0 {
        return Ok(());
    }

    let folders = seed_folders();
    for folder in folders {
        conn.execute(
            "
            insert into folders (id, name, hotkey, color, sort_order)
            values (?1, ?2, ?3, ?4, ?5)
            ",
            params![
                folder.id,
                folder.name,
                folder.hotkey,
                folder.color,
                folder.sort_order
            ],
        )
        .map_err(|error| format!("Failed to seed folder '{}': {error}", folder.id))?;
    }

    let clips = seed_clips();
    for clip in clips {
        conn.execute(
            "
            insert into clips (
                id,
                folder_id,
                title,
                content,
                content_hash,
                source,
                source_app,
                time_label,
                created_at,
                updated_at,
                last_used_at,
                mime_type,
                deleted_at,
                pinned,
                kind
            )
            values (?1, ?2, ?3, ?4, ?5, ?6, null, ?7, datetime('now'), datetime('now'), null, ?8, null, ?9, ?10)
            ",
            params![
                clip.id,
                clip.folder_id,
                clip.title,
                clip.content,
                content::content_hash(clip.content),
                clip.source,
                clip.time,
                content::mime_type_for_kind(&clip.kind),
                clip.pinned,
                clip.kind.as_str()
            ],
        )
        .map_err(|error| format!("Failed to seed clip '{}': {error}", clip.title))?;
    }

    Ok(())
}

fn seed_folders() -> [SeedFolder; 5] {
    [
        SeedFolder {
            id: "inbox",
            name: "Inbox",
            hotkey: "Ctrl Alt V",
            color: "#3a8f74",
            sort_order: 0,
        },
        SeedFolder {
            id: "code",
            name: "Code",
            hotkey: "Ctrl Alt 1",
            color: "#4d6fb3",
            sort_order: 1,
        },
        SeedFolder {
            id: "links",
            name: "Links",
            hotkey: "Ctrl Alt 2",
            color: "#c45f42",
            sort_order: 2,
        },
        SeedFolder {
            id: "writing",
            name: "Writing",
            hotkey: "Ctrl Alt 3",
            color: "#8a6f2a",
            sort_order: 3,
        },
        SeedFolder {
            id: "support",
            name: "Support",
            hotkey: "Ctrl Alt 4",
            color: "#7c5c9f",
            sort_order: 4,
        },
    ]
}

fn seed_clips() -> [SeedClip; 7] {
    [
        SeedClip {
            id: 1,
            folder_id: "inbox",
            title: "SQLite migration note",
            content: "Keep clip content local by default. Store text in SQLite, index searchable text with FTS5, and keep binary payloads in an app-managed data folder.",
            source: "Notes",
            time: "2m",
            pinned: true,
            kind: ClipKind::Text,
        },
        SeedClip {
            id: 2,
            folder_id: "code",
            title: "Tauri command shape",
            content: "#[tauri::command]\nfn list_clips(folder_id: String, query: Option<String>) -> Vec<ClipSummary> {\n    // read from SQLite and return summaries\n}",
            source: "Cursor",
            time: "11m",
            pinned: true,
            kind: ClipKind::Code,
        },
        SeedClip {
            id: 3,
            folder_id: "links",
            title: "Tauri architecture",
            content: "https://tauri.app/concept/architecture/",
            source: "Arc",
            time: "24m",
            pinned: false,
            kind: ClipKind::Link,
        },
        SeedClip {
            id: 4,
            folder_id: "writing",
            title: "Project positioning",
            content: "A keyboard-first clipboard manager for people who live in folders, snippets, and repeated context switching.",
            source: "ChatGPT",
            time: "36m",
            pinned: true,
            kind: ClipKind::Text,
        },
        SeedClip {
            id: 5,
            folder_id: "support",
            title: "Reply template",
            content: "Thanks for the report. I reproduced the issue and will ship a fix in the next patch. For now, the workaround is to disable direct paste and use copy-only mode.",
            source: "Mail",
            time: "1h",
            pinned: false,
            kind: ClipKind::Text,
        },
        SeedClip {
            id: 6,
            folder_id: "code",
            title: "Search query",
            content: "select id, title, content_text from clips where folder_id = ? order by pinned desc, used_at desc limit 80;",
            source: "TablePlus",
            time: "2h",
            pinned: false,
            kind: ClipKind::Code,
        },
        SeedClip {
            id: 7,
            folder_id: "inbox",
            title: "Privacy baseline",
            content: "No account, no cloud sync, no telemetry. The app database and attachments should live in one user-visible local data location.",
            source: "Obsidian",
            time: "3h",
            pinned: false,
            kind: ClipKind::Text,
        },
    ]
}
