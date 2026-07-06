use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use super::DbResult;

const DATABASE_FILE_NAME: &str = "project-c.sqlite";

pub fn open(app: &AppHandle) -> DbResult<Connection> {
    let path = database_path(app)?;
    Connection::open(path).map_err(|error| format!("Failed to open database: {error}"))
}

fn database_path(app: &AppHandle) -> DbResult<PathBuf> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    Ok(data_dir.join(DATABASE_FILE_NAME))
}
