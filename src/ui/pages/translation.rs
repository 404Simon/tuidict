use crate::app::{AppState, InputMode};
use crate::components::{SearchInput, StatusBar, StatusType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.size());

    render_search_bar(f, state, chunks[0]);
    render_main_area(f, state, chunks[1]);
    render_footer(
        f,
        chunks[2],
        "1: [Translation] | 2: Manage | 3: Download | Tab: Switch Dict | q: Quit",
    );
}

fn render_search_bar(f: &mut Frame, state: &AppState, area: Rect) {
    let dict_name = state.get_active_dict_name();
    let title = format!("Search ({})", dict_name);
    let search_input = SearchInput::new(&state.input)
        .title(&title)
        .show_cursor(state.input_mode == InputMode::Editing)
        .active(state.input_mode == InputMode::Editing);
    search_input.render(f, area);
}

fn render_main_area(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let items: Vec<ListItem> = state
        .results
        .iter()
        .map(|entry| ListItem::new(Line::from(entry.headword.clone())))
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
                .title(format!(" Results ({}) ", state.results.len())),
        )
        .highlight_style(highlight_style);

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let definition_text = if state.loaded_dictionaries.is_empty() {
        "No active dictionaries. Press '3' to download or '2' to manage.".to_string()
    } else if let Some(entry) = state.results.get(state.selected_index) {
        entry.definition.clone()
    } else if state.input.is_empty() {
        "Start typing to search...".to_string()
    } else {
        "No results found.".to_string()
    };

    let definition = Paragraph::new(definition_text)
        .block(Block::default().borders(Borders::ALL).title(" Definition "))
        .wrap(Wrap { trim: true });
    f.render_widget(definition, chunks[1]);
}

fn render_footer(f: &mut Frame, area: Rect, help_text: &str) {
    let status_bar = StatusBar::new(help_text, StatusType::Help).show_border(false);
    status_bar.render(f, area);
}
