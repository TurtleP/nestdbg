use std::net::Ipv4Addr;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Remote debugging tool for LÖVE Potion games.",
    author = "support@lovebrew.org"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new connection
    Add {
        #[arg(help = "Name to assign to this connection")]
        name: String,
        #[arg(help = "IP address of the target")]
        address: Ipv4Addr,
    },
    /// Remove an existing connection
    #[command(visible_alias = "rm", alias = "delete")]
    Remove {
        #[arg(help = "Name of the connection to remove")]
        name: String,
    },
    /// Open the configuration file in the file browser
    OpenConfig,
    /// List all connections
    List,
    /// Connect to a target using an existing connection or IP address
    Connect {
        #[arg(help = "IP address or name of the target")]
        target: String,
        #[arg(long, help = "File to write logging output to")]
        file: Option<PathBuf>,
    },
    /// Resolve exception addresses using a debug binary
    Addr2line {
        #[arg(help = "Path to the debug binary")]
        filepath: PathBuf,
        #[arg(help = "Exception addresses to resolve")]
        addresses: Vec<String>,
    },
}
