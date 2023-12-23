use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::{
    app::App,
    store::tui_store::{Tab, TUI_STORE},
};

const POISONED_TUI_STORE: &'static str = "Tui store lock poisoned";
pub fn handle_events(app: &mut App) -> io::Result<()> {
    // would it be better to use mpsc instead of polling?
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                TUI_STORE.write().expect(POISONED_TUI_STORE).shutdown();
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Tab {
                TUI_STORE.write().expect(POISONED_TUI_STORE).next_tab();
            } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::BackTab {
                TUI_STORE.write().expect(POISONED_TUI_STORE).prev_tab();
            } else if key.kind == event::KeyEventKind::Press
                && (key.code == KeyCode::Char('k') || key.code == KeyCode::Up)
            {
                let current_tab = TUI_STORE
                    .read()
                    .expect(POISONED_TUI_STORE)
                    .current_tab
                    .clone();
                if current_tab == Tab::UserEntryList {
                    app.index_page_state.scroll_user_up();
                } else if current_tab == Tab::SharedEntryList {
                    app.index_page_state.scroll_shared_up();
                }
            } else if key.kind == event::KeyEventKind::Press
                && (key.code == KeyCode::Char('j') || key.code == KeyCode::Down)
            {
                let current_tab = TUI_STORE
                    .read()
                    .expect(POISONED_TUI_STORE)
                    .current_tab
                    .clone();
                if current_tab == Tab::UserEntryList {
                    app.index_page_state.scroll_user_down();
                } else if current_tab == Tab::SharedEntryList {
                    app.index_page_state.scroll_shared_down();
                }
            }
        }
    }
    Ok(())
}
