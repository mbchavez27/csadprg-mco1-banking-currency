use std::io::{self, Write};

struct BankAccount {
    name: String,
    balance: f64,
}

fn print_menu() {
    println!("Select Transaction:");
    println!("[1] Register Account Name");
    println!("[2] Deposit Amount");
    println!("[3] Withdraw Amount");
    println!("[4] Currency Exchange");
    println!("[5] Record Exchange Rates");
    println!("[6] Show Interest Computation");
    println!("[7] Exit");
    println!();
}

fn return_menu() -> bool {
    let mut input = String::new();
    print!("Back to the Main Menu (Y/N): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_uppercase().as_str(), "Y")
}

fn register_account(account_name: &mut String) {
    loop {
        println!("Register Account Name");
        print!("Account Name: ");
        io::stdout().flush().unwrap();

        account_name.clear();
        io::stdin().read_line(account_name).unwrap();
        *account_name = account_name.trim().to_string();
        println!();

        if return_menu() {
            println!();
            break;
        }
    }
}

fn main() {
    let mut account = BankAccount {
        name: String::new(),
        balance: 0.0,
    };

    loop {
        print_menu();
        print!("Choose Action: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        println!();

        match input {
            "1" => {
                if account.name.is_empty() {
                    register_account(&mut account.name);
                } else {
                    println!("You already have an existing account!");
                    println!();
                }
            }
            "7" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
