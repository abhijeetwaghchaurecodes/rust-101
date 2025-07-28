
use super::traits::Logger;

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("[Console] {}", message);
    }
}
