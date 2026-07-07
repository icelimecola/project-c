mod clipboard;
mod commands;
mod db;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::init(app.handle())
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
            clipboard::monitor::start(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clips::capture_clipboard_text,
            commands::clips::create_clip,
            commands::clips::delete_clip,
            commands::clips::list_clips,
            commands::clips::list_folders,
            commands::clips::toggle_clip_pinned
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
