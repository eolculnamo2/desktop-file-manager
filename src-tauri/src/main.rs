// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::future::Future;

use manager_core::{app_entry::AppEntry, app_finder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// this is how the front end will ask for information
#[tauri::command]
async fn get_shared_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    // let main_window = app.get_window("main").unwrap();
    // main_window
    //     .emit("list-upated", &manager_core::app_finder::list_shared_apps())
    //     .unwrap();
    app_finder::list_shared_apps()
}

#[tauri::command]
async fn get_user_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    app_finder::list_user_apps()
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // let main_window = app.get_window("main").unwrap();
            // not sure why, but this doesn't seem to work on load
            // main_window
            //     .emit("list-upated", &manager_core::app_finder::list_shared_apps())
            //     .unwrap();

            // this is how we'll get data from the front end;
            // main_window.listen("refresh-shared-apps", |event| {
            //     println!("got window event-name with payload {:?}", event.payload());
            // });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_user_apps, get_shared_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
