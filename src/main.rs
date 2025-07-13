use clap::{Error, Parser};

use std::{net::Ipv4Addr, path::PathBuf};

mod addr2line;
mod commands;
mod config;
mod output_writer;
mod socket;

use commands::{Cli, Command};
use config::ConnectionConfig;
use output_writer::OutputWriter;
use socket::Socket;

fn connect_to_target(address: (Ipv4Addr, u16), file: Option<PathBuf>) -> Result<(), Error> {
    println!("Connecting to target at {}...", address.0);
    let mut socket = Socket::new(address)?;

    clearscreen::clear().expect("Failed to clear the screen.");

    let mut file = OutputWriter::new(file)?;

    loop {
        match socket.read() {
            Ok(data_read) => {
                if data_read.is_empty() {
                    break;
                }

                file.write(data_read)?;
            }
            Err(e) => {
                eprintln!("Failed to read from the socket: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let mut config = ConnectionConfig::load();

    let args = Cli::parse();

    match args.command {
        Command::Add { name, address } => {
            config.add_connection(&name, address)?;
            println!("Connection '{}' added for target '{}'.", name, address);
        }
        Command::Remove { name } => {
            if config.remove_connection(&name)? {
                println!("Connection '{}' removed.", name);
            } else {
                eprintln!("No connection found with the name '{}'.", name);
            }
        }
        Command::OpenConfig => {
            let _ = opener::reveal(&ConnectionConfig::get_filepath()?);
        }
        Command::List => config.list_connections(),
        Command::Connect { target, file } => {
            if let Some(address) = config.resolve_target(&target) {
                if connect_to_target(address, file).is_err() {
                    eprintln!("Failed to connect to the target.");
                }
            } else {
                eprintln!("No connection found for target '{}'.", target);
            }
        }
        Command::Addr2line {
            filepath,
            addresses,
        } => {
            addr2line::run(&filepath, addresses);
        }
    }

    Ok(())
}
