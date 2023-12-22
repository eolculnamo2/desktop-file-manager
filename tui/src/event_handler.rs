use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::store::tui_store::TUI_STORE;

pub fn handle_events() -> io::Result<()> {
    // would it be better to use mpsc instead of polling?
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                TUI_STORE.write().unwrap().shutdown();
            }
        }
    }
    Ok(())
}
