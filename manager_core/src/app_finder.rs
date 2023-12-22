use std::{
    fs::{read_dir, ReadDir},
    sync::Mutex,
};

use logger::log::Log;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    app_entry::AppEntry,
    constants::location_constants::{get_shared_app, get_user_app},
    desktop_file_parser::{self, DesktopFileParseError},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListAppError {
    IoError,
    ParseAppError(DesktopFileParseError),
}

impl From<std::io::Error> for ListAppError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

pub type Result<T> = std::result::Result<T, ListAppError>;

/// Get all shared apps
pub fn list_shared_apps() -> Result<Vec<AppEntry>> {
    Log::info("Fetching shared apps".to_string()).send_log();
    let dir = read_dir(get_shared_app())?;
    list_apps(dir)
}
/// Get all user apps
pub fn list_user_apps() -> Result<Vec<AppEntry>> {
    Log::info("Fetching user apps".to_string()).send_log();
    let dir = read_dir(get_user_app())?;
    list_apps(dir)
}

fn list_apps(dir: ReadDir) -> Result<Vec<AppEntry>> {
    let apps: Mutex<Vec<AppEntry>> = Mutex::new(Vec::new());
    let err: Mutex<Option<ListAppError>> = Mutex::new(None);

    dir.par_bridge().for_each(|dir_entry_res| {
        if let Ok(dir_entry) = dir_entry_res {
            let path = dir_entry.path();
            if path.to_str().expect("Invalid path").ends_with(".desktop") {
                let result = desktop_file_parser::get_entry_from_file(path);
                if let Err(ref e) = result {
                    if e != &DesktopFileParseError::InvalidHeader {
                        *err.lock().expect("Poisoned lock") =
                            Some(ListAppError::ParseAppError(e.clone()));
                    }
                }
                if let Ok(app) = result {
                    apps.lock().expect("Lock poisoned").push(app);
                }
            }
        };
    });
    if let Some(e) = err.lock().expect("Lock poisoned").as_ref() {
        return Err(e.clone());
    }
    let mut apps = apps.lock().expect("lock poisoined");
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(apps.to_vec())
}
