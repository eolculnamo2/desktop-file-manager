use std::{
    fs::{DirBuilder, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

use chrono::{DateTime, Utc};

use crate::log::Log;

const RELATIVE_LOG_DIR: &'static str = "~/.local/state/derpity";
const RELATIVE_LOG_LOCATION: &'static str = "~/.local/state/derpity/logs.txt";
static LOG_LOCATION: RwLock<OnceLock<PathBuf>> = RwLock::new(OnceLock::new());

fn is_location_unset() -> bool {
    LOG_LOCATION
        .read()
        .expect("Failed to acquire lock")
        .get()
        .is_none()
}

fn handle_missing_state_dir() {
    let log_location_dir =
        expanduser::expanduser(RELATIVE_LOG_DIR).expect("Failed to expand relative");
    DirBuilder::new()
        .recursive(true)
        .create(log_location_dir)
        .expect("Failed to create state dir");
}

fn create_open_options(log_location: &PathBuf) -> Result<File, io::Error> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_location)
}

pub fn persist_log(log: &Log) {
    let mut open_options_file: Option<File> = None;
    if is_location_unset() {
        handle_missing_state_dir();
        let log_location = expanduser::expanduser(RELATIVE_LOG_LOCATION)
            .expect("Failed to convert relative log state to full log state");

        open_options_file =
            Some(create_open_options(&log_location).expect("Failed to open logs file"));

        let _ = LOG_LOCATION
            .write()
            .expect("Failed to acquire lock")
            .set(log_location);
    }

    if let Some(log_location) = LOG_LOCATION.read().expect("Failed to acquire lock").get() {
        if let None = open_options_file {
            match create_open_options(&log_location) {
                Ok(options) => {
                    open_options_file = Some(options);
                }
                // if we fail to create the file
                // because of NOT_FOUND, attempt to create path
                // and try again
                Err(err) => {
                    if err.kind() == io::ErrorKind::NotFound {
                        handle_missing_state_dir();
                        open_options_file = Some(
                            create_open_options(&log_location).expect("Failed to open logs file"),
                        );
                    }
                }
            }
        }

        let datetime: DateTime<Utc> = log.timestamp.into();
        let log_input = format!(
            "{:?} [{}] {}\n",
            &log.level,
            datetime.format("%d/%m/%Y %T"),
            &log.message
        );

        if let Some(mut open_options_file) = open_options_file {
            open_options_file
                .write(log_input.as_bytes())
                .expect("Failed to write log");
        } else {
            eprintln!("Open options file is unexpectedly None");
        }
        return;
    }
    eprintln!("Failed to set log state location. Logs may not persists");
}
