use std::{fs, io, time::SystemTime};

use chrono::DateTime;

use crate::{
    constants::{RELATIVE_LOG_LOCATION, TIMESTAMP_SHAPE},
    log::{Log, LogLevel},
};

fn parse_log_from_string(s: &str) -> Log {
    let mut message = String::new();
    let mut level = LogLevel::Info;
    let mut timestamp = SystemTime::now();

    if let Some((level_str, res)) = s.split_once('[') {
        // LEVEL
        match LogLevel::try_from(level_str) {
            Ok(l) => {
                level = l;
            }
            Err(e) => {
                Log::error(format!(
                    "Failed to infer level from string {}: {}",
                    level_str, e
                ))
                .send_log();
            }
        };
        // TIMESTAMP and MESSAGE
        if let Some((timestamp_str, msg)) = res.split_once(']') {
            match DateTime::parse_from_str(timestamp_str.trim(), TIMESTAMP_SHAPE) {
                Ok(datetime) => {
                    timestamp = datetime.into();
                }
                Err(e) => {
                    Log::error(format!(
                        "Failed to parse datetime from string {} with err {}",
                        timestamp_str, e
                    ))
                    .send_log();
                }
            }
            message = msg.to_string();
        }
    };
    let mut log = Log::new(message, level);
    log.timestamp = timestamp;
    log
}

pub fn read_logs() -> Vec<Log> {
    let log_location_dir =
        expanduser::expanduser(RELATIVE_LOG_LOCATION).expect("Failed to expand relative");

    match fs::read(log_location_dir) {
        Ok(logs) => {
            let logs_string = if let Ok(s) = String::from_utf8(logs) {
                s
            } else {
                Log::error(String::from("Failed to read log")).send_log();
                String::new()
            };
            logs_string
                .split('\n')
                .map(parse_log_from_string)
                .filter(|l| l.message.len() > 0) // filter out invalid entries
                .collect()
        }
        Err(e) => {
            if e.kind() != io::ErrorKind::NotFound {
                Log::error(String::from("Failed to read from dir")).send_log();
            }
            Vec::new()
        }
    }
}
