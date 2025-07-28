mod booking;
mod plane;
mod error;

use booking::BookingSystem;
use plane::Plane;
use std::io::{self, Write};

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let plane = Plane::new("AI-101", 5);
    let system = BookingSystem::new(plane);

    loop {
        println!("\nCommands: book | cancel | show | exit");
        match input(">> ").as_str() {
            "book" => {
                let name = input("Passenger Name: ");
                match system.book_seat(name) {
                    Ok(seat) => println!("✅ Seat {} booked!", seat.number),
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
            "cancel" => {
                let seat = input("Seat Number to Cancel: ");
                match seat.parse::<usize>() {
                    Ok(num) => match system.cancel_booking(num) {
                        Ok(_) => println!("✅ Seat {} cancelled.", num),
                        Err(e) => println!("❌ Error: {}", e),
                    },
                    Err(_) => println!("❌ Invalid number"),
                }
            }
            "show" => {
                system.show_seats();
            }
            "exit" => break,
            _ => println!("Unknown command."),
        }
    }
}
