mod app;
mod components;
mod config;
mod dictionary;
mod download;
mod ui;

use app::App;
use crossterm::{
    event::poll,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    while !app.should_exit() {
        terminal.draw(|f| ui::draw(f, app.state()))?;

        // poll with a timeout to update download progress
        if poll(Duration::from_millis(100))? {
            app.handle_event()?;
        } else {
            // no event, just check for download updates
            app.state_mut().check_download_progress();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
