use notify::{
    event::{CreateKind, RemoveKind},
    EventKind, RecommendedWatcher,
};

use crate::TX_FILE_WATCHER;
use logger::log::Log;

fn should_handle_update(event: &notify::Event) -> bool {
    if event.kind == EventKind::Create(CreateKind::File) {
        Log::info(format!(
            "File system updated. File at path(s) {:?} created",
            event.paths
        ))
        .send_log();
        true
    } else if event.kind == EventKind::Remove(RemoveKind::File) {
        Log::info(format!(
            "File system updated. File at path(s) {:?} removed",
            event.paths
        ))
        .send_log();
        true
    } else {
        false
    }
}

pub fn watch_for_changes() -> notify::Result<RecommendedWatcher> {
    let watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            if should_handle_update(&event) == false {
                return;
            }
            if let Some(tx) = TX_FILE_WATCHER.get() {
                if let Err(e) = tx.send(event) {
                    Log::warn(format!("Failed to forward file change {}", e)).send_log();
                }
            }
        }
        Err(e) => Log::error(format!("File watcher create failure {}", e)).send_log(),
    })?;

    Ok(watcher)
}
