use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{log_channel::LOG_CHANNEL, persist::persist_log};

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
