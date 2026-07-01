pub mod cases;
pub mod exports;

use cases::Db;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let workspace_dir = app.path().document_dir()?.join("Docket");
            std::fs::create_dir_all(&workspace_dir)?;

            let conn = Connection::open(workspace_dir.join("workspace.db"))?;
            cases::init_schema(&conn)?;
            app.manage(Db(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cases::get_case,
            cases::list_cases,
            cases::create_case,
            cases::update_case,
            cases::delete_case,
            exports::export_cases_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
