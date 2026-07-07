use rusqlite::{params, Connection};

use super::content;

use super::DbResult;

pub fn ensure(conn: &Connection) -> DbResult<()> {
    conn.execute_batch(
        "
        create table if not exists folders (
            id text primary key,
            name text not null,
            hotkey text not null,
            color text not null,
            sort_order integer not null
        );

        create table if not exists clips (
            id integer primary key,
            folder_id text not null references folders(id),
            title text not null,
            content text not null,
            content_hash text not null,
            source text not null,
            source_app text,
            time_label text not null,
            created_at text not null,
            updated_at text not null,
            last_used_at text,
            mime_type text not null default 'text/plain',
            deleted_at text,
            pinned integer not null default 0,
            kind text not null
        );

        create table if not exists settings (
            key text primary key,
            value text not null,
            updated_at text not null
        );

        ",
    )
    .map_err(|error| format!("Failed to ensure database schema: {error}"))?;

    migrate_clips_table(conn)?;
    backfill_clip_metadata(conn)?;
    ensure_clip_fts(conn)?;
    ensure_default_settings(conn)
}

fn migrate_clips_table(conn: &Connection) -> DbResult<()> {
    ensure_column(conn, "content_hash", "text not null default ''")?;
    ensure_column(conn, "source_app", "text")?;
    ensure_column(conn, "created_at", "text not null default ''")?;
    ensure_column(conn, "updated_at", "text not null default ''")?;
    ensure_column(conn, "last_used_at", "text")?;
    ensure_column(conn, "mime_type", "text not null default 'text/plain'")?;
    ensure_column(conn, "deleted_at", "text")?;

    conn.execute_batch(
        "
        create index if not exists idx_clips_folder_id on clips(folder_id);
        create index if not exists idx_clips_content_hash on clips(content_hash);
        create index if not exists idx_clips_deleted_at on clips(deleted_at);
        create index if not exists idx_clips_pinned on clips(pinned);
        ",
    )
    .map_err(|error| format!("Failed to ensure clip indexes: {error}"))
}

fn ensure_column(conn: &Connection, name: &str, definition: &str) -> DbResult<()> {
    if column_exists(conn, name)? {
        return Ok(());
    }

    conn.execute_batch(&format!(
        "alter table clips add column {name} {definition};"
    ))
    .map_err(|error| format!("Failed to add clips.{name}: {error}"))
}

fn column_exists(conn: &Connection, name: &str) -> DbResult<bool> {
    let mut stmt = conn
        .prepare("pragma table_info(clips)")
        .map_err(|error| format!("Failed to inspect clips table: {error}"))?;

    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| format!("Failed to read clips columns: {error}"))?;

    for column in columns {
        if column.map_err(|error| format!("Failed to read clips column: {error}"))? == name {
            return Ok(true);
        }
    }

    Ok(false)
}

fn backfill_clip_metadata(conn: &Connection) -> DbResult<()> {
    conn.execute_batch(
        "
        update clips
        set
            created_at = case when created_at = '' then datetime('now') else created_at end,
            updated_at = case when updated_at = '' then datetime('now') else updated_at end,
            mime_type = case when mime_type = '' then 'text/plain' else mime_type end
        ",
    )
    .map_err(|error| format!("Failed to backfill clip timestamps: {error}"))?;

    let mut stmt = conn
        .prepare("select id, content from clips where content_hash = ''")
        .map_err(|error| format!("Failed to prepare clip hash backfill: {error}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("Failed to query clips for hash backfill: {error}"))?;

    for row in rows {
        let (id, clip_content) =
            row.map_err(|error| format!("Failed to read clip hash backfill row: {error}"))?;
        conn.execute(
            "update clips set content_hash = ?1 where id = ?2",
            params![content::content_hash(&clip_content), id],
        )
        .map_err(|error| format!("Failed to backfill content hash for clip {id}: {error}"))?;
    }

    Ok(())
}

fn ensure_clip_fts(conn: &Connection) -> DbResult<()> {
    let should_rebuild = !table_exists(conn, "clips_fts")?;

    conn.execute_batch(
        "
        create virtual table if not exists clips_fts using fts5(
            title,
            content,
            source,
            source_app,
            content='clips',
            content_rowid='id',
            tokenize='trigram'
        );

        create trigger if not exists clips_ai_fts after insert on clips
        when new.deleted_at is null
        begin
            insert into clips_fts(rowid, title, content, source, source_app)
            values (new.id, new.title, new.content, new.source, new.source_app);
        end;

        create trigger if not exists clips_ad_fts after delete on clips
        begin
            insert into clips_fts(clips_fts, rowid, title, content, source, source_app)
            values ('delete', old.id, old.title, old.content, old.source, old.source_app);
        end;

        create trigger if not exists clips_au_fts
        after update of title, content, source, source_app, deleted_at on clips
        begin
            insert into clips_fts(clips_fts, rowid, title, content, source, source_app)
            values ('delete', old.id, old.title, old.content, old.source, old.source_app);

            insert into clips_fts(rowid, title, content, source, source_app)
            select new.id, new.title, new.content, new.source, new.source_app
            where new.deleted_at is null;
        end;
        ",
    )
    .map_err(|error| format!("Failed to ensure clip FTS schema: {error}"))?;

    if should_rebuild {
        conn.execute("insert into clips_fts(clips_fts) values ('rebuild')", [])
            .map_err(|error| format!("Failed to rebuild clip FTS index: {error}"))?;
    }

    Ok(())
}

fn table_exists(conn: &Connection, name: &str) -> DbResult<bool> {
    conn.query_row(
        "select exists(select 1 from sqlite_master where type in ('table', 'view') and name = ?1)",
        params![name],
        |row| row.get::<_, bool>(0),
    )
    .map_err(|error| format!("Failed to inspect database table {name}: {error}"))
}

fn ensure_default_settings(conn: &Connection) -> DbResult<()> {
    conn.execute(
        "
        insert or ignore into settings (key, value, updated_at)
        values ('clipboard_monitor_enabled', 'true', datetime('now'))
        ",
        [],
    )
    .map_err(|error| format!("Failed to ensure default settings: {error}"))?;

    Ok(())
}
