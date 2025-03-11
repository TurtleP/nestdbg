use std::fs::File;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::path::PathBuf;

use clap::{Error, Parser};

use crate::config::NiceNames;
mod config;

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true)]
struct Args {
    #[arg(short, long, help = "IP address or name of the target")]
    target: Option<String>,

    #[arg(short, long, help = "Output file name")]
    filename: Option<PathBuf>,

    #[arg(short, long, help = "Optional nice name to save or update")]
    name: Option<String>,

    #[arg(short, long, help = "List all connections")]
    list_connections: bool,
}

fn is_ipv4(address: &str) -> bool {
    address.parse::<Ipv4Addr>().is_ok()
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut nice_names = NiceNames::load()?;

    if args.list_connections {
        nice_names.list_names();
        return Ok(());
    }

    let mut address: String = String::new();

    if let Some(target) = args.target {
        address = match nice_names.resolve_name(&target) {
            Some(address) => address,
            None => {
                // Check if the target is a valid IPv4 address before adding it
                if !is_ipv4(&target) {
                    eprintln!("Invalid IPv4 address: {}.", target);
                    return Ok(()); // Return an error here if needed
                }

                // Add the name and address and save it
                nice_names.add_name(args.name.as_deref(), &target)?;
                nice_names.save()?;
                target
            }
        };
    }

    if !is_ipv4(&address) {
        return Ok(());
    }

    let connection = (address, 8000);
    println!("Connecting to: {:?}.", connection);

    if let Ok(mut socket) = TcpStream::connect(connection) {
        clearscreen::clear().expect("Failed to clear screen.");
        let mut buffer = [0; 0x1000];

        let mut file: Option<File> = None;
        if let Some(filename) = args.filename {
            file = Some(File::create(filename)?);
        }

        loop {
            if let Ok(bytes_read) = socket.read(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }

                let bytes = &buffer[..bytes_read];

                match file {
                    Some(ref mut output) => output.write(bytes)?,
                    None => std::io::stdout().write(bytes)?,
                };
            } else {
                eprintln!("Failed to read from the socket.");
                break;
            }
        }
    } else {
        eprintln!("Failed to connect to the target.");
    }

    Ok(())
}
