use rusqlite::Connection;

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
            source text not null,
            time_label text not null,
            pinned integer not null default 0,
            kind text not null
        );

        create index if not exists idx_clips_folder_id on clips(folder_id);
        create index if not exists idx_clips_pinned on clips(pinned);
        ",
    )
    .map_err(|error| format!("Failed to ensure database schema: {error}"))
}
