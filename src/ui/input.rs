use std::io::{self, Write};

pub fn return_menu() -> bool {
    let mut input = String::new();
    print!("Back to the Main Menu (Y/N): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_uppercase().as_str(), "Y")
}
