// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, time::Instant};

use manager_core::{app_entry::AppEntry, app_entry_write, app_finder};
use tauri::Manager;

const EMIT_FAILED: &'static str = "Emit failed";

// this is how the front end will ask for information
#[tauri::command]
async fn get_shared_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    let i = Instant::now();
    let apps = app_finder::list_shared_apps();
    println!("Shared elapsed {}", i.elapsed().as_nanos());
    apps
}

#[tauri::command]
async fn get_user_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    let i = Instant::now();
    let apps = app_finder::list_user_apps();
    println!("User elapsed {}", i.elapsed().as_nanos());
    apps
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = Arc::new(app.get_window("main").unwrap());

            //  respond to front end with errors
            let main_window_clone = main_window.clone();
            main_window.listen("entry_update", move |event| {
                let payload = event.payload();
                if payload.is_none() {
                    eprintln!("No payload");
                    main_window_clone
                        .emit("entry_update_err", "Payload required")
                        .expect(EMIT_FAILED);
                    return;
                }
                let update_payload = payload.expect("Already dealt with None");
                let entry: AppEntry = serde_json::from_str(update_payload).expect("derp");
                if let Err(e) = app_entry_write::update_entry(entry) {
                    eprintln!("Failed to update entry {:?}", e);
                    main_window_clone
                        .emit("entry_update_err", "IO Error: File may not have updated")
                        .expect(EMIT_FAILED);
                    return;
                }
                main_window_clone
                    .emit("entry_update_ok", "Done")
                    .expect(EMIT_FAILED);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_user_apps, get_shared_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
