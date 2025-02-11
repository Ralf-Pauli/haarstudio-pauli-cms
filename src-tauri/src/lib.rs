mod commands;
mod utils;
mod models;

use commands::category_commands::load_categories;
use commands::category_commands::save_categories;
use commands::contact_commands::save_contact;
use commands::contact_commands::load_contact;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_categories, save_categories, save_contact, load_contact])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
