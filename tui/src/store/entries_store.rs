use manager_core::app_entry::AppEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntriesStore {
    pub shared_entries: Vec<AppEntry>,
    pub user_entries: Vec<AppEntry>,
}

impl EntriesStore {
    pub fn new() -> Self {
        Self {
            shared_entries: Vec::new(),
            user_entries: Vec::new(),
        }
    }
}
