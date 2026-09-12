use crate::db::init_db::init_db;
use crate::db::year_plan::{create_year_plan, list_all};
use crate::db::year_plan_model::YearPlan;

pub mod db;

#[tauri::command]
fn create_year_plan_cmd(name: String, grade: String) {
    let _ = create_year_plan(name, grade);
}

#[tauri::command]
fn list_plans_cmd() -> Vec<YearPlan> {
    list_all().unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = init_db();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_year_plan_cmd,
            list_plans_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
