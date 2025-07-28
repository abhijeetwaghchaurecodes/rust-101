mod models;
mod errors;
mod auth;
mod wallet;
mod logger;

use models::*;
use logger::traits::Logger;
use logger::console_logger::ConsoleLogger;
use wallet::*;
use auth::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let logger: Arc<dyn Logger> = Arc::new(ConsoleLogger);
    let mut users: HashMap<String, Arc<Mutex<UserAccount>>> = HashMap::new();

    // Create dummy users
    users.insert(
        "abhi".into(),
        Arc::new(Mutex::new(UserAccount::new("abhi".into(), "1234".into(), 500.0))),
    );
    users.insert(
        "jeet".into(),
        Arc::new(Mutex::new(UserAccount::new("jeet".into(), "abcd".into(), 300.0))),
    );

    println!("Welcome to Rust Wallet");

    loop {
        println!("\nAvailable commands: login | exit");
        let cmd = read("Enter command: ");

        match cmd.as_str() {
            "login" => {
                let username = read("Username: ");
                let password = read("Password: ");

                let user = match users.get(&username) {
                    Some(u) => u.clone(),
                    None => {
                        println!("User not found.");
                        continue;
                    }
                };

                let guard = user.lock().unwrap();
                if let Err(e) = authenticate(&guard, &password) {
                    println!("Auth failed: {}", e);
                    continue;
                }
                drop(guard); // release lock

                println!("Logged in as {}", username);

                loop {
                    println!("\nCommands: balance | transfer | history | logout");
                    let cmd = read("> ");

                    if cmd == "logout" {
                        println!("Logging out...");
                        break;
                    }

                    let mut user_lock = user.lock().unwrap();

                    match cmd.as_str() {
                        "balance" => {
                            let bal = check_balance(&user_lock);
                            logger.log(&format!("Balance for {}: ${}", username, bal));
                        }

                        "transfer" => {
                            let target = read("Transfer to: ");
                            let amt = read("Amount: ").parse::<f64>().unwrap_or(0.0);

                            if let Some(tgt_user) = users.get(&target) {
                                let mut tgt_lock = tgt_user.lock().unwrap();

                                match transfer_funds(&mut user_lock, &mut tgt_lock, amt) {
                                    Ok(_) => {
                                        logger.log(&format!(
                                            "Transfer: ${} from {} to {}",
                                            amt, username, target
                                        ));
                                    }
                                    Err(e) => println!("Transfer failed: {}", e),
                                }
                            } else {
                                println!("Target user not found.");
                            }
                        }

                        "history" => {
                            for txn in user_lock.get_history() {
                                println!("Sent ${} to {}", txn.amount, txn.to);
                            }
                        }

                        _ => println!("Unknown command."),
                    }
                }
            }

            "exit" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid command."),
        }
    }
}
