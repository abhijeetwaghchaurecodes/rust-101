mod atm;
mod cash;
mod utils;
mod errors;

use std::io::{self, Write};
use cash::Cash;
use atm::ATM;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("== ATM Machine Simulation ==");

    let mut cash = Cash::new(1000);
    let mut atm = ATM::new(1, &mut cash);
    atm.print_memory_segment();

    loop {
        println!("\n----- ATM MENU -----");
        println!("1. Show Balance");
        println!("2. Withdraw Cash");
        println!("3. Insert (Deposit) Cash");
        println!("4. Print Memory Info");
        println!("5. Exit");
        println!("---------------------");

        let choice = read_input("Enter your choice: ");

        match choice.as_str() {
            "1" => atm.show_balance(),
            "2" => {
                let amount_input = read_input("Enter amount to withdraw: ");
                match amount_input.parse::<u32>() {
                    Ok(amount) => {
                        if let Err(e) = atm.perform_transaction(amount) {
                            println!("[Error] {:?}", e);
                        }
                    }
                    Err(_) => println!("[Error] Invalid amount entered."),
                }
            }
            "3" => {
                let deposit_input = read_input("Enter amount to deposit: ");
                match deposit_input.parse::<u32>() {
                    Ok(amount) => {
                        if let Err(e) = atm.credit_cash(amount) {
                            println!("[Error] {:?}", e);
                        }
                    }
                    Err(_) => println!("[Error] Invalid amount entered."),
                }
            }
            "4" => atm.print_memory_segment(),
            "5" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("[Warning] Invalid choice, please try again."),
        }
    }

    println!("== ATM Simulation Ended ==");
}
