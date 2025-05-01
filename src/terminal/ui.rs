use std::io::Result;

use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear},
    Terminal,
};

use crate::config::Connection;
use crate::terminal::app::Application;

fn popup_area_with_height(area: Rect, percent_x: u16, height: u16) -> Rect {
    let vertical_margin = area.height.saturating_sub(height) / 2;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Length(area.height.saturating_sub(vertical_margin + height)),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn render_popup(
    frame: &mut ratatui::Frame,
    area: Rect,
    selection: usize,
    connections: &[Connection],
) {
    let popup_area = popup_area_with_height(area, 50, 2 + connections.len() as u16);

    let outer_block = Block::default()
        .title("Connections")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(&outer_block, popup_area);

    let inner_area = outer_block.inner(popup_area); // removes border spacing

    // Each row takes a fixed height
    let row_constraints = vec![Constraint::Length(1); connections.len()];
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(inner_area);

    for (i, connection) in connections.iter().enumerate() {
        let row_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[i]);

        let mut style = Style::default();
        if selection == i {
            style = Style::default().fg(Color::Black).bg(Color::White);
        }

        frame.render_widget(
            Paragraph::new(connection.name.as_str()).style(style),
            row_chunks[0],
        );
        frame.render_widget(
            Paragraph::new(connection.address.as_str())
                .alignment(Alignment::Right)
                .style(style),
            row_chunks[1],
        );
    }
}

impl Application {
    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[1]);

            let status_line = format!("[{}]", &mut self.network_manager.get_address());
            let status_bar = Paragraph::new(status_line).centered();
            frame.render_widget(status_bar, chunks[0]);

            let log_block = Block::default().borders(Borders::ALL).title("Log");
            frame.render_widget(log_block, top_chunks[0]);

            let globals = Paragraph::new("")
                .block(Block::default().borders(Borders::ALL).title("Lua Globals"));
            frame.render_widget(globals, top_chunks[1]);

            let input = format!("~> {}", self.input);
            let input_box = Paragraph::new(input);
            frame.render_widget(input_box, chunks[2]);

            if self.show_popup {
                render_popup(
                    frame,
                    frame.area(),
                    self.selected_connection,
                    &self.config.get_connections(),
                );
            }
        })?;

        Ok(())
    }
}
