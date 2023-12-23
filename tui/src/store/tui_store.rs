use ratatui::style::{Style, Stylize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    None,
    Header,
    UserEntryList,
    SharedEntryList,
}

pub fn get_selected_style(is_current_tab: bool) -> Style {
    if is_current_tab {
        Style::new().yellow()
    } else {
        Style::new()
    }
}

impl Tab {
    // think about switching this doubly linked list?
    fn handle_next(&self, reverse: bool) -> Self {
        match self {
            Tab::None if reverse => Tab::SharedEntryList,
            Tab::None => Tab::Header,
            Tab::Header if reverse => Tab::None,
            Tab::Header => Tab::UserEntryList,
            Tab::UserEntryList if reverse => Tab::Header,
            Tab::UserEntryList => Tab::SharedEntryList,
            Tab::SharedEntryList if reverse => Tab::UserEntryList,
            Tab::SharedEntryList => Tab::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiStore {
    pub current_tab: Tab,
    pub quit: bool,
}

impl TuiStore {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::None,
            quit: false,
        }
    }

    pub fn next_tab(&mut self) {
        let next_tab = self.current_tab.handle_next(false);
        self.current_tab = next_tab;
    }

    pub fn prev_tab(&mut self) {
        let next_tab = self.current_tab.handle_next(true);
        self.current_tab = next_tab;
    }

    pub fn shutdown(&mut self) {
        self.quit = true;
    }
}
