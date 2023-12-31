use colored::*;
use std::io::{self, Write};

pub fn get_user_input() -> Option<String> {
    print!("{}", ">> You: ".green().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}

pub fn display_response(response: &str) {
    println!("\n{} {}", "GPT:".bright_blue().bold(), response);
}

pub fn print_border() {
    println!("{}", "=".repeat(50).bright_blue().bold());
}
