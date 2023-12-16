// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use manager_core::{app_entry::AppEntry, app_entry_write, app_finder};
use tauri::{Manager, Window};

// this is how the front end will ask for information
#[tauri::command]
async fn get_shared_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    app_finder::list_shared_apps()
}

#[tauri::command]
async fn get_user_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    app_finder::list_user_apps()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            // up next is to 1) update front end to send snake_case paylaod (or have serde do it)
            // and make serde ignore unknown values OR remove them on FE
            // 2) respond to front end with errors
            main_window.listen("entry_update", |event| {
                if let Some(update_payload) = event.payload() {
                    let entry: AppEntry = serde_json::from_str(update_payload).expect("derp");
                    if let Err(e) = app_entry_write::update_entry(entry) {
                        eprintln!("Failed to update entry {:?}", e);
                    }
                } else {
                    eprintln!("No payload");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_user_apps, get_shared_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
