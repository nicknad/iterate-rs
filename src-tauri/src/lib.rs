mod commands;
mod crypto;
mod database;
mod error;
mod services;
mod state;

use parking_lot::Mutex;
use state::AppState;
use tauri::Manager;

use crate::state::AppConfig;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState {
                db_path: Mutex::new(None),
                serivce_keys: Mutex::new(None),
                app_config: Mutex::new(AppConfig::default()),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::journal::open_journal,
            commands::journal::create_journal,
            commands::journal::unlock_journal,
            commands::record::save_journal_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
