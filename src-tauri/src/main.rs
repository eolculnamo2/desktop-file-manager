// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod file_watcher;

use std::{
    path::Path,
    sync::{
        mpsc::{channel, Sender},
        Arc, OnceLock,
    },
    thread,
    time::Instant,
};

use file_watcher::watch_for_changes;
use manager_core::{
    app_entry::{events_list_types, AppEntry},
    app_entry_write,
    app_finder::{self, ListAppError},
    constants::location_constants::{self},
    delete_entry::{delete_entry, DeleteEntryError},
};

use logger::{log::Log, log_channel::init_logger_channel, read_logs::read_logs};
use notify::{RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tauri::Manager;

const EMIT_FAILED: &'static str = "Emit failed";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllApps {
    user_apps: Option<Vec<AppEntry>>,
    shared_apps: Option<Vec<AppEntry>>,
}
impl AllApps {
    fn new() -> Self {
        Self {
            user_apps: None,
            shared_apps: None,
        }
    }
}

// this is how the front end will ask for information
#[tauri::command]
async fn get_shared_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    let i = Instant::now();
    let apps = app_finder::list_shared_apps();
    Log::trace(format!(
        "Time to fetch shared apps time in nanos {}",
        i.elapsed().as_nanos()
    ))
    .send_log();
    apps
}

#[tauri::command]
async fn get_user_apps(_app: tauri::AppHandle) -> app_finder::Result<Vec<AppEntry>> {
    let i = Instant::now();
    let apps = app_finder::list_user_apps();
    Log::trace(format!(
        "Time to fetch user apps time in nanos {}",
        i.elapsed().as_nanos()
    ))
    .send_log();
    apps
}

#[derive(Serialize, Deserialize)]
enum DeleteEntryCommandError {
    DeleteError(DeleteEntryError),
    LoadAppFailure(ListAppError),
}
impl From<DeleteEntryError> for DeleteEntryCommandError {
    fn from(value: DeleteEntryError) -> Self {
        DeleteEntryCommandError::DeleteError(value)
    }
}
impl From<ListAppError> for DeleteEntryCommandError {
    fn from(value: ListAppError) -> Self {
        DeleteEntryCommandError::LoadAppFailure(value)
    }
}

#[tauri::command]
async fn delete_entry_command(app_entry: AppEntry) -> Result<(), DeleteEntryCommandError> {
    delete_entry(app_entry)?;
    Ok(())
}

#[tauri::command]
async fn get_logs_from_disk() -> Vec<Log> {
    read_logs()
}

// why can i use mutex, but not rwlock for OnceCell?
// pub static TX_FILE_WATCHER: Mutex<OnceCell<Sender<notify::Event>>> = Mutex::new(OnceCell::new());
pub static TX_FILE_WATCHER: OnceLock<Sender<notify::Event>> = OnceLock::new();

fn main() {
    let (tx, rx) = channel();
    let (tx_file_watcher, rx_file_watcher) = channel();

    init_logger_channel(tx);

    if let Err(e) = TX_FILE_WATCHER.set(tx_file_watcher) {
        Log::error(format!("File watcher already set {:?}", e)).send_log();
    }

    let mut watcher = watch_for_changes().expect("failed to start watching");
    let user_apps = location_constants::get_user_app();
    watcher
        .watch(Path::new(&user_apps), RecursiveMode::Recursive)
        .expect("Unable to watch");

    tauri::Builder::default()
        .setup(|app| {
            let main_window = Arc::new(app.get_window("main").unwrap());

            let main_window_clone = main_window.clone();
            thread::spawn(move || loop {
                match rx_file_watcher.recv() {
                    Err(e) => {
                        eprintln!("Receiver failed! {}", e);
                    }
                    Ok(event) => {
                        let changed_folders = events_list_types(event.paths);
                        let mut updated_apps = AllApps::new();

                        if changed_folders.has_user {
                            if let Ok(user_apps) = app_finder::list_user_apps() {
                                updated_apps.user_apps = Some(user_apps);
                            }
                        }
                        if changed_folders.has_shared {
                            if let Ok(shared_apps) = app_finder::list_shared_apps() {
                                updated_apps.shared_apps = Some(shared_apps);
                            }
                        }
                        main_window_clone
                            .emit("files_altered", updated_apps)
                            .expect("Failed to send files altered");
                    }
                };
            });

            let main_window_clone = main_window.clone();
            thread::spawn(move || loop {
                match rx.recv() {
                    Err(e) => {
                        eprintln!("Receiver failed! {}", e);
                    }
                    Ok(payload) => {
                        main_window_clone
                            .emit("log", payload)
                            .expect("failed to send log");
                    }
                };
            });

            let main_window_clone = main_window.clone();
            main_window.listen("entry_create", move |event| {
                let payload = event.payload().ok_or(());
                match payload {
                    Err(_) => {
                        eprintln!("No payload");
                        main_window_clone
                            .emit("entry_create_err", "Payload required")
                            .expect(EMIT_FAILED);
                    }
                    Ok(p) => {
                        let entry: AppEntry = serde_json::from_str(p).expect("Invalid payload");
                        if let Err(e) = app_entry_write::create_entry(entry) {
                            eprintln!("Failed to create entry {:?}", e);
                            main_window_clone
                                .emit(
                                    "entry_create_err",
                                    format!("File may not have saved. Error value: {:?}", e),
                                )
                                .expect(EMIT_FAILED);
                            return;
                        }
                        main_window_clone
                            .emit("entry_create_ok", "Done")
                            .expect(EMIT_FAILED);
                    }
                }
            });

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
                let entry: AppEntry =
                    serde_json::from_str(update_payload).expect("Invalid payload");
                if let Err(e) = app_entry_write::update_entry(entry) {
                    eprintln!("Failed to update entry {:?}", e);
                    main_window_clone
                        .emit(
                            "entry_update_err",
                            format!(
                                "IO Error: File may not have updated. Error value is {:?}",
                                e
                            ),
                        )
                        .expect(EMIT_FAILED);
                    return;
                }
                main_window_clone
                    .emit("entry_update_ok", "Done")
                    .expect(EMIT_FAILED);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_user_apps,
            get_shared_apps,
            delete_entry_command,
            get_logs_from_disk
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
