use crate::{
    screens::index_page_store::IndexPageStore,
    store::{entries_store::EntriesStore, nav_store::NavStore, tui_store::TuiStore},
};

// holds app level items such as shared state
// if this works, will transition other global state here
pub struct App {
    pub tui_state: TuiStore,
    pub nav_state: NavStore,
    pub entries_state: EntriesStore,
    pub index_page_state: IndexPageStore,
}

impl App {
    pub fn new() -> Self {
        Self {
            index_page_state: IndexPageStore::new(),
            tui_state: TuiStore::new(),
            nav_state: NavStore::new(),
            entries_state: EntriesStore::new(),
        }
    }
}
