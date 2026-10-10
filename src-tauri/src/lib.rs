mod commands;

use std::sync::Mutex;

use tauri::Manager;

/// The open library. One connection, shared by every command.
pub struct Library(pub Mutex<rusqlite::Connection>);

/// Which device wrote a change; recorded in `change_log` for sync.
pub struct Device(pub &'static str);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // The phone keeps its database in Documents, which iOS file
            // sharing exposes to the Companion over USB. The Companion keeps
            // the main library in its own data folder.
            #[cfg(mobile)]
            let (dir, device) = (app.path().document_dir()?, "phone");
            #[cfg(desktop)]
            let (dir, device) = (app.path().app_data_dir()?, "companion");

            std::fs::create_dir_all(&dir)?;
            let conn = discover_core::db::open(&dir.join("discover.sqlite"))?;
            app.manage(Library(Mutex::new(conn)));
            app.manage(Device(device));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::search_library,
            commands::add_copy,
            commands::add_to_wishlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Disc-over");
}
