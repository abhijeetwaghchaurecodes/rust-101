
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::traits::Logger;

pub struct FileLogger {
    path: PathBuf,
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl FileLogger {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        let writer = BufWriter::new(file);

        Ok(Self {
            path: PathBuf::from(filename),
            writer: Arc::new(Mutex::new(writer)),
        })
    }
}

impl Logger for FileLogger {
    fn log(&mut self, message: &str) {
        let mut writer = self.writer.lock().unwrap();

        if let Err(e) = writeln!(writer, "{}", message) {
            eprintln!("[CRITICAL] Failed to write to log file: {:?}", e);
            panic!("Disk error or write failure: {}", e);
        }
    }
}
