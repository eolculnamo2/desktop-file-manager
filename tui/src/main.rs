mod app;
mod components;
mod event_handler;
mod screens;
mod store;

use app::App;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use event_handler::handle_events;
use manager_core::app_finder;
use ratatui::prelude::*;
use screens::index_page::index_page;
use std::io::{self, stdout};
use store::{
    entries_store::{EntriesStore, ENTRIES_STORE},
    nav_store::{Screens, NAV_STORE},
    tui_store::TUI_STORE,
};

// scope read lock to function, not the entire while loop
// *note the lock seems fine in the while loop?
// fn should_quit() -> bool {
//     TUI_STORE.read().unwrap().quit
// }

fn main() -> io::Result<()> {
    let user_apps = app_finder::list_user_apps().expect("unable to load user apps");
    let shared_apps = app_finder::list_shared_apps().expect("unable to load shared apps");
    let mut app = App::new();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    *ENTRIES_STORE.write().expect("Failed to read entries store") = EntriesStore {
        shared_entries: shared_apps,
        user_entries: user_apps,
    };

    // while should_quit() == false {
    while TUI_STORE.read().unwrap().quit == false {
        terminal.draw(|f| ui(f, &mut app))?;
        handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    match NAV_STORE.read().unwrap().get_current_screen() {
        Screens::Index => index_page(frame, app),
    };
}
