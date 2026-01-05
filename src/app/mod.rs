mod events;
mod models;
mod search;
mod state;

pub use models::{InputMode, Page};
pub use state::AppState;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            state: AppState::new()?,
        })
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn should_exit(&self) -> bool {
        self.state.exit
    }

    /// handle input
    pub fn handle_event(&mut self) -> anyhow::Result<()> {
        self.state.check_download_progress();

        if let Event::Key(key) = event::read()? {
            // global keys
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.state.exit = true;
                    return Ok(());
                }
                KeyCode::Char('1') => {
                    self.state.page = Page::Translation;
                    return Ok(());
                }
                KeyCode::Char('2') => {
                    self.state.page = Page::Management;
                    return Ok(());
                }
                KeyCode::Char('3') => {
                    self.state.page = Page::Download;
                    if self.state.available_dicts.is_none() && !self.state.loading_dicts {
                        self.state.fetch_available_dictionaries();
                    }
                    return Ok(());
                }
                _ => {}
            }

            // page specific handlers
            match self.state.page {
                Page::Translation => self.state.handle_translation_event(key)?,
                Page::Management => self.state.handle_management_event(key)?,
                Page::Download => self.state.handle_download_event(key)?,
            }
        }
        Ok(())
    }
}
