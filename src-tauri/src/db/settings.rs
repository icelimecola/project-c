use rusqlite::{params, Connection};
use tauri::AppHandle;

use crate::models::AppSettings;

use super::{connection, schema, DbResult};

const CLIPBOARD_MONITOR_ENABLED: &str = "clipboard_monitor_enabled";

pub fn get_settings(app: &AppHandle) -> DbResult<AppSettings> {
    let conn = open_ready_connection(app)?;

    Ok(AppSettings {
        clipboard_monitor_enabled: get_bool(&conn, CLIPBOARD_MONITOR_ENABLED)?,
    })
}

pub fn set_clipboard_monitor_enabled(app: &AppHandle, enabled: bool) -> DbResult<AppSettings> {
    let conn = open_ready_connection(app)?;

    conn.execute(
        "
        insert into settings (key, value, updated_at)
        values (?1, ?2, datetime('now'))
        on conflict(key) do update set
            value = excluded.value,
            updated_at = excluded.updated_at
        ",
        params![CLIPBOARD_MONITOR_ENABLED, bool_to_str(enabled)],
    )
    .map_err(|error| format!("Failed to update clipboard monitor setting: {error}"))?;

    Ok(AppSettings {
        clipboard_monitor_enabled: enabled,
    })
}

pub fn clipboard_monitor_enabled(app: &AppHandle) -> DbResult<bool> {
    let conn = connection::open(app)?;
    get_bool(&conn, CLIPBOARD_MONITOR_ENABLED)
}

fn open_ready_connection(app: &AppHandle) -> DbResult<Connection> {
    let conn = connection::open(app)?;
    schema::ensure(&conn)?;
    Ok(conn)
}

fn get_bool(conn: &Connection, key: &str) -> DbResult<bool> {
    let value: String = conn
        .query_row(
            "select value from settings where key = ?1",
            params![key],
            |row| row.get(0),
        )
        .map_err(|error| format!("Failed to read setting '{key}': {error}"))?;

    Ok(value == "true")
}

fn bool_to_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
