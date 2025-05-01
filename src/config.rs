use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub connections: Vec<Connection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub name: String,
    pub address: String,
}

const CONFIG_FILE: &str = "config.toml";

fn get_filepath() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "lovebrew", "nestdbg") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).map_err(|e| Error::new(ErrorKind::Other, e))?;
        return Ok(config_dir.join(CONFIG_FILE));
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "Could not find project directories",
    ))
}

pub fn load_config() -> ConnectionConfig {
    let path = match get_filepath() {
        Ok(path) => path,
        Err(_) => return ConnectionConfig::default(),
    };

    if !path.exists() {
        return ConnectionConfig::default();
    }

    if let Ok(content) = fs::read_to_string(path) {
        match toml::from_str::<ConnectionConfig>(&content) {
            Ok(mut config) => {
                config.connections.sort_by(|a, b| a.name.cmp(&b.name));
                return config;
            }
            Err(_) => return ConnectionConfig::default(),
        };
    } else {
        return ConnectionConfig::default();
    }
}

impl ConnectionConfig {
    pub fn get_connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn get_connection(&self, index: usize) -> Option<&Connection> {
        self.connections.get(index)
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn save(&self) -> Result<()> {
        let path = get_filepath()?;
        let content = toml::to_string(self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        fs::write(path, content).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}
