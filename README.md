# 🏦 Banking and Currency App (Rust)

A command-line **banking system** written in **Rust**, featuring basic account management, deposit and withdrawal operations, interest computation, and multi-currency exchange functionality with live menu-driven interaction.

---

## 🚀 Features

### 💳 Account Management

- Register a new bank account
- Deposit and withdraw funds
- Displays balance updates in real time

### 💱 Currency Exchange

- Exchange between multiple currencies (`PHP`, `USD`, `JPY`, `GBP`, `EUR`, `CNY`)
- Record and update exchange rates dynamically
- Uses `IndexMap` to preserve insertion order for currencies

### 💰 Interest Computation

- Calculates simple interest based on the number of days
- Uses a fixed interest rate (5% by default)
- Outputs formatted table of daily interest and balance growth

### 🧭 Interactive CLI Menu

- Text-based UI for all actions
- Input validation and error handling
- Organized module-based structure for scalability

---

## 🧩 Project Structure

```
src/
├── main.rs              # Entry point (menu + program loop)
├── models/              # Data models for accounts and currencies
│   └── mod.rs
├── services/            # Core business logic
│   ├── account_services.rs
│   ├── currency_services.rs
│   └── interest_services.rs
└── ui/                  # User interface and input helpers
    ├── menu.rs
    └── input.rs
```

---

## 🦀 Technologies Used

- **Rust** — main programming language
- **IndexMap** — preserves currency order
- **Standard IO** — for CLI input/output
- **Modular architecture** — separates UI, models, and logic

---

## ⚙️ Installation

Make sure you have **Rust** and **Cargo** installed.

```bash
# Clone the repository
git clone https://github.com/<your-username>/banking-currency-app.git
cd banking-currency-app

# Build the project
cargo build

# Run the app
cargo run
```

---

## 🧠 Usage

When you run the program, the main menu will appear:

```
==== Banking and Currency System ====
[1] Register Account
[2] Deposit
[3] Withdraw
[4] Currency Exchange
[5] Record Exchange Rate
[6] Show Interest Amount
[7] Exit
```

Each option leads to an interactive prompt for user inputs.  
Example:

```
Account Name: Max Chavez
Current Balance: 1000.00
Deposit Amount: 500.00
New Balance: 1500.00
```

---

## 🧱 Example Code Highlights

### Flushing Output

```rust
use std::io::Write;

print!("Choose Action: ");
std::io::stdout().flush().unwrap();
```

### Account Initialization

```rust
let mut account = BankAccount {
    name: String::new(),
    balance: 0.0,
    currency_id: String::from("PHP"),
};
```

### Currency Setup

```rust
let mut currencies: IndexMap<String, CurrencyType> = IndexMap::from([
    ("PHP".to_string(), CurrencyType { id: "PHP".into(), name: "Philippine Peso".into(), exchange_rate: 1.0 }),
    ("USD".to_string(), CurrencyType { id: "USD".into(), name: "United States Dollar".into(), exchange_rate: 0.0 }),
    // ...
]);
```

---

## 🧹 Common Warnings & Fixes

| Warning                                   | Fix                                                              |
| ----------------------------------------- | ---------------------------------------------------------------- |
| `no method named 'flush' found`           | Add `use std::io::Write;`                                        |
| `unused import: 'ui::input::return_menu'` | Remove it if not used, or call `return_menu()` in your menu loop |

---

## 🧾 License

This project is licensed under the **MIT License**.  
You are free to use, modify, and distribute it with proper attribution.

---

## ✨ Author

**Max Benedict Chavez**  
Computer Science Major — De La Salle University  
Built as part of the **Banking and Currency Application Project** for systems programming practice in Rust.
