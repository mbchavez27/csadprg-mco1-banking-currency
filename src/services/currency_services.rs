use crate::{models::CurrencyType, ui::input::return_menu};
use indexmap::IndexMap;
use std::io::{self, Write};

pub fn record_exchange_rate(currencies: &mut IndexMap<String, CurrencyType>) {
    loop {
        let mut currency_input = String::new();
        let mut rate_input = String::new();
        println!("Record Exchange Rate\n");

        for (i, (key, value)) in currencies.iter_mut().enumerate() {
            println!("[{}] {} ({})", i + 1, value.name, key);
        }

        println!();
        print!("Select Foreign Currency (input ex: PHP): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut currency_input).unwrap();
        let currency_code = currency_input.trim().to_uppercase();

        if currencies.contains_key(&currency_code) {
            print!("Input Exchange Rate: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut rate_input).unwrap();

            let rate_amount: f64 = rate_input
                .trim()
                .parse()
                .expect("[ERROR] Invalid input: Please enter a valid number.");

            if let Some(currency) = currencies.get_mut(&currency_code) {
                currency.exchange_rate = rate_amount;
                println!("New Exchange Rate: {}\n", currency.exchange_rate);
            }
        } else {
            println!(
                "[ERROR] {} is not a valid currency\n",
                currency_input.trim()
            );
        }

        if return_menu() {
            println!();
            break;
        }
    }
}

pub fn currency_exchange(currencies: &mut IndexMap<String, CurrencyType>) {
    loop {
        let mut foreign_currency_input = String::new();
        let mut exchange_currency_input = String::new();
        let mut foreign_amount_input = String::new();

        println!("Foreign Currency Exchange");
        println!("Source Currency Option:");

        for (i, (key, value)) in currencies.iter().enumerate() {
            println!("[{}] {} ({})", i + 1, value.name, key);
        }

        print!("\nSelect Source Currency (ex: PHP): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut foreign_currency_input).unwrap();
        let foreign_currency_code = foreign_currency_input.trim().to_uppercase();

        if let Some(source_currency) = currencies.get(&foreign_currency_code) {
            print!("Input Source Amount: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut foreign_amount_input).unwrap();

            let foreign_rate_amount: f64 = foreign_amount_input
                .trim()
                .parse()
                .expect("[ERROR] Invalid input: Please enter a valid number.");

            println!("\nExchanged Currency Option:");
            for (i, (key, value)) in currencies.iter().enumerate() {
                println!("[{}] {} ({})", i + 1, value.name, key);
            }

            print!("\nSelect Exchange Currency (ex: PHP): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut exchange_currency_input).unwrap();
            let exchange_currency_code = exchange_currency_input.trim().to_uppercase();

            if let Some(target_currency) = currencies.get(&exchange_currency_code) {
                let exchange_amount = foreign_rate_amount * source_currency.exchange_rate
                    / target_currency.exchange_rate;
                println!("Exchange Amount: {:.2}\n", exchange_amount);
            } else {
                println!(
                    "[ERROR] {} is not a valid currency\n",
                    exchange_currency_input.trim()
                );
            }
        } else {
            println!(
                "[ERROR] {} is not a valid currency\n",
                foreign_currency_input.trim()
            );
        }

        if return_menu() {
            println!();
            break;
        }
    }
}
