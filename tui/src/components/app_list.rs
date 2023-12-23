use manager_core::app_entry::{Access, AppEntry};
use ratatui::{prelude::*, widgets::*};

use crate::store::tui_store::get_selected_style;

// replace list and listitems with Tables
pub fn app_entry_list<'a>(
    entries: &Vec<AppEntry>,
    list_type: Access,
    is_current_tab: bool,
) -> Table<'a> {
    let rows: Vec<Row> = entries
        .iter()
        .map(|ent| {
            let icon = ent.icon.clone().unwrap_or(String::new());
            let cells = vec![
                Cell::from(ent.name.clone()),
                Cell::from(format!("{:?}", ent.app_type.clone())),
                Cell::from(icon),
            ];
            Row::new(cells)
        })
        .collect();

    let header_cells = ["Name", "Type", "Icon Location"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan)));

    let header = Row::new(header_cells)
        .height(1)
        .bottom_margin(1)
        .style(get_selected_style(is_current_tab));

    Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Max(25),
            Constraint::Min(50),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(get_selected_style(is_current_tab))
            .title(match list_type {
                Access::User => "User entries",
                Access::Shared => "Shared entries",
            }),
    )
    .highlight_symbol(">> ")
}
