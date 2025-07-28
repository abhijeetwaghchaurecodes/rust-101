
use super::traits::Logger;

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[ConsoleLog] {}", message);
    }
}
