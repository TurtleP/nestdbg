use ratatui::DefaultTerminal;

use crate::config::{load_config, ConnectionConfig};
use crate::network::NetworkManager;

pub struct Application {
    pub terminal: DefaultTerminal,
    pub input: String,
    pub log: Vec<String>,
    pub globals: Vec<String>,
    pub show_popup: bool,
    pub config: ConnectionConfig,

    pub network_manager: NetworkManager,
    pub selected_connection: usize,
}

impl Drop for Application {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl Application {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            input: String::new(),
            log: Vec::new(),
            globals: Vec::new(),
            show_popup: false,

            config: load_config(),

            network_manager: NetworkManager::new(),
            selected_connection: 0,
        }
    }

    pub fn send_input(&mut self) {
        if !self.input.is_empty() {
            self.log.push(self.input.clone());
            self.input.clear();
        }
    }

    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }

    pub fn clear_log(&mut self) {
        self.log.clear();
    }
}
