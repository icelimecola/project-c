use rusqlite::{params, Connection, OptionalExtension, Row};
use tauri::AppHandle;

use crate::models::{Clip, ClipKind, CreateClipPayload, Folder};

use super::{connection, content, schema, seed, DbResult};

pub fn list_folders(app: &AppHandle) -> DbResult<Vec<Folder>> {
    let conn = open_ready_connection(app)?;
    query_folders(&conn)
}

pub fn list_clips(app: &AppHandle) -> DbResult<Vec<Clip>> {
    let conn = open_ready_connection(app)?;
    query_clips(&conn)
}

pub fn search_clips(
    app: &AppHandle,
    query: Option<String>,
    folder_id: Option<String>,
    kind: Option<ClipKind>,
) -> DbResult<Vec<Clip>> {
    let conn = open_ready_connection(app)?;
    query_searched_clips(&conn, query, folder_id, kind)
}

pub fn create_clip(app: &AppHandle, payload: CreateClipPayload) -> DbResult<Clip> {
    let conn = open_ready_connection(app)?;
    let title = payload.title.trim();
    let content = payload.content.trim();
    let source = payload.source.trim();
    let source_app = payload
        .source_app
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let mime_type = payload
        .mime_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| content::mime_type_for_kind(&payload.kind));

    if title.is_empty() {
        return Err("Clip title cannot be empty.".into());
    }

    if content.is_empty() {
        return Err("Clip content cannot be empty.".into());
    }

    let content_hash = content::content_hash(content);

    if let Some(existing_clip) = find_active_clip_by_hash(&conn, &content_hash)? {
        touch_clip(&conn, existing_clip.id)?;
        return query_clip_by_id(&conn, existing_clip.id);
    }

    let folder_id = if payload.folder_id.trim().is_empty() {
        "inbox"
    } else {
        payload.folder_id.trim()
    };

    ensure_folder_exists(&conn, folder_id)?;

    conn.execute(
        "
        insert into clips (
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
        values (?1, ?2, ?3, ?4, ?5, ?6, 'now', datetime('now'), datetime('now'), null, ?7, null, 0, ?8)
        ",
        params![
            folder_id,
            title,
            content,
            content_hash,
            if source.is_empty() { "Manual" } else { source },
            source_app,
            mime_type,
            payload.kind.as_str()
        ],
    )
    .map_err(|error| format!("Failed to create clip: {error}"))?;

    let id = conn.last_insert_rowid() as u32;
    query_clip_by_id(&conn, id)
}

fn find_active_clip_by_hash(conn: &Connection, content_hash: &str) -> DbResult<Option<Clip>> {
    conn.query_row(
        "
        select
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
        from clips
        where content_hash = ?1 and deleted_at is null
        order by datetime(created_at) desc, id desc
        limit 1
        ",
        params![content_hash],
        row_to_clip,
    )
    .optional()
    .map_err(|error| format!("Failed to query duplicate clip: {error}"))
}

fn touch_clip(conn: &Connection, id: u32) -> DbResult<()> {
    conn.execute(
        "
        update clips
        set
            last_used_at = datetime('now'),
            updated_at = datetime('now')
        where id = ?1 and deleted_at is null
        ",
        params![id],
    )
    .map_err(|error| format!("Failed to update duplicate clip usage: {error}"))?;

    Ok(())
}

pub fn delete_clip(app: &AppHandle, id: u32) -> DbResult<()> {
    let conn = open_ready_connection(app)?;
    let changed = conn
        .execute(
            "
            update clips
            set deleted_at = datetime('now'), updated_at = datetime('now')
            where id = ?1 and deleted_at is null
            ",
            params![id],
        )
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
            set
                pinned = case pinned when 0 then 1 else 0 end,
                updated_at = datetime('now')
            where id = ?1 and deleted_at is null
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
                    when folders.id = 'inbox' then (
                        select count(*) from clips where deleted_at is null
                    )
                    else count(clips.id)
                end as clip_count
            from folders
            left join clips on clips.folder_id = folders.id and clips.deleted_at is null
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
            select
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
            from clips
            where deleted_at is null
            order by pinned desc, datetime(created_at) desc, id desc
            ",
        )
        .map_err(|error| format!("Failed to prepare clip query: {error}"))?;

    let clips = stmt
        .query_map([], |row| row_to_clip(row))
        .map_err(|error| format!("Failed to query clips: {error}"))?;

    collect_rows(clips, "clip")
}

fn query_searched_clips(
    conn: &Connection,
    query: Option<String>,
    folder_id: Option<String>,
    kind: Option<ClipKind>,
) -> DbResult<Vec<Clip>> {
    let folder_filter = folder_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let kind_filter = kind.as_ref().map(ClipKind::as_str);
    let query_pattern = query
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{}%", value.to_lowercase()));

    let mut stmt = conn
        .prepare(
            "
            select
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
            from clips
            where deleted_at is null
                and (?1 is null or ?1 = 'inbox' or folder_id = ?1)
                and (?2 is null or kind = ?2)
                and (
                    ?3 is null
                    or lower(
                        title || ' ' || content || ' ' || source || ' ' || coalesce(source_app, '')
                    ) like ?3
                )
            order by pinned desc, datetime(created_at) desc, id desc
            limit 200
            ",
        )
        .map_err(|error| format!("Failed to prepare clip search query: {error}"))?;

    let clips = stmt
        .query_map(
            params![folder_filter, kind_filter, query_pattern.as_deref()],
            row_to_clip,
        )
        .map_err(|error| format!("Failed to search clips: {error}"))?;

    collect_rows(clips, "clip")
}

fn query_clip_by_id(conn: &Connection, id: u32) -> DbResult<Clip> {
    conn.query_row(
        "
        select
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
        from clips
        where id = ?1 and deleted_at is null
        ",
        params![id],
        row_to_clip,
    )
    .optional()
    .map_err(|error| format!("Failed to query clip {id}: {error}"))?
    .ok_or_else(|| format!("Clip {id} was not found."))
}

fn row_to_clip(row: &Row<'_>) -> rusqlite::Result<Clip> {
    let kind: String = row.get(14)?;

    Ok(Clip {
        id: row.get::<_, i64>(0)? as u32,
        folder_id: row.get(1)?,
        title: row.get(2)?,
        content: row.get(3)?,
        content_hash: row.get(4)?,
        source: row.get(5)?,
        source_app: row.get(6)?,
        time: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
        last_used_at: row.get(10)?,
        mime_type: row.get(11)?,
        deleted_at: row.get(12)?,
        pinned: row.get(13)?,
        kind: ClipKind::from_str(&kind),
    })
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
