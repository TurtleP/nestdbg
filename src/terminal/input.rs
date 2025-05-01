use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::{message::Message, terminal::app::Application};

pub async fn handle_popup_input(state: &mut Application, key_event: KeyEvent) -> Message {
    match key_event.code {
        KeyCode::Up => {
            state.selected_connection =
                (state.selected_connection + state.config.get_connections().len() - 1)
                    % state.config.get_connections().len();
        }
        KeyCode::Down => {
            state.selected_connection =
                (state.selected_connection + 1) % state.config.get_connections().len();
        }
        KeyCode::Enter => {
            let id = state.selected_connection;

            if let Some(connection) = state.config.get_connection(id) {
                if let Ok(_) = state.network_manager.connect(connection).await {
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

pub async fn handle_input(state: &mut Application) -> Message {
    if event::poll(Duration::from_millis(100)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind != KeyEventKind::Press {
                return Message::none();
            }

            if state.show_popup {
                return handle_popup_input(state, key_event).await;
            }

            match key_event.code {
                KeyCode::Char(c) => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        match c {
                            'c' => return Message::quit(),
                            ' ' => return Message::show_popup(),
                            _ => state.input.push(c),
                        }
                    } else {
                        state.input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    state.input.pop();
                }
                KeyCode::Enter => {
                    if !state.input.is_empty() {
                        let command = state.input.clone();
                        state.input.clear();
                        return Message::send_input(command);
                    }
                }
                _ => {}
            }
        }
    }
    Message::none()
}
