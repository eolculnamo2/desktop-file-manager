use std::{
    cell::OnceCell,
    fs::{DirBuilder, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    sync::{mpsc::Sender, Mutex, OnceLock, RwLock},
    time::SystemTime,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: SystemTime,
    pub message: String,
    pub level: LogLevel,
}

impl Log {
    pub fn new(message: String, level: LogLevel) -> Self {
        Self {
            timestamp: SystemTime::now(),
            message,
            level,
        }
    }

    pub fn trace(message: String) -> Self {
        Self::new(message, LogLevel::Trace)
    }

    pub fn info(message: String) -> Self {
        Self::new(message, LogLevel::Info)
    }

    pub fn warn(message: String) -> Self {
        Self::new(message, LogLevel::Warn)
    }

    pub fn error(message: String) -> Self {
        Self::new(message, LogLevel::Error)
    }

    pub fn send_log(self) {
        persist_log(&self);
        match self.level {
            LogLevel::Trace => println!("TRACE: {}", &self.message),
            LogLevel::Info => println!("INFO: {}", &self.message),
            LogLevel::Warn => println!("WARN: {}", &self.message),
            LogLevel::Error => println!("ERROR: {}", &self.message),
        }
        if let Some(logger_channel) = &LOG_CHANNEL.get() {
            if let Err(e) = logger_channel.sender.send(self) {
                eprintln!("Failed to send log {}", e);
            }
        } else {
            eprintln!("Failed to send log because channel not initialized");
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoggerChannel {
    pub sender: Sender<Log>,
}

impl LoggerChannel {
    fn new(sender: Sender<Log>) -> Self {
        Self { sender }
    }
}

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

fn persist_log(log: &Log) {
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

// pub static LOG_CHANNEL: Mutex<OnceCell<LoggerChannel>> = Mutex::new(OnceCell::new());
pub static LOG_CHANNEL: OnceLock<LoggerChannel> = OnceLock::new();

pub fn init_channel(sender: Sender<Log>) {
    let log_init_result = LOG_CHANNEL.set(LoggerChannel::new(sender));
    if let Err(_) = log_init_result {
        eprintln!("Log channel already started");
        return;
    }
}
