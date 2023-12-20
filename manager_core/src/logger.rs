use std::{
    sync::{mpsc::Sender, OnceLock},
    time::SystemTime,
};

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

// pub static LOG_CHANNEL: Mutex<OnceCell<LoggerChannel>> = Mutex::new(OnceCell::new());
pub static LOG_CHANNEL: OnceLock<LoggerChannel> = OnceLock::new();

pub fn init_channel(sender: Sender<Log>) {
    let log_init_result = LOG_CHANNEL.set(LoggerChannel::new(sender));
    if let Err(_) = log_init_result {
        eprintln!("Log channel already started");
        return;
    }
}
