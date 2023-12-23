use std::{fs, time::SystemTime};

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
                eprintln!("Failed to infer level from string {}: {}", level_str, e);
            }
        };
        // TIMESTAMP and MESSAGE
        if let Some((timestamp_str, msg)) = res.split_once(']') {
            match DateTime::parse_from_str(timestamp_str.trim(), TIMESTAMP_SHAPE) {
                Ok(datetime) => {
                    timestamp = datetime.into();
                }
                Err(e) => {
                    eprintln!(
                        "Failed to parse datetime from string {} with err {}",
                        timestamp_str, e
                    );
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

    if let Ok(logs) = fs::read(log_location_dir) {
        let logs_string = if let Ok(s) = String::from_utf8(logs) {
            s
        } else {
            eprintln!("Failed to read log");
            String::new()
        };
        logs_string
            .split('\n')
            .map(parse_log_from_string)
            .filter(|l| l.message.len() > 0) // filter out invalid entries
            .collect()
    } else {
        eprintln!("Failed to read from dir");
        Vec::new()
    }
}
