use std::io::{self, Write};

struct CurrencyType {
    id: String,
    name: String,
    exchange_rate: f64,
}

struct BankAccount {
    name: String,
    balance: f64,
    currency: CurrencyType,
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
/// * `account_currency` - The currency of the account.
/// * `account_balance` - Mutable reference to the account balance.
fn deposit_account(
    account_name: String,
    account_currency: &CurrencyType,
    account_balance: &mut f64,
) {
    loop {
        let mut deposit_input = String::new();
        println!("Deposit Amount");
        println!("Account Name: {}", account_name);
        println!("Current Balance: {}", account_balance);
        println!("Currency: {}", account_currency.id);
        println!();

        print!("Deposit Amount: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut deposit_input)
            .expect("Failed to read input");

        let deposit_amount: f64 = deposit_input
            .trim()
            .parse()
            .expect("Invalid input: Please enter a valid floating-point number.");

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
/// * `account_currency` - The currency of the account.
/// * `account_balance` - Mutable reference to the account balance.
fn withdraw_amount(
    account_name: String,
    account_currency: &CurrencyType,
    account_balance: &mut f64,
) {
    loop {
        let mut withdraw_input = String::new();
        println!("Withdraw Amount");
        println!("Account Name: {}", account_name);
        println!("Current Balance: {:.2}", account_balance);
        println!("Currency: {}", account_currency.id);
        println!();

        print!("Withdraw Amount: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut withdraw_input)
            .expect("Failed to read input");

        let withdraw_amount: f64 = match withdraw_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input: Please enter a valid number.\n");
                continue;
            }
        };

        if withdraw_amount > *account_balance {
            println!("Cannot withdraw more than the current balance.\n");
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

/// Main entry point of the banking application.
/// Initializes the account and manages the transaction loop.
fn main() {
    let php_currency = CurrencyType {
        id: String::from("PHP"),
        name: String::from("Philippine Peso"),
        exchange_rate: 1.0, // base currency rate
    };

    let usd_currency = CurrencyType {
        id: String::from("USD"),
        name: String::from("United States Dollar"),
        exchange_rate: 0.0,
    };

    let jpy_currency = CurrencyType {
        id: String::from("JPY"),
        name: String::from("Japanese Yen"),
        exchange_rate: 0.0,
    };

    let gbp_currency = CurrencyType {
        id: String::from("GBP"),
        name: String::from("British Pound Sterling"),
        exchange_rate: 0.0,
    };

    let eur_currency = CurrencyType {
        id: String::from("EUR"),
        name: String::from("Euro"),
        exchange_rate: 0.0,
    };

    let cny_currency = CurrencyType {
        id: String::from("CNY"),
        name: String::from("Chinese Yuan Renminni"),
        exchange_rate: 0.0,
    };

    let mut account = BankAccount {
        name: String::new(),
        balance: 0.0,
        currency: php_currency,
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
                        &account.currency,
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
                        &account.currency,
                        &mut account.balance,
                    );
                } else {
                    println!("[ERROR] You dont have an existing account!");
                    println!();
                }
            }
            "7" => break,
            _ => {
                println!("[ERROR] Invalid choice!");
                println!();
            }
        }
    }
}
