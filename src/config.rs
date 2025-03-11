use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct NiceNames {
    pub mappings: HashMap<String, String>,
}

const CONFIG_FILE: &str = "config.toml";

fn get_filepath() -> Result<PathBuf, std::io::Error> {
    if let Some(directory) = dirs::config_dir() {
        let config_path = directory.join(std::env!("CARGO_PKG_NAME"));

        if !config_path.exists() {
            std::fs::create_dir(&config_path)?;
        }

        return Ok(config_path.join(CONFIG_FILE));
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Directory does not exist",
    ))
}

impl NiceNames {
    /// Loads the mappings from the configuration file.
    ///
    /// If the file exists and contains valid TOML, the mappings are loaded into the `NiceNames` struct.
    /// If the file does not exist or the content is invalid, it returns a default `NiceNames` instance with empty mappings.
    ///
    /// ### Errors
    /// Returns an `std::io::Error` if the configuration directory cannot be accessed or created.
    pub fn load() -> Result<Self, std::io::Error> {
        let path = get_filepath()?;
        let mut content: String = String::new();

        if Path::new(&path).exists() {
            content = fs::read_to_string(&path)?;
        }

        match toml::from_str(&content) {
            Ok(value) => Ok(value),
            Err(_) => Ok(NiceNames::default()),
        }
    }

    /// Saves the current mappings to the configuration file in TOML format.
    ///
    /// ### Errors
    /// Returns an `std::io::Error` if the file cannot be written to or if serialization fails.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = get_filepath()?;

        let toml = toml::to_string_pretty(self).map_err(|err| {
            eprintln!("Failed to serialize config: {}", err);
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize config")
        })?;

        fs::write(path, toml)
    }

    /// Resolves a name into its corresponding IPv4 address.
    ///
    /// ### Arguments
    /// * `name` - A string slice representing the name to resolve.
    ///
    /// ### Returns
    /// Returns `Some<String>` with the IPv4 address if the name exists in the mappings,
    /// or `None` if the name is not found.
    pub fn resolve_name(&self, name: &str) -> Option<String> {
        self.mappings.get(name).cloned()
    }

    /// Adds a new mapping between a name and an IPv4 address.
    ///
    /// ### Arguments
    /// * `name` - An optional string slice representing the name to add or update.
    /// * `address` - A string slice representing the IPv4 address to associate with the name.
    ///
    /// If `name` is `None`, the mapping is ignored.
    ///
    /// ### Errors
    /// Returns an `std::io::Error` if saving the updated mappings to the file fails.
    pub fn add_name(&mut self, name: Option<&str>, address: &str) -> Result<(), std::io::Error> {
        if let Some(value) = name {
            self.mappings
                .insert(String::from(value), String::from(address));
        }
        Ok(())
    }

    /// Prints all mappings in a formatted, human-readable table.
    ///
    /// If no mappings exist, prints "No connections found."
    pub fn list_names(&self) {
        if self.mappings.is_empty() {
            println!("No connections found.");
            return;
        }

        let rows: Vec<_> = self
            .mappings
            .iter()
            .map(|(name, address)| {
                vec![
                    name.cell().justify(Justify::Left),
                    address.cell().justify(Justify::Left),
                ]
            })
            .collect();

        let table = rows
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "IPv4 Address".cell().bold(true),
            ])
            .border(cli_table::format::Border::builder().build());

        print_stdout(table).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::NiceNames;

    #[test]
    fn test_resolve_target_nice_name() {
        let mut names = NiceNames::default();
        names.add_name(Some("ctr"), "192.168.42.0").unwrap();

        let resolved = names.resolve_name("ctr");
        assert_eq!(resolved.unwrap(), "192.168.42.0");
    }

    #[test]
    fn test_none_name() {
        let mut names = NiceNames::default();
        names.add_name(None, "192.168.42.0").unwrap();

        let resolved = names.resolve_name("");
        assert_eq!(resolved, None);

        let empty_names = NiceNames::default();
        assert_eq!(empty_names, names);
    }

    #[test]
    fn test_non_existent_name() {
        let names = NiceNames::default();

        let resolved = names.resolve_name("newlima3dsxl");
        assert_eq!(resolved, None);
    }

    #[test]
    fn test_updating_name() {
        let mut names = NiceNames::default();

        let address = "192.168.30.1";

        names.add_name(Some("desktop"), &address).unwrap();
        names.add_name(Some("desktop"), "192.168.69.0").unwrap();

        let resolved = names.resolve_name("desktop");
        assert_eq!(resolved.unwrap(), "192.168.69.0");
    }
}
