use manager_core::app_entry::Access;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::App,
    components::app_list::app_entry_list,
    store::{
        entries_store::ENTRIES_STORE,
        tui_store::{get_selected_style, Tab, TUI_STORE},
    },
};

// Next is to populate lists with formatted data rows
pub fn index_page(frame: &mut Frame, app: &mut App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100)])
        .split(outer_layout[2]);

    let text = vec![
        Line::from(vec![
            Span::raw("Derpity"),
            Span::styled(
                " : Gnome Desktop Files Manager",
                Style::new().cyan().italic(),
            ),
        ]),
        Line::from("Push 'n' to add new desktop file".green()),
        Line::from("Push 'l' to view logs".green()),
        Line::from("Push 'q' to quit".red()),
    ];

    let mut current_tab = Tab::None;
    {
        current_tab = TUI_STORE.read().expect("Lock poisoned").current_tab.clone();
    }
    frame.render_widget(
        Paragraph::new(text)
            .style(get_selected_style(current_tab == Tab::Header))
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );

    frame.render_stateful_widget(
        app_entry_list(
            &ENTRIES_STORE
                .read()
                .expect("Failed to read users entries")
                .user_entries,
            Access::User,
            current_tab == Tab::UserEntryList,
        ),
        outer_layout[1],
        &mut app.index_page_state.user_table_state,
    );

    frame.render_stateful_widget(
        app_entry_list(
            &ENTRIES_STORE
                .read()
                .expect("Failed to read shared entries")
                .shared_entries,
            Access::Shared,
            current_tab == Tab::SharedEntryList,
        ),
        inner_layout[0],
        &mut app.index_page_state.shared_table_state,
    );
}
