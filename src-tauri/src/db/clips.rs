use rusqlite::{params, Connection, OptionalExtension};
use tauri::AppHandle;

use crate::models::{Clip, ClipKind, CreateClipPayload, Folder};

use super::{connection, schema, seed, DbResult};

pub fn list_folders(app: &AppHandle) -> DbResult<Vec<Folder>> {
    let conn = open_ready_connection(app)?;
    query_folders(&conn)
}

pub fn list_clips(app: &AppHandle) -> DbResult<Vec<Clip>> {
    let conn = open_ready_connection(app)?;
    query_clips(&conn)
}

pub fn create_clip(app: &AppHandle, payload: CreateClipPayload) -> DbResult<Clip> {
    let conn = open_ready_connection(app)?;
    let title = payload.title.trim();
    let content = payload.content.trim();
    let source = payload.source.trim();

    if title.is_empty() {
        return Err("Clip title cannot be empty.".into());
    }

    if content.is_empty() {
        return Err("Clip content cannot be empty.".into());
    }

    let folder_id = if payload.folder_id.trim().is_empty() {
        "inbox"
    } else {
        payload.folder_id.trim()
    };

    ensure_folder_exists(&conn, folder_id)?;

    conn.execute(
        "
        insert into clips (folder_id, title, content, source, time_label, pinned, kind)
        values (?1, ?2, ?3, ?4, 'now', 0, ?5)
        ",
        params![
            folder_id,
            title,
            content,
            if source.is_empty() { "Manual" } else { source },
            payload.kind.as_str()
        ],
    )
    .map_err(|error| format!("Failed to create clip: {error}"))?;

    let id = conn.last_insert_rowid() as u32;
    query_clip_by_id(&conn, id)
}

pub fn delete_clip(app: &AppHandle, id: u32) -> DbResult<()> {
    let conn = open_ready_connection(app)?;
    let changed = conn
        .execute("delete from clips where id = ?1", params![id])
        .map_err(|error| format!("Failed to delete clip: {error}"))?;

    if changed == 0 {
        return Err(format!("Clip {id} was not found."));
    }

    Ok(())
}

pub fn toggle_clip_pinned(app: &AppHandle, id: u32) -> DbResult<Clip> {
    let conn = open_ready_connection(app)?;
    let changed = conn
        .execute(
            "
            update clips
            set pinned = case pinned when 0 then 1 else 0 end
            where id = ?1
            ",
            params![id],
        )
        .map_err(|error| format!("Failed to toggle clip pin: {error}"))?;

    if changed == 0 {
        return Err(format!("Clip {id} was not found."));
    }

    query_clip_by_id(&conn, id)
}

fn open_ready_connection(app: &AppHandle) -> DbResult<Connection> {
    let conn = connection::open(app)?;
    schema::ensure(&conn)?;
    seed::ensure(&conn)?;
    Ok(conn)
}

fn query_folders(conn: &Connection) -> DbResult<Vec<Folder>> {
    let mut stmt = conn
        .prepare(
            "
            select
                folders.id,
                folders.name,
                folders.hotkey,
                folders.color,
                case
                    when folders.id = 'inbox' then (select count(*) from clips)
                    else count(clips.id)
                end as clip_count
            from folders
            left join clips on clips.folder_id = folders.id
            group by folders.id
            order by folders.sort_order asc
            ",
        )
        .map_err(|error| format!("Failed to prepare folder query: {error}"))?;

    let folders = stmt
        .query_map([], |row| {
            let count: i64 = row.get(4)?;

            Ok(Folder {
                id: row.get(0)?,
                name: row.get(1)?,
                hotkey: row.get(2)?,
                color: row.get(3)?,
                count: count as u32,
            })
        })
        .map_err(|error| format!("Failed to query folders: {error}"))?;

    collect_rows(folders, "folder")
}

fn query_clips(conn: &Connection) -> DbResult<Vec<Clip>> {
    let mut stmt = conn
        .prepare(
            "
            select id, folder_id, title, content, source, time_label, pinned, kind
            from clips
            order by pinned desc, id desc
            ",
        )
        .map_err(|error| format!("Failed to prepare clip query: {error}"))?;

    let clips = stmt
        .query_map([], |row| {
            let kind: String = row.get(7)?;

            Ok(Clip {
                id: row.get::<_, i64>(0)? as u32,
                folder_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                source: row.get(4)?,
                time: row.get(5)?,
                pinned: row.get(6)?,
                kind: ClipKind::from_str(&kind),
            })
        })
        .map_err(|error| format!("Failed to query clips: {error}"))?;

    collect_rows(clips, "clip")
}

fn query_clip_by_id(conn: &Connection, id: u32) -> DbResult<Clip> {
    conn.query_row(
        "
        select id, folder_id, title, content, source, time_label, pinned, kind
        from clips
        where id = ?1
        ",
        params![id],
        |row| {
            let kind: String = row.get(7)?;

            Ok(Clip {
                id: row.get::<_, i64>(0)? as u32,
                folder_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                source: row.get(4)?,
                time: row.get(5)?,
                pinned: row.get(6)?,
                kind: ClipKind::from_str(&kind),
            })
        },
    )
    .optional()
    .map_err(|error| format!("Failed to query clip {id}: {error}"))?
    .ok_or_else(|| format!("Clip {id} was not found."))
}

fn ensure_folder_exists(conn: &Connection, folder_id: &str) -> DbResult<()> {
    let exists = conn
        .query_row(
            "select exists(select 1 from folders where id = ?1)",
            params![folder_id],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|error| format!("Failed to validate folder: {error}"))?;

    if exists {
        Ok(())
    } else {
        Err(format!("Folder '{folder_id}' was not found."))
    }
}

fn collect_rows<T>(
    rows: impl Iterator<Item = rusqlite::Result<T>>,
    label: &'static str,
) -> DbResult<Vec<T>> {
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| format!("Failed to read {label} row: {error}"))
}
