use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::components::app_list::app_entry_list;

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
                Style::new().green().italic(),
            ),
            ".".into(),
        ]),
        Line::from("Push 'q' to quit".red()),
    ];
    frame.render_widget(
        Paragraph::new(text).block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );
    frame.render_widget(
        Paragraph::new("Derpity: Desktop File Management")
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[1],
    );
    frame.render_widget(app_entry_list(vec![]), inner_layout[0]);
}
