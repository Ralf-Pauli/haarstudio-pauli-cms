mod models;
mod commands;

use commands::services_commands::load_categories;
use commands::services_commands::save_categories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_categories, save_categories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
