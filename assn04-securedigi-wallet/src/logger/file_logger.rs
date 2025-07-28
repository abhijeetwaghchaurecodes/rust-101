
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

use super::traits::Logger;

pub struct FileLogger {
    file: Mutex<std::fs::File>,
}

impl FileLogger {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();
        Self {
            file: Mutex::new(file),
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        let mut f = self.file.lock().unwrap();
        writeln!(f, "{}", message).unwrap();
    }
}
