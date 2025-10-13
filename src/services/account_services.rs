use crate::ui::input::return_menu;
use std::io::{self, Write};

pub fn register_account(account_name: &mut String) {
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

pub fn deposit_account(account_name: String, currency_id: &str, account_balance: &mut f64) {
    loop {
        let mut deposit_input = String::new();
        println!("Deposit Amount");
        println!("Account Name: {}", account_name);
        println!("Current Balance: {}", account_balance);
        println!("Currency: {}", currency_id);
        println!();

        print!("Deposit Amount: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut deposit_input)
            .expect("Failed to read input");

        let deposit_amount: f64 = deposit_input
            .trim()
            .parse()
            .expect("[ERROR] Invalid input: Please enter a valid floating-point number.");

        *account_balance += deposit_amount;

        println!("Updated Balance: {:.2}\n", account_balance);

        if return_menu() {
            println!();
            break;
        }
    }
}

pub fn withdraw_amount(account_name: String, currency_id: &str, account_balance: &mut f64) {
    loop {
        let mut withdraw_input = String::new();
        println!("Withdraw Amount");
        println!("Account Name: {}", account_name);
        println!("Current Balance: {:.2}", account_balance);
        println!("Currency: {}", currency_id);
        println!();

        print!("Withdraw Amount: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut withdraw_input)
            .expect("[ERROR] Failed to read input");

        let withdraw_amount: f64 = match withdraw_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Invalid input: Please enter a valid number.\n");
                continue;
            }
        };

        if withdraw_amount > *account_balance {
            println!("[ERROR] Cannot withdraw more than the current balance.\n");
            continue;
        }

        *account_balance -= withdraw_amount;

        println!("Updated Balance: {:.2}\n", account_balance);

        if return_menu() {
            println!();
            break;
        }
    }
}
