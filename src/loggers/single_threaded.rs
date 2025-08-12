use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::ErrorKind::*;
use anyhow::{Result};
use chrono::prelude::*;
use super::LogLevel;

pub struct SingleThreadLogger {
    destination: PathBuf,
}

impl SingleThreadLogger {
    pub fn new(destination: PathBuf) -> Result<Self> {
        match File::create_new(destination.as_path()) {
            Ok(_)       => {}
            Err(err)    => { match err.kind() {
                AlreadyExists => {}
                _ => { anyhow::bail!("Logger creation error") }
            }}
        }

        Ok(Self {
            destination,
        })
    }

    pub fn write(&self, log_level: LogLevel, s: &str) -> Result<()> {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let message = format!("{} {}: {}\n", time, log_level.prefix(), s);

        let mut file = OpenOptions::new()
            .append(true)
            .open(self.destination.as_path())?;

        file.write_all(message.as_bytes())?;
        file.sync_data()?;

        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.destination.as_path())?;

        Ok(())
    }
}