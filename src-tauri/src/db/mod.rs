pub mod clips;
mod connection;
mod content;
mod schema;
mod seed;

type DbResult<T> = Result<T, String>;

pub fn init(app: &tauri::AppHandle) -> DbResult<()> {
    let conn = connection::open(app)?;
    schema::ensure(&conn)?;
    seed::ensure(&conn)
}
