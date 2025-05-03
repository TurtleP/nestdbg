use std::fs::File;
use std::io::{Result, Write, stdout};
use std::path::PathBuf;

pub struct OutputWriter {
    file: Option<File>,
}

impl Drop for OutputWriter {
    fn drop(&mut self) {
        if let Some(ref mut file) = self.file {
            if let Err(e) = file.flush() {
                eprintln!("Failed to flush log file: {}", e);
            }
        }
    }
}

impl OutputWriter {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = path {
            let file = File::create(path)?;
            return Ok(OutputWriter { file: Some(file) });
        }
        Ok(OutputWriter { file: None })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        if let Some(ref mut file) = self.file {
            return file.write_all(data);
        }

        stdout().write_all(data)?;
        Ok(())
    }
}
