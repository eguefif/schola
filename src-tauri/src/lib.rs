use crate::db::get_conn;
use crate::db::init_db::init_db;
use crate::db::seed_db::{fetch_curriculum, fetch_year, seed_db};
use crate::db::year_plan::{create_year_plan, list_all};
use crate::db::year_plan_model::YearPlan;
use rusqlite::Connection;
use serde_json::json;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
//use tauri::Wry;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::{JsonValue, StoreExt};

pub mod db;

#[tauri::command]
fn create_year_plan_cmd(name: String, grade: String) {
    let _ = create_year_plan(name, grade);
}

#[tauri::command]
fn list_plans_cmd() -> Vec<YearPlan> {
    list_all().unwrap()
}

struct AppState {
    conn: Connection,
}

const STORE_PATH: &str = "./store.json";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            create_year_plan_cmd,
            list_plans_cmd,
        ])
        .setup(|app| {
            let store = app.store(STORE_PATH)?;
            let conn = get_conn().unwrap();
            app.manage(Mutex::new(AppState { conn: conn }));
            match store.get("initialized") {
                Some(JsonValue::Bool(true)) => log::info!("Already initialized."),
                _ => {
                    let _ = spawn(setup(app.handle().clone()));
                    store.set("initialized", json!(true));
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: AppHandle) -> Result<(), ()> {
    log::info!("Loading school calendar");
    let school_calendar = match fetch_year().await {
        Ok(data) => data,
        Err(err) => panic!("Error while fetching year {}", err),
    };
    let store = app.store(STORE_PATH).expect("Failed to open store");
    store.set("schoolCalendar", json!({"value": school_calendar}));

    log::info!("Loading Curriculum");
    let content = match fetch_curriculum().await {
        Ok(data) => data,
        Err(err) => panic!("Error while fetching curriculum {}", err),
    };
    {
        let app_state = app.state::<Mutex<AppState>>();
        let state = app_state.lock().unwrap();
        init_db(&state.conn);
        seed_db(&state.conn, content);
    }
    Ok(())
}
