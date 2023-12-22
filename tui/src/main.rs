mod components;
mod screens;
mod store;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use screens::index_page::index_page;
use std::io::{self, stdout};
use store::{
    nav_store::{Screens, NAV_STORE},
    tui_store::TUI_STORE,
};

// scope read lock to function, not the entire while loop
fn should_quit() -> bool {
    TUI_STORE.read().unwrap().quit
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    while should_quit() == false {
        terminal.draw(ui)?;
        handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<()> {
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

fn ui(frame: &mut Frame) {
    match NAV_STORE.read().unwrap().get_current_screen() {
        Screens::Index => index_page(frame),
    };
}
