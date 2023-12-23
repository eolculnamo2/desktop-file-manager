use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::App,
    store::tui_store::{get_selected_style, Tab},
};

pub fn create_entry_page(frame: &mut Frame, app: &mut App) {
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
        Line::from("Push 'h' to go to home screen".green()),
        Line::from("Push 'l' to view logs".green()),
        Line::from("Push 'q' to quit".red()),
    ];

    let current_tab = &app.tui_state.current_tab;
    frame.render_widget(
        Paragraph::new(text)
            .style(get_selected_style(current_tab == &Tab::Header))
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );
}
