// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn app_list_changed(app: tauri::AppHandle) {
    app.emit_all("list-upated", &manager_core::app_finder::list_shared_apps())
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.emit_all("list-upated", &manager_core::app_finder::list_shared_apps())
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, app_list_changed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
