use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::{app::App, store::tui_store::Tab};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    // would it be better to use mpsc instead of polling?
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                app.tui_state.shutdown();
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Tab {
                app.tui_state.next_tab();
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::BackTab {
                app.tui_state.prev_tab();
            } else if key.kind == event::KeyEventKind::Press
                && (key.code == KeyCode::Char('k') || key.code == KeyCode::Up)
            {
                let current_tab = &app.tui_state.current_tab;
                if current_tab == &Tab::UserEntryList {
                    app.index_page_state.scroll_user_up();
                } else if current_tab == &Tab::SharedEntryList {
                    app.index_page_state.scroll_shared_up();
                }
            } else if key.kind == event::KeyEventKind::Press
                && (key.code == KeyCode::Char('j') || key.code == KeyCode::Down)
            {
                let current_tab = &app.tui_state.current_tab;
                if current_tab == &Tab::UserEntryList {
                    app.index_page_state.scroll_user_down();
                } else if current_tab == &Tab::SharedEntryList {
                    app.index_page_state.scroll_shared_down();
                }
            }
        }
    }
    Ok(())
}
