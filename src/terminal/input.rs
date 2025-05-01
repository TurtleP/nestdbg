use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::{message::Message, terminal::app::Application};

impl Application {
    async fn handle_popup_input(&mut self, key_event: KeyEvent) -> Message {
        match key_event.code {
            KeyCode::Up => {
                self.selected_connection =
                    (self.selected_connection + self.config.get_connections().len() - 1)
                        % self.config.get_connections().len();
            }
            KeyCode::Down => {
                self.selected_connection =
                    (self.selected_connection + 1) % self.config.get_connections().len();
            }
            KeyCode::Enter => {
                let id = self.selected_connection;

                if let Some(connection) = self.config.get_connection(id) {
                    if let Ok(_) = self.network_manager.connect(connection).await {
                        return Message::none();
                    }
                    return Message::none();
                }
            }
            KeyCode::Esc => {
                return Message::show_popup();
            }
            _ => {}
        }
        Message::none()
    }

    pub async fn handle_input(&mut self) -> Message {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind != KeyEventKind::Press {
                    return Message::none();
                }

                if self.show_popup {
                    return self.handle_popup_input(key_event).await;
                }

                match key_event.code {
                    KeyCode::Char(c) => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            match c {
                                'c' => return Message::quit(),
                                ' ' => return Message::show_popup(),
                                _ => self.input.push(c),
                            }
                        } else {
                            self.input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    KeyCode::Enter => {
                        if !self.input.is_empty() {
                            let command = self.input.clone();
                            self.input.clear();
                            return Message::send_input(command);
                        }
                    }
                    _ => {}
                }
            }
        }
        Message::none()
    }
}
