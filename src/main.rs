mod models;
mod services;
mod ui;

use indexmap::IndexMap;
use std::io::Write;

use models::{BankAccount, CurrencyType};
use services::{
    account_services::{deposit_account, register_account, withdraw_amount},
    currency_services::{currency_exchange, record_exchange_rate},
    interest_services::interest_amount,
};

use ui::menu::print_menu;

fn main() {
    let mut currencies: IndexMap<String, CurrencyType> = IndexMap::from([
        (
            "PHP".to_string(),
            CurrencyType {
                id: "PHP".to_string(),
                name: "Philippine Peso".to_string(),
                exchange_rate: 1.0,
            },
        ),
        (
            "USD".to_string(),
            CurrencyType {
                id: "USD".to_string(),
                name: "United States Dollar".to_string(),
                exchange_rate: 0.0,
            },
        ),
        (
            "JPY".to_string(),
            CurrencyType {
                id: "JPY".to_string(),
                name: "Japanese Yen".to_string(),
                exchange_rate: 0.0,
            },
        ),
        (
            "GBP".to_string(),
            CurrencyType {
                id: "GBP".to_string(),
                name: "British Pound Sterling".to_string(),
                exchange_rate: 0.0,
            },
        ),
        (
            "EUR".to_string(),
            CurrencyType {
                id: "EUR".to_string(),
                name: "Euro".to_string(),
                exchange_rate: 0.0,
            },
        ),
        (
            "CNY".to_string(),
            CurrencyType {
                id: "CNY".to_string(),
                name: "Chinese Yuan Renminbi".to_string(),
                exchange_rate: 0.0,
            },
        ),
    ]);

    let mut account = BankAccount {
        name: String::new(),
        balance: 0.0,
        currency_id: String::from("PHP"),
    };

    loop {
        print_menu();
        print!("Choose Action: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        println!();

        match input {
            "1" => {
                if account.name.is_empty() {
                    register_account(&mut account.name);
                } else {
                    println!("[ERROR] You already have an existing account!\n");
                }
            }
            "2" => {
                if !account.name.is_empty() {
                    deposit_account(
                        account.name.clone(),
                        &account.currency_id,
                        &mut account.balance,
                    );
                } else {
                    println!("[ERROR] You don't have an existing account!\n");
                }
            }
            "3" => {
                if !account.name.is_empty() {
                    withdraw_amount(
                        account.name.clone(),
                        &account.currency_id,
                        &mut account.balance,
                    );
                } else {
                    println!("[ERROR] You don't have an existing account!\n");
                }
            }
            "4" => currency_exchange(&mut currencies),
            "5" => record_exchange_rate(&mut currencies),
            "6" => interest_amount(&account),
            "7" => break,
            _ => println!("[ERROR] Invalid choice!\n"),
        }
    }
}
