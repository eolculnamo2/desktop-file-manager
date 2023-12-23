use std::{
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

pub const RELATIVE_LOG_DIR: &'static str = "~/.local/state/derpity";
pub const RELATIVE_LOG_LOCATION: &'static str = "~/.local/state/derpity/logs.txt";
pub static LOG_LOCATION: RwLock<OnceLock<PathBuf>> = RwLock::new(OnceLock::new());
pub static TIMESTAMP_SHAPE: &'static str = "%Y-%m-%d %H:%M:%S%z";
