use crate::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    render_search_bar(f, app, chunks[0]);
    render_main_area(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let dict_name = if app.active_dict_is_de_en {
        "DE -> EN"
    } else {
        "EN -> DE"
    };

    let input_style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::Yellow),
        InputMode::Normal => Style::default(),
    };

    let input = Paragraph::new(app.input.as_str()).style(input_style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Search ({}) ", dict_name)),
    );
    f.render_widget(input, area);
}

fn render_main_area(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let items: Vec<ListItem> = app
        .results
        .iter()
        .map(|entry| ListItem::new(Line::from(entry.headword.clone())))
        .collect();

    let highlight_style = Style::default()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Results "))
        .highlight_style(highlight_style);

    let mut state = ListState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(list, chunks[0], &mut state);

    let definition_text = if let Some(entry) = app.results.get(app.selected_index) {
        entry.definition.clone()
    } else {
        String::from("No result selected.")
    };

    let definition = Paragraph::new(definition_text)
        .block(Block::default().borders(Borders::ALL).title(" Definition "))
        .wrap(Wrap { trim: true });
    f.render_widget(definition, chunks[1]);
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = "Esc: Normal Mode | 'e': Edit | Tab: Switch Lang | q: Quit";
    let footer = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, area);
}
