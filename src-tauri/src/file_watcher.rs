use manager_core::logger::Log;

use notify::{
    event::{CreateKind, RemoveKind},
    EventKind, RecommendedWatcher,
};

use crate::TX_FILE_WATCHER;

fn should_handle_update(event: &notify::Event) -> bool {
    if event.kind == EventKind::Create(CreateKind::File) {
        true
    } else if event.kind == EventKind::Remove(RemoveKind::File) {
        true
    } else {
        false
    }
}

fn log_update(event: &notify::Event) {
    if event.kind == EventKind::Create(CreateKind::File) {
        Log::info(format!(
            "File system updated. File at path(s) {:?} created",
            event.paths
        ))
        .send_log();
    } else if event.kind == EventKind::Remove(RemoveKind::File) {
        Log::info(format!(
            "File system updated. File at path(s) {:?} removed",
            event.paths
        ))
        .send_log();
    }
}

pub fn watch_for_changes() -> notify::Result<RecommendedWatcher> {
    let watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            if should_handle_update(&event) == false {
                return;
            }
            log_update(&event);
            if let Ok(tx_cell) = TX_FILE_WATCHER.lock() {
                if let Some(tx) = tx_cell.get() {
                    if let Err(e) = tx.send(event) {
                        Log::warn(format!("Failed to forward file change {}", e)).send_log();
                    }
                }
            }
        }
        Err(e) => Log::error(format!("File watcher create failure {}", e)).send_log(),
    })?;

    Ok(watcher)
}
