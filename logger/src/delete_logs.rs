use std::{fs, io};

use crate::{constants::RELATIVE_LOG_LOCATION, log::Log};

pub fn delete_logs() {
    let log_location_dir =
        expanduser::expanduser(RELATIVE_LOG_LOCATION).expect("Failed to expand relative");
    if let Err(e) = fs::remove_file(log_location_dir) {
        // if parent directoory o file itself is missing, ignore
        if e.kind() != io::ErrorKind::NotFound {
            Log::error(format!("Failed to remove logs {}", e)).send_log();
        }
    }
}
