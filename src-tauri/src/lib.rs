use crate::db::get_conn;
use crate::db::init_db::init_db;
use crate::db::seed_db::{fetch_curriculum, seed_db};
use crate::db::year_plan::{create_year_plan, list_all};
use crate::db::year_plan_model::YearPlan;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};

pub mod db;

#[tauri::command]
fn create_year_plan_cmd(name: String, grade: String) {
    let _ = create_year_plan(name, grade);
}

#[tauri::command]
fn list_plans_cmd() -> Vec<YearPlan> {
    list_all().unwrap()
}

struct SetupState {
    backend_init: bool,
}

struct AppState {
    conn: Connection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(SetupState {
            backend_init: false,
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            create_year_plan_cmd,
            list_plans_cmd,
        ])
        .setup(|app| {
            let conn = get_conn().unwrap();
            app.manage(Mutex::new(AppState { conn: conn }));
            spawn(setup(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: AppHandle) -> Result<(), ()> {
    // Do the network I/O first, with no lock held and no `Connection` in scope,
    // so nothing non-Send is ever alive across an .await.
    let content = match fetch_curriculum().await {
        Ok(data) => data,
        Err(err) => panic!("Error while fetching curriculum {}", err),
    };
    {
        let app_state = app.state::<Mutex<AppState>>();
        let state = app_state.lock().unwrap();
        init_db(&state.conn);
        seed_db(&state.conn, content);
    } // `state` (MutexGuard) dropped here, before the next await
    set_complete(app.clone(), app.state::<Mutex<SetupState>>()).await?;
    Ok(())
}

async fn set_complete(app: AppHandle, state: State<'_, Mutex<SetupState>>) -> Result<(), ()> {
    let mut state_lock = state.lock().unwrap();
    state_lock.backend_init = true;
    let splash_window = app.get_webview_window("splashscreen").unwrap();
    let schola_window = app.get_webview_window("schola").unwrap();
    splash_window.close().unwrap();
    schola_window.show().unwrap();
    Ok(())
}
