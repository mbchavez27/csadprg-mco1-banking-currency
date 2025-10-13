use core::num;
use indexmap::IndexMap;
use std::io::{self, Write};

/// Represents a type of currency with its ID, name, and exchange rate.
struct CurrencyType {
    id: String,
    name: String,
    exchange_rate: f64,
}

/// Represents a bank account with a name, balance, and associated currency ID.
struct BankAccount {
    name: String,
    balance: f64,
    currency_id: String,
}

/// Displays the main transaction menu to the user.
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

/// Prompts the user to return to the main menu.
///
/// # Returns
/// - `true` if the user inputs 'Y' or 'y'.
/// - `false` otherwise.
fn return_menu() -> bool {
    let mut input = String::new();
    print!("Back to the Main Menu (Y/N): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_uppercase().as_str(), "Y")
}

/// Registers or sets the account name.
///
/// # Arguments
/// * `account_name` - A mutable reference to the account name string.
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

/// Handles depositing money into the account.
///
/// # Arguments
/// * `account_name` - The account holder's name.
/// * `currency_id` - The ID of the account's currency.
/// * `account_balance` - Mutable reference to the account balance.
fn deposit_account(account_name: String, currency_id: &str, account_balance: &mut f64) {
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

        println!("Updated Balance: {:.2}", account_balance);
        println!();

        if return_menu() {
            println!();
            break;
        }
    }
}

/// Handles withdrawing money from the account.
///
/// # Arguments
/// * `account_name` - The account holder's name.
/// * `currency_id` - The ID of the account's currency.
/// * `account_balance` - Mutable reference to the account balance.
fn withdraw_amount(account_name: String, currency_id: &str, account_balance: &mut f64) {
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
                println!("[ERROR]Invalid input: Please enter a valid number.\n");
                continue;
            }
        };

        if withdraw_amount > *account_balance {
            println!("[ERROR] Cannot withdraw more than the current balance.\n");
            continue;
        }

        *account_balance -= withdraw_amount;

        println!("Updated Balance: {:.2}", account_balance);
        println!();

        if return_menu() {
            println!();
            break;
        }
    }
}

