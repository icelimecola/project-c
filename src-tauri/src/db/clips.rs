use rusqlite::Connection;
use tauri::AppHandle;

use crate::models::{Clip, ClipKind, Folder};

use super::{connection, schema, seed, DbResult};

pub fn list_folders(app: &AppHandle) -> DbResult<Vec<Folder>> {
    let conn = open_ready_connection(app)?;
    query_folders(&conn)
}

pub fn list_clips(app: &AppHandle) -> DbResult<Vec<Clip>> {
    let conn = open_ready_connection(app)?;
    query_clips(&conn)
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
            order by pinned desc, id asc
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

fn collect_rows<T>(
    rows: impl Iterator<Item = rusqlite::Result<T>>,
    label: &'static str,
) -> DbResult<Vec<T>> {
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| format!("Failed to read {label} row: {error}"))
}
