use crate::screens::index_page_store::IndexPageStore;

// holds app level items such as shared state
// if this works, will transition other global state here
pub struct App {
    pub index_page_state: IndexPageStore,
}

impl App {

    pub fn new() -> Self {
        Self {
            index_page_state: IndexPageStore::new(),
        }
    }
}
