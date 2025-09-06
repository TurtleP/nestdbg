use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use which::which;

#[derive(Debug)]
struct Candidate {
    binary: &'static str,
    magic: &'static [u8],
    args: &'static [&'static str],
    search_path: &'static str,
}

impl Candidate {
    fn matches(&self, filepath: &Path) -> bool {
        if self.magic.is_empty() {
            return false;
        }

        let file = match File::open(filepath) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut reader = BufReader::new(file);

        const CHUNK: usize = 8 * 1024;
        let mut buffer = vec![0u8; CHUNK];
        let mut overlap: Vec<u8> = Vec::new();
        let keep = self.magic.len().saturating_sub(1);

        loop {
            let n = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => return false,
            };
            let mut window = Vec::with_capacity(overlap.len() + n);
            window.extend_from_slice(&overlap);
            window.extend_from_slice(&buffer[..n]);
            if window.windows(self.magic.len()).any(|w| w == self.magic) {
                return true;
            }
            overlap.clear();
            let total = window.len();
            if keep > 0 && total >= keep {
                overlap.extend_from_slice(&window[total - keep..]);
            } else if total < keep {
                overlap.extend_from_slice(&window);
            }
        }
        false
    }

    fn command(&self, filepath: &Path, addresses: &[String]) {
        let binary_path = Path::new(self.search_path).join(self.binary);
        if which(&binary_path).is_err() {
            eprintln!("Could not find {} in {}", self.binary, self.search_path);
            return;
        }

        let mut command = Command::new(binary_path);
        command
            .arg("-aipfCe")
            .arg(self.args.join(" "))
            .arg("-e")
            .arg(filepath);

        for addr in addresses {
            command.arg(addr);
        }

        match command.output() {
            Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
            Err(e) => eprintln!("Failed to execute {}: {}", self.binary, e),
        }
    }
}

static CANDIDATES: &[Candidate] = &[
    Candidate {
        binary: "arm-none-eabi-addr2line",
        magic: b"3dsx_ctr0.o",
        args: &["arm"],
        #[cfg(target_os = "windows")]
        search_path: "C:\\devkitPro\\devkitARM\\bin",
        #[cfg(not(target_os = "windows"))]
        search_path: "/opt/devkitpro/devkitARM/bin",
    },
    Candidate {
        binary: "aarch64-none-elf-addr2line",
        magic: b"switch_crt0.o",
        args: &[""],
        #[cfg(target_os = "windows")]
        search_path: "C:\\devkitPro\\devkitA64\\bin",
        #[cfg(not(target_os = "windows"))]
        search_path: "/opt/devkitpro/devkitA64/bin",
    },
    Candidate {
        binary: "powerpc-eabi-addr2line",
        magic: b"crt0_rpx.o",
        args: &[""],
        #[cfg(target_os = "windows")]
        search_path: "C:\\devkitPro\\devkitPPC\\bin",
        #[cfg(not(target_os = "windows"))]
        search_path: "/opt/devkitpro/devkitPPC/bin",
    },
];

pub fn run(filepath: &PathBuf, addresses: Vec<String>) {
    match CANDIDATES.iter().find(|c| c.matches(filepath)) {
        Some(candidate) => candidate.command(filepath, &addresses),
        None => println!(
            "No suitable addr2line found for the given file: {}",
            filepath.display()
        ),
    }
}
