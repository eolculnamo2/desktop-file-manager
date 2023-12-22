use manager_core::app_entry::Access;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{components::app_list::app_entry_list, store::entries_store::ENTRIES_STORE};

// Next is to populate lists with formatted data rows
pub fn index_page(frame: &mut Frame) {
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
    frame.render_widget(
        Paragraph::new(text).block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );

    frame.render_widget(
        app_entry_list(
            &ENTRIES_STORE
                .read()
                .expect("Failed to read users entries")
                .user_entries,
            Access::User,
        ),
        outer_layout[1],
    );
    frame.render_widget(
        app_entry_list(
            &ENTRIES_STORE
                .read()
                .expect("Failed to read shared entries")
                .shared_entries,
            Access::Shared,
        ),
        inner_layout[0],
    );
}
