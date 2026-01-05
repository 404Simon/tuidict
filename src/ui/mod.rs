mod pages;

use crate::app::{AppState, Page};
use ratatui::Frame;

pub fn draw(f: &mut Frame, state: &AppState) {
    match state.page {
        Page::Translation => pages::translation::render(f, state),
        Page::Management => pages::management::render(f, state),
        Page::Download => pages::download::render(f, state),
    }
}
