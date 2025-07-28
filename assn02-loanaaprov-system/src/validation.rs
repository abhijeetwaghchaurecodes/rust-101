
use std::io::{self, Write};

pub fn read_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        }
    }
}

pub fn read_optional(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
