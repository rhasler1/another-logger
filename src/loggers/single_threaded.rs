use std::io::{Write, ErrorKind::*};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use chrono::prelude::*;
use super::{LogLevel, LogFormat};

#[derive(Clone)]
pub struct SingleThreadLogger {
    destination: PathBuf,
    log_format:  LogFormat,
}

impl SingleThreadLogger {
    pub fn new(
        destination: &str,
        log_format: LogFormat
    ) -> std::io::Result<Self> {
        
        let destination = PathBuf::from(destination);

        match File::create_new(destination.as_path()) {
            Ok(_)       => {}
            Err(err)    => { match err.kind() {
                AlreadyExists   => {}
                _               => { return Err(err)  }
            }}
        }

        Ok(Self {
            destination,
            log_format,
        })
    }

    pub fn write(
        &self,
        log_level: LogLevel,
        message: &str
    ) -> std::io::Result<()> {

        let time = Local::now();
        let level = log_level.to_str();
        let formatted = self.log_format.format(&time, level, message);

        let mut file = OpenOptions::new()
            .append(true)
            .open(self.destination.as_path())?;

        file.write_all(formatted.as_bytes())?;
        file.sync_data()?;

        Ok(())
    }

    pub fn clear(&self) -> std::io::Result<()> {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.destination.as_path())?;

        Ok(())
    }
}