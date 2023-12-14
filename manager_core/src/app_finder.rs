use std::{
    fs::{read_dir, ReadDir},
    rc::Rc,
};

use crate::{
    app_entry::{AppEntry, AppType},
    constants::location_constants::{get_shared_app, get_user_app},
    desktop_file_parser::{self, DesktopFileParseError},
};

pub enum ListAppError {
    IoError(std::io::Error),
    ParseAppError(DesktopFileParseError),
}

impl From<std::io::Error> for ListAppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub type Result<T> = std::result::Result<T, ListAppError>;

/// Get all shared apps
pub fn list_shared_apps() -> Result<Vec<AppEntry>> {
    let dir = read_dir(get_shared_app())?;
    list_apps(dir)
}
/// Get all user apps
pub fn list_user_apps() -> Result<Vec<AppEntry>> {
    let dir = read_dir(get_user_app())?;
    list_apps(dir)
}

fn list_apps(dir: ReadDir) -> Result<Vec<AppEntry>> {
    let mut apps: Vec<AppEntry> = Vec::new();
    for dir_entry_res in dir {
        if let Ok(dir_entry) = dir_entry_res {
            let path = dir_entry.path();
            if path.to_str().expect("Invalid path").ends_with(".desktop") {
                let result = desktop_file_parser::get_entry_from_file(path);
                if let Err(ref e) = result {
                    if e != &DesktopFileParseError::InvalidHeader {
                        return Err(ListAppError::ParseAppError(e.clone()));
                    }
                }
                if let Ok(app) = result {
                    apps.push(app);
                }
            }
        };
    }
    Ok(apps)
}