/// Allows the user to record or update exchange rates for currencies.
///
/// # Arguments
/// * `currencies` - A mutable reference to the `IndexMap` of currencies.
fn record_exchange_rate(currencies: &mut IndexMap<String, CurrencyType>) {
    loop {
        let mut currency_input = String::new();
        let mut rate_input = String::new();
        println!("Record Exchange Rate");
        println!();

        for (i, (key, value)) in currencies.iter_mut().enumerate() {
            println!("[{}] {} ({})", i + 1, value.name, key);
        }

        println!();
        print!("Select Foreign Currency (input the ex: PHP): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut currency_input).unwrap();
        let currency_code = currency_input.trim().to_uppercase();

        if currencies.contains_key(&currency_code) {
            print!("Input Exchange Rate: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut rate_input)
                .expect("[ERROR] Failed to read input");

            let rate_amount: f64 = rate_input
                .trim()
                .parse()
                .expect("[ERROR] Invalid input: Please enter a valid floating-point number.");

            if let Some(currency) = currencies.get_mut(&currency_code) {
                currency.exchange_rate = rate_amount;
                println!("New Exchange Rate: {}", currency.exchange_rate);
            }
            println!();
        } else {
            println!("[ERROR] {} is not a valid currency", currency_input);
            println!();
        }

        if return_menu() {
            println!();
            break;
        }
    }
}

/// Handles currency exchange between available currencies.
///
/// Prompts the user to select a source and target currency from the provided list,
/// then computes and displays the exchanged amount based on their exchange rates.
/// Runs repeatedly until the user chooses to return to the main menu.
///
/// # Parameters
/// * `currencies` - Mutable reference to an `IndexMap<String, CurrencyType>`
///   containing currency codes, names, and their exchange rates.
///
/// # Output
/// Displays a menu of available currencies, exchange results, or error messages
/// for invalid currency codes or inputs.
///
/// # Panics
/// If user input for amounts is invalid or I/O operations fail.
fn currency_exchange(currencies: &mut IndexMap<String, CurrencyType>) {
    loop {
        let mut foreign_currency_input = String::new();
        let mut exchange_currency_input = String::new();
        let mut foreign_amount_input = String::new();

        println!("Foreign Currency Exchange");
        println!("Source Currency Option:");

        for (i, (key, value)) in currencies.iter().enumerate() {
            println!("[{}] {} ({})", i + 1, value.name, key);
        }

        print!("\nSelect Source Currency (input the ex: PHP): ");
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

            print!("\nSelect Exchange Currency (input the ex: PHP): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut exchange_currency_input).unwrap();
            let exchange_currency_code = exchange_currency_input.trim().to_uppercase();

            if let Some(target_currency) = currencies.get(&exchange_currency_code) {
                let exchange_amount = foreign_rate_amount * source_currency.exchange_rate
                    / target_currency.exchange_rate;
                println!("Exchange Amount: {:.2}", exchange_amount);
                println!();
            } else {
                println!(
                    "[ERROR] {} is not a valid currency",
                    exchange_currency_input
                );
                println!();
            }
        } else {
            println!("[ERROR] {} is not a valid currency", foreign_currency_input);
            println!();
        }

        if return_menu() {
            println!();
            break;
        }
    }
}

/// Displays daily compounded interest for a given bank account.
///
/// Uses a fixed 5% annual rate and computes daily interest as:
/// `interest = balance * (RATE / 365.0)`, updating the balance each day.
/// The function runs in a loop until the user returns to the main menu.
///
/// # Parameters
/// * `account` - Reference to a `BankAccount` containing `name`, `balance`, and `currency_id`.
///
/// # Output
/// Prints a table of daily interest and balance updates.
///
/// # Panics
/// If user input for days is invalid or I/O fails.
fn interest_amount(account: &BankAccount) {
    loop {
        const RATE: f64 = 0.05; //Constant 5% Interest Rate
        let mut num_days_input = String::new();

        println!("Show Interst Amount: ");
        println!("Account Name: {}", account.name);
        println!("Current Balance: {:.2} ", account.balance);
        println!("Current: {} ", account.currency_id);
        println!("Interest Rate: {}%", RATE * 100.0);

        println!();
        print!("Total Number of Days: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut num_days_input)
            .expect("Failed to read input");

        let days: i32 = num_days_input
            .trim()
            .parse()
            .expect("Please enter a valid integer");

        println!();

        println!("{:<3} | {:<8} | {:<8} |", "Day", "Interest", "Balance");

        let mut balance: f64 = account.balance;
        let mut interest_amount: f64;

        for day in 1..=days {
            interest_amount = balance * (RATE / 365.0);
            balance += interest_amount;

            let display_interest = (interest_amount * 100.0).round() / 100.0;
            let display_balance = (balance * 100.0).round() / 100.0;

            println!(
                "{:<3} | {:<8.2} | {:<8.2} |",
                day, display_interest, display_balance
            );
        }

        println!();
        if return_menu() {
            println!();
            break;
        }
    }
}

/// Main entry point of the banking application.
/// Initializes currencies, the user account, and handles user transactions.
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
                    println!("[ERROR] You already have an existing account!");
                    println!();
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
                    println!("[ERROR] You dont have an existing account!");
                    println!();
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
                    println!("[ERROR] You dont have an existing account!");
                    println!();
                }
            }
            "4" => {
                currency_exchange(&mut currencies);
            }
            "5" => {
                record_exchange_rate(&mut currencies);
            }
            "6" => {
                interest_amount(&account);
            }
            "7" => break,
            _ => {
                println!("[ERROR] Invalid choice!");
                println!();
            }
        }
    }
}
