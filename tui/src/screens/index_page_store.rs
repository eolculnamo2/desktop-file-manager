use ratatui::widgets::TableState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexPageStore {
    pub user_table_state: TableState,
    pub shared_table_state: TableState,
}

impl IndexPageStore {
    pub fn new() -> Self {
        Self {
            user_table_state: TableState::default(),
            shared_table_state: TableState::default(),
        }
    }
}

impl IndexPageStore {
    pub fn scroll_user_up(&mut self) {
        if self.user_table_state.offset() > 0 {
            self.user_table_state
                .select(Some(self.user_table_state.offset() - 1));
            *self.user_table_state.offset_mut() -= 1;
        }
    }
    pub fn scroll_user_down(&mut self) {
        self.user_table_state
            .select(Some(self.user_table_state.offset() + 1));
        *self.user_table_state.offset_mut() += 1;
    }
    pub fn scroll_shared_up(&mut self) {
        if self.shared_table_state.offset() > 0 {
            self.shared_table_state
                .select(Some(self.shared_table_state.offset() - 1));
            *self.shared_table_state.offset_mut() -= 1;
        }
    }
    pub fn scroll_shared_down(&mut self) {
        self.shared_table_state
            .select(Some(self.shared_table_state.offset() + 1));
        *self.shared_table_state.offset_mut() += 1;
    }
}
