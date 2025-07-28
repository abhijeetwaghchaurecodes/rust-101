mod account;
mod transaction;
mod error;

use crate::account::Account;
use crate::transaction::{Transaction, process_transaction};
use std::collections::HashMap;
use std::io;

fn main() {
    let mut accounts: HashMap<u32, Account> = HashMap::new();

    accounts.insert(1, Account::new(1, 5000.0));
    accounts.insert(2, Account::new(2, 3000.0));

    loop {
        println!("\n--- Transaction Menu ---");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Transfer");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let (id, amt) = get_account_and_amount();
                let txn = Transaction::Deposit { account_id: id, amount: amt };
                handle_transaction(txn, &mut accounts);
            }
            "2" => {
                let (id, amt) = get_account_and_amount();
                let txn = Transaction::Withdraw { account_id: id, amount: amt };
                handle_transaction(txn, &mut accounts);
            }
            "3" => {
                let mut from = String::new();
                let mut to = String::new();
                let mut amt = String::new();

                println!("From Account ID: ");
                io::stdin().read_line(&mut from).unwrap();

                println!("To Account ID: ");
                io::stdin().read_line(&mut to).unwrap();

                println!("Amount: ");
                io::stdin().read_line(&mut amt).unwrap();

                let from_id = from.trim().parse().unwrap_or(0);
                let to_id = to.trim().parse().unwrap_or(0);
                let amount = amt.trim().parse().unwrap_or(0.0);

                let txn = Transaction::Transfer {
                    from_id,
                    to_id,
                    amount,
                };
                handle_transaction(txn, &mut accounts);
            }
            "4" => break,
            _ => println!("Invalid option."),
        }
    }
}

fn handle_transaction(txn: Transaction, accounts: &mut HashMap<u32, Account>) {
    match process_transaction(txn, accounts) {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_account_and_amount() -> (u32, f64) {
    let mut id = String::new();
    let mut amt = String::new();

    println!("Account ID: ");
    io::stdin().read_line(&mut id).unwrap();

    println!("Amount: ");
    io::stdin().read_line(&mut amt).unwrap();

    let account_id = id.trim().parse().unwrap_or(0);
    let amount = amt.trim().parse().unwrap_or(0.0);
    (account_id, amount)
}
