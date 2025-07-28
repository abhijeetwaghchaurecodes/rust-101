mod logger;
mod panic_handler;

use logger::traits::Logger;
use logger::file_logger::FileLogger;
use logger::console_logger::ConsoleLogger;

fn main() {
    println!("File Logger - Safe Rust + Panic Handling");

    let use_file_logger = true;

    if use_file_logger {
        match FileLogger::new("app.log") {
            Ok(mut logger) => {
                logger.log("application started.");
                logger.log("logging info message.");
                // Uncomment below to simulate panic
                // panic_handler::simulate_unwind_mode();
                logger.log("application finished safely");
            }
            Err(e) => {
                eprintln!("[ERROR] failed to initialize FileLogger: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let mut logger = ConsoleLogger;
        logger.log("this logs to console.");
    }
}
