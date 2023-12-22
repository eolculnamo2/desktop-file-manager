use std::sync::RwLock;

pub static TUI_STORE: RwLock<TuiStore> = RwLock::new(TuiStore::new());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiStore {
    pub quit: bool,
}

impl TuiStore {
    const fn new() -> Self {
        Self { quit: false }
    }

    pub fn shutdown(&mut self) {
        self.quit = true;
    }
}
