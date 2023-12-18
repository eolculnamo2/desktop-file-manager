use std::{
    sync::{mpsc::Sender, Mutex},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogLevel {
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
            LogLevel::Info => println!("INFO: {}", &self.message),
            LogLevel::Warn => println!("WARN: {}", &self.message),
            LogLevel::Error => println!("ERROR: {}", &self.message),
        }
        if let Some(sender) = &LOG_CHANNEL.lock().expect(LOG_CHANNEL_POISON).sender {
            if let Err(e) = sender.send(self) {
                eprintln!("Failed to send log {}", e);
            }
        } else {
            eprintln!("Failed to send log because channel not initialized");
        }
    }
}

pub struct LoggerChannel {
    pub sender: Option<Sender<Log>>,
}

impl LoggerChannel {
    const fn new() -> Self {
        Self { sender: None }
    }
}

pub static LOG_CHANNEL: Mutex<LoggerChannel> = Mutex::new(LoggerChannel::new());
pub const LOG_CHANNEL_POISON: &'static str = "Log channel poisoned";

pub fn init_channel(sender: Sender<Log>) {
    if LOG_CHANNEL
        .lock()
        .expect(LOG_CHANNEL_POISON)
        .sender
        .is_some()
    {
        println!("Log channel already started");
        return;
    }
    LOG_CHANNEL.lock().expect(LOG_CHANNEL_POISON).sender = Some(sender);
}
