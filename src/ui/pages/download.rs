use crate::app::{AppState, InputMode};
use crate::components::{SearchInput, StatusBar, StatusType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(f.size());

    render_download_search(f, state, chunks[0]);
    render_download_list(f, state, chunks[1]);
    render_download_status(f, state, chunks[2]);
    render_footer(
        f,
        chunks[3],
        "1: Translation | 2: Manage | 3: [Download] | Enter: Install | Ctrl+n/p: Navigate | q: Quit",
    );
}

fn render_download_search(f: &mut Frame, state: &AppState, area: Rect) {
    let search_input = SearchInput::new(&state.download_filter)
        .title("Filter Dictionaries")
        .show_cursor(state.download_input_mode == InputMode::Editing)
        .active(state.download_input_mode == InputMode::Editing);
    search_input.render(f, area);
}

fn render_download_list(f: &mut Frame, state: &AppState, area: Rect) {
    if state.loading_dicts {
        let loading = Paragraph::new("Loading available dictionaries...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" Download "));
        f.render_widget(loading, area);
        return;
    }

    let items: Vec<ListItem> = if let Some(dicts) = &state.available_dicts {
        let filtered = state.get_filtered_dicts(dicts);

        if filtered.is_empty() {
            vec![ListItem::new(Line::from("No dictionaries match filter"))]
        } else {
            filtered
                .iter()
                .map(|dict| {
                    let is_installed = state.config.dictionaries.iter().any(|d| d.id == dict.name);

                    let status = if is_installed { "[Installed]" } else { "" };

                    let size_mb = dict
                        .get_dictd_release()
                        .map(|r| r.size / 1024 / 1024)
                        .unwrap_or(0);

                    let text = format!("{} ({} MB) {}", dict.name, size_mb, status);

                    let style = if is_installed {
                        Style::default().fg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    ListItem::new(Line::from(text)).style(style)
                })
                .collect()
        }
    } else {
        vec![ListItem::new(Line::from(
            "Failed to load dictionaries. Press '3' to retry.",
        ))]
    };

    let highlight_style = Style::default()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Available Dictionaries ")
                .title_bottom(" Press Enter to download "),
        )
        .highlight_style(highlight_style);

    let mut list_state = ListState::default();
    list_state.select(Some(state.download_selected));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_download_status(f: &mut Frame, state: &AppState, area: Rect) {
    // if there is progress, show it
    if let Some((downloaded, total)) = state.download_progress {
        if total > 0 {
            let percentage = (downloaded as f64 / total as f64 * 100.0).min(100.0) as u16;

            let downloaded_mb = downloaded as f64 / 1024.0 / 1024.0;
            let total_mb = total as f64 / 1024.0 / 1024.0;
            let label = format!(
                "{:.1} MB / {:.1} MB ({}%)",
                downloaded_mb, total_mb, percentage
            );

            // progress bar
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title(" Download Progress "),
                )
                .gauge_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .label(label)
                .ratio(percentage as f64 / 100.0);

            f.render_widget(gauge, area);
            return;
        }
    }

    // otherwise show status text
    let (status_text, status_type) = if let Some(ref status) = state.download_status {
        let status_type = if status.contains("Failed") || status.contains("failed") {
            StatusType::Error
        } else if status.contains("Success") {
            StatusType::Success
        } else if status.contains("Loading") || status.contains("Downloading") {
            StatusType::Loading
        } else {
            StatusType::Info
        };
        (status.as_str(), status_type)
    } else {
        ("Ready to download", StatusType::Info)
    };

    let status_bar = StatusBar::new(status_text, status_type);
    status_bar.render(f, area);
}

fn render_footer(f: &mut Frame, area: Rect, help_text: &str) {
    let status_bar = StatusBar::new(help_text, StatusType::Help).show_border(false);
    status_bar.render(f, area);
}
