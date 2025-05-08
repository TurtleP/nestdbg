
# nestdbg

`nestdbg` is a remote debugging tool for [LÖVE Potion](https://github.com/lovebrew/lovepotion) games, designed to streamline the development process by providing a command-line interface for managing and connecting to debug targets over a network.

#### Command Line

```
Remote debugging tool for LÖVE Potion games.

Usage: nestdbg.exe <COMMAND>

Commands:
  add          Add a new connection
  remove       Remove an existing connection [aliases: rm]
  open-config  Open the configuration file in the file browser
  list         List all connections
  connect      Connect to a target using an existing connection or IP address
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

> [!IMPORTANT] 
> The project is built using the rust programming language. Ensure that it is installed before trying to build the project.


**1. Clone the repository**

```bash
git clone https://github.com/TurtleP/nestdbg
cd nestdbg
```

**2. Build the project in release mode**
```bash
cargo build --release
```

**3. Install the binary locally**
```bash
cargo install --path .
```

Alternatively, install the tool through crates.io: 
```bash
cargo install nestdbg
```
