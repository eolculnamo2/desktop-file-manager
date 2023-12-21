use serde::{Deserialize, Serialize};

use crate::{
    app_entry::{Access, AppEntry},
    constants::location_constants::get_shared_app,
    logger::Log,
};
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub enum DeleteEntryError {
    IoError,
}

impl From<io::Error> for DeleteEntryError {
    fn from(e: io::Error) -> Self {
        Log::error(format!("IO Error - Failed to delete file {}", e)).send_log();
        DeleteEntryError::IoError
    }
}

pub fn delete_entry(entry: AppEntry) -> Result<Access, DeleteEntryError> {
    let abs_path = entry
        .absolute_path
        .to_str()
        .unwrap_or("[Unable to find path]");

    fs::remove_file(&abs_path)?;
    Log::info(format!(
        "Entry {} was deleted from path {}",
        entry.name, &abs_path,
    ))
    .send_log();

    if abs_path.contains(get_shared_app()) {
        Ok(Access::Shared)
    } else {
        Ok(Access::User)
    }
}
