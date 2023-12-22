use std::sync::RwLock;

use manager_core::app_entry::AppEntry;

pub static ENTRIES_STORE: RwLock<EntriesStore> = RwLock::new(EntriesStore::new());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntriesStore {
    pub shared_entries: Vec<AppEntry>,
    pub user_entries: Vec<AppEntry>,
}

impl EntriesStore {
    const fn new() -> Self {
        Self {
            shared_entries: Vec::new(),
            user_entries: Vec::new(),
        }
    }
}
