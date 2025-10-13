use crate::{models::BankAccount, ui::input::return_menu};
use std::io::{self, Write};

pub fn interest_amount(account: &BankAccount) {
    loop {
        const RATE: f64 = 0.05;
        let mut num_days_input = String::new();

        println!("Show Interest Amount:");
        println!("Account Name: {}", account.name);
        println!("Current Balance: {:.2}", account.balance);
        println!("Currency: {}", account.currency_id);
        println!("Interest Rate: {}%\n", RATE * 100.0);

        print!("Total Number of Days: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut num_days_input).unwrap();

        let days: i32 = num_days_input
            .trim()
            .parse()
            .expect("Please enter a valid integer");

        println!("\n{:<3} | {:<8} | {:<8} |", "Day", "Interest", "Balance");

        let mut balance: f64 = account.balance;
        for day in 1..=days {
            let interest_amount = balance * (RATE / 365.0);
            balance += interest_amount;

            println!("{:<3} | {:<8.2} | {:<8.2} |", day, interest_amount, balance);
        }

        println!();
        if return_menu() {
            println!();
            break;
        }
    }
}
