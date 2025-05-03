use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tabled::settings::Style;
use tabled::{Table, Tabled};

use std::collections::BTreeMap;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub connections: BTreeMap<String, Ipv4Addr>,
}

#[derive(Tabled)]
struct ConnectionRow {
    name: String,
    address: Ipv4Addr,
}

const CONFIG_FILE_NAME: &str = "connections.toml";

impl ConnectionConfig {
    /// Load the connection configuration from the file system.
    /// If the file does not exist or is invalid, return a default configuration.
    /// The connections are sorted by name after loading.
    pub fn load() -> Self {
        let path = match Self::get_filepath() {
            Ok(path) => path,
            Err(_) => return ConnectionConfig::default(),
        };

        if !path.exists() {
            return ConnectionConfig::default();
        }

        if let Ok(content) = fs::read_to_string(path) {
            return toml::from_str::<ConnectionConfig>(&content).unwrap_or_default();
        }
        ConnectionConfig::default()
    }

    /// Get the file path for the connection configuration file.
    /// This will create the directory if it does not exist.
    pub fn get_filepath() -> Result<PathBuf> {
        match ProjectDirs::from("com", "lovebrew", "nestdbg") {
            Some(proj_dirs) => {
                let config_dir = proj_dirs.config_dir();
                fs::create_dir_all(config_dir)?;
                Ok(config_dir.join(CONFIG_FILE_NAME))
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Could not find project directories",
            )),
        }
    }

    /// List all connections in a table format.
    /// If no connections are found, it will print a message indicating that.
    pub fn list_connections(&self) {
        if self.connections.is_empty() {
            return println!("No connections found.");
        }

        let collection: Vec<ConnectionRow> = self
            .connections
            .iter()
            .map(|(name, &address)| ConnectionRow {
                name: name.clone(),
                address,
            })
            .collect();

        let mut table = Table::new(collection);
        table.with(Style::markdown());

        println!("{table}");
    }

    /// Resolve a target by name or IP address.
    pub fn resolve_target(&self, target: impl ToString) -> Option<(Ipv4Addr, u16)> {
        let connection = self
            .connections
            .iter()
            .find(|(name, _)| name.to_string() == target.to_string());

        /* grab the Ipv4Addr from Connection */
        if let Some(data) = connection {
            return Some((*data.1, 8000));
        }

        /* parse the address from the string */
        Some((target.to_string().parse().ok()?, 8000))
    }

    /// Add a new connection to the configuration.
    pub fn add_connection(&mut self, name: impl ToString, address: Ipv4Addr) -> Result<()> {
        self.connections.insert(name.to_string(), address);
        self.save()
    }

    /// Remove a connection by name.
    /// Returns `Ok(true)` if the connection was found and removed, `Ok(false)` if not found.
    pub fn remove_connection(&mut self, filter: &str) -> Result<bool> {
        if self.connections.iter().any(|(name, _)| filter == name) {
            self.connections.remove(filter);
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Save the current configuration to the file system.
    pub fn save(&self) -> Result<()> {
        let path = Self::get_filepath()?;

        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
        }

        match toml::to_string_pretty(self) {
            Ok(content) => fs::write(path, content),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
