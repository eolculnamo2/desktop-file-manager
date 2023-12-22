use manager_core::app_entry::AppEntry;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListDirection},
};

pub fn app_entry_list<'a>(entries: Vec<AppEntry>) -> List<'a> {
    let items = ["Item 1", "Item 2", "Item 3"];
    List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}
