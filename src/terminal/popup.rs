// src/popup.rs

use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct Popup {
    pub title: String,
    pub content: String,
    pub width_percent: u16,
    pub height_percent: u16,
}

impl Popup {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: String::new(),
            width_percent: 60,
            height_percent: 40,
        }
    }

    pub fn with_size(mut self, width: u16, height: u16) -> Self {
        self.width_percent = width;
        self.height_percent = height;
        self
    }

    pub fn render(&self, f: &mut Frame) -> Rect {
        let area = centered_rect(self.width_percent, self.height_percent, f.area());
        f.render_widget(Clear, area);

        let paragraph = Paragraph::new(Text::from(self.content.clone())).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone())
                .title_alignment(Alignment::Center),
        );

        f.render_widget(paragraph, area);
        area
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
