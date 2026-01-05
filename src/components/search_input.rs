use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct SearchInput<'a> {
    pub value: &'a str,
    pub title: &'a str,
    pub show_cursor: bool,
    pub active: bool,
}

impl<'a> SearchInput<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            title: "Search",
            show_cursor: true,
            active: true,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn show_cursor(mut self, show_cursor: bool) -> Self {
        self.show_cursor = show_cursor;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let style = if self.active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let mut text = self.value.to_string();
        if self.show_cursor {
            text.push('█');
        }

        let border_style = if self.active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let input = Paragraph::new(Span::styled(text, style)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" {} ", self.title)),
        );
        f.render_widget(input, area);
    }
}
