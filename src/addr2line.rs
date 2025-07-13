use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::Command;

use which::which;

const ADDR2LINE_CANDIDATES: [(&str, &str, &[u8]); 3] = [
    ("arm-none-eabi-addr2line", "arm", b"3dsx_crt0.o"),
    ("aarch64-none-elf-addr2line", "", b"switch_crt0.o"),
    ("powerpc-eabi-addr2line", "", b"crt0_rpx.o"),
];

fn get_addr2line(filepath: &PathBuf) -> Result<(String, String)> {
    if !filepath.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found or not a file: {:?}", filepath),
        ));
    }

    let contents = std::fs::read(filepath)?;

    if contents.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("File is empty: {:?}", filepath),
        ));
    }

    for (binary, args, magic) in ADDR2LINE_CANDIDATES {
        if contents.windows(magic.len()).any(|window| window == magic) {
            if which(binary).is_ok() {
                return Ok((binary.to_string(), format!("-aipfCe {args} -e")));
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("{} not found", binary),
                ));
            }
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "No suitable addr2line binary found",
    ))
}

pub fn run(filepath: &PathBuf, addresses: Vec<String>) {
    let (program, args) = match get_addr2line(filepath) {
        Ok((program, args)) => (program, args),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut command = Command::new(program);
    command
        .args(args.split_whitespace())
        .arg(filepath)
        .args(addresses);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stdout.is_empty() {
                    println!("{}", stdout);
                }
                if !stderr.is_empty() {
                    eprintln!("{}", stderr);
                }
            } else {
                eprintln!("addr2line failed {output:?}");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute addr2line: {e}");
        }
    }
}
