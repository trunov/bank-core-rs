// use bank_system::balance::balance_manager::BalanceManager;
// use bank_system::users::user_manager::UserManager;
use bank_system::Name;
use bank_system::storage::Storage;
use std::env;

fn main() {
    let mut storage = Storage::load_data("balance.csv");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  deposit <name> <amount>");
        eprintln!("  withdraw <name> <amount>");
        eprintln!("  balance <name>");
        return;
    }

    match args[1].as_str() {
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Wrong arguments. Example: deposit John 200");
                return;
            }

            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Sum must be digit");
            match storage.deposit(&name, amount) {
                Ok(()) => {
                    println!("Deposited: {} amount {}", name, amount);
                    storage.save("balance.csv");
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Wrong arguments. Example: add John 200");
                return;
            }

            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Sum must be digit");
            match storage.withdraw(&name, amount) {
                Ok(()) => {
                    println!("Withdrawn: {} amount {}", name, amount);
                    storage.save("balance.csv");
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        "balance" => {
            if args.len() != 3 {
                eprintln!("Wrong arguments. Example: balance Kirill");
                return;
            }
            let name: Name = args[2].clone();
            match storage.get_balance(&name) {
                Some(b) => println!("Balance {}: {}", name, b),
                None => println!("User {} has not been found", name),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
