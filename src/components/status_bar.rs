use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub enum StatusType {
    Info,
    Success,
    Error,
    Loading,
    Help,
}

pub struct StatusBar<'a> {
    pub message: &'a str,
    pub status_type: StatusType,
    pub show_border: bool,
}

impl<'a> StatusBar<'a> {
    pub fn new(message: &'a str, status_type: StatusType) -> Self {
        Self {
            message,
            status_type,
            show_border: true,
        }
    }

    pub fn show_border(mut self, show_border: bool) -> Self {
        self.show_border = show_border;
        self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let color = match self.status_type {
            StatusType::Info => Color::Yellow,
            StatusType::Success => Color::Green,
            StatusType::Error => Color::Red,
            StatusType::Loading => Color::Cyan,
            StatusType::Help => Color::DarkGray,
        };

        let style = Style::default().fg(color);

        let widget = if self.show_border {
            Paragraph::new(self.message)
                .style(style)
                .block(Block::default().borders(Borders::ALL).title(" Status "))
        } else {
            Paragraph::new(self.message).style(style)
        };

        f.render_widget(widget, area);
    }
}
