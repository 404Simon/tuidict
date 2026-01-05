use crate::app::AppState;
use crate::components::{StatusBar, StatusType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.size());

    render_management_list(f, state, chunks[0]);
    render_footer(
        f,
        chunks[1],
        "1: Translation | 2: [Manage] | 3: Download | Space/Enter: Toggle | d: Delete | q: Quit",
    );
}

fn render_management_list(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .config
        .dictionaries
        .iter()
        .map(|dict| {
            let status = if dict.active { "[✓]" } else { "[ ]" };
            let text = format!(
                "{} {} ({} -> {})",
                status, dict.name, dict.from_lang, dict.to_lang
            );
            ListItem::new(Line::from(text))
        })
        .collect();

    let highlight_style = Style::default()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Dictionary Management ")
                .title_bottom(" Active dictionaries will be loaded on startup "),
        )
        .highlight_style(highlight_style);

    let mut list_state = ListState::default();
    list_state.select(Some(state.management_selected));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_footer(f: &mut Frame, area: Rect, help_text: &str) {
    let status_bar = StatusBar::new(help_text, StatusType::Help).show_border(false);
    status_bar.render(f, area);
}
