use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpSocket, TcpStream},
    sync::mpsc::Sender,
};

use std::net::SocketAddr;

use crate::config::Connection;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NetworkStatus {
    Connected,
    Disconnected,
    Connecting,
    Error(String),
}

#[derive(Debug)]
pub struct NetworkManager {
    address: Option<SocketAddr>,
    socket: Option<TcpStream>,
    peer_name: String,
    status: NetworkStatus,

    spinner_index: usize,
}

const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            address: None,
            socket: None,
            peer_name: String::new(),
            status: NetworkStatus::Disconnected,
            spinner_index: 0,
        }
    }

    pub async fn connect(&mut self, connection: &Connection) -> Result<(), String> {
        self.peer_name = connection.name.clone();

        if let Ok(address) = format!("{}:{}", connection.address, 8000).parse() {
            self.address = Some(address);
            self.spinner_index = 0;

            self.status = NetworkStatus::Connecting;
        } else {
            return Err("Invalid address format".to_string());
        }

        Ok(())
    }

    pub async fn send(&mut self, data: String) -> Result<(), String> {
        if let Some(ref mut socket) = self.socket {
            match socket.write_all(data.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to send data: {}", e)),
            }
        } else {
            Err("Socket is not connected".to_string())
        }
    }

    pub async fn receive(&mut self) -> Result<String, String> {
        if let Some(ref mut socket) = self.socket {
            let mut buffer = vec![0; 1024]; // Adjust buffer size as needed
            match socket.read(&mut buffer).await {
                Ok(size) if size > 0 => {
                    let data = String::from_utf8_lossy(&buffer[..size]).to_string();
                    Ok(data)
                }
                Ok(_) => Err("No data received".to_string()),
                Err(e) => Err(format!("Failed to read data: {}", e)),
            }
        } else {
            Err("Socket is not connected".to_string())
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        if let Some(mut socket) = self.socket.take() {
            match socket.shutdown().await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to disconnect: {}", e)),
            }
        } else {
            Err("Socket is not connected".to_string())
        }
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_some() && self.status == NetworkStatus::Connected
    }

    fn get_ip(&self) -> Option<String> {
        if let Some(address) = self.address {
            return Some(address.ip().to_string());
        }
        None
    }

    pub fn get_address(&mut self) -> String {
        let address = self.get_ip();

        match self.status {
            NetworkStatus::Connected => {
                if let Some(addr) = address {
                    return format!("{} / {}", self.peer_name, addr);
                } else {
                    return String::from("Address not available");
                }
            }
            NetworkStatus::Disconnected => return String::from("Disconnected"),
            NetworkStatus::Connecting => {
                self.spinner_index += 1;
                return format!(
                    "{} Connecting to {}...",
                    SPINNER_FRAMES[self.spinner_index % SPINNER_FRAMES.len()],
                    self.peer_name
                );
            }
            NetworkStatus::Error(_) => return String::from("Disconnected"),
        }
    }
}
