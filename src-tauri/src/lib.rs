use crate::db::init_db::init_db;

pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_db();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
