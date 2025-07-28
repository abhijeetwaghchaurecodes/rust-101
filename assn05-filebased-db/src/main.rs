
mod record;
mod store;
mod utils;

use crate::store::KeyValueStore;
use std::io::{self, Write};

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() -> std::io::Result<()> {
    println!("File‑Based KV Store Starting");

    let mut store = KeyValueStore::new("kv.log")?;

    loop {
        println!("\nCommands: insert | get | dump | exit");
        let cmd = read("> ");
        match cmd.as_str() {
            "insert" => {
                let key = read("key: ");
                let value = read("value: ");
                store.insert(key, value)?;
            }
            "get" => {
                let key = read("key: ");
                match store.get(&key) {
                    Some(rec) => println!("found: {}", rec),
                    None => println!("key not found."),
                }
            }
            "dump" => {
                println!("all Records in memory:");
                for boxed in store.all_records() {
                    println!("  {}", boxed);
                }
            }
            "exit" => {
                println!("exiting…");
                break;
            }
            _ => {
                println!("unknown command.");
            }
        }
    }
    
    Ok(())
}
