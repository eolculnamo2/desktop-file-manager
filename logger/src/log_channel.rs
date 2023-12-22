use std::sync::{mpsc::Sender, OnceLock};

use crate::log::Log;

#[derive(Debug, Clone)]
pub struct LoggerChannel {
    pub sender: Sender<Log>,
}

impl LoggerChannel {
    fn new(sender: Sender<Log>) -> Self {
        Self { sender }
    }
}

pub static LOG_CHANNEL: OnceLock<LoggerChannel> = OnceLock::new();

pub fn init_logger_channel(sender: Sender<Log>) {
    let log_init_result = LOG_CHANNEL.set(LoggerChannel::new(sender));
    if let Err(_) = log_init_result {
        eprintln!("Log channel already started");
        return;
    }
}
