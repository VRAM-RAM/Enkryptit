pub mod action;
pub mod browse;
pub mod help;
pub mod treatment;
pub mod parameters;

use crate::VERSION;
use colored::*;
use inquire::{Select};
use self::action::EnkryptitTuiAction;

// Macros with colors

/// Success macro helper
#[allow(unused)] // It is not unused, but... Rust Analyser thinks that
macro_rules! success {
    ($message:expr) => {
        println!("\n[OK] {}", $message.to_string().green());
    };
}

/// Log macro helper
macro_rules! log_error {
    ($msg:expr) => {
        eprintln!("\n[ERROR] {}", $msg.to_string().red())
    };
}

/// Show Params macro helper
#[allow(unused)] // It is not unused, but... Rust Analyser thinks that
macro_rules! show_params {
    ($kt:expr, $c:expr) => {
        println!("\n{}", "Current Parameters".cyan().bold());
        println!("   Key Type:    {}", format!("{:?}", $kt).yellow());
        println!("   Compression: {}", format!("{:?}", $c).yellow());
    };
}


/// Public function that launches the TUI
pub fn launch_ui() {
    println!("\n{}", "Enkryptit".cyan().bold());
    println!("   Fast & Secure File Encryption Manager v0.{}", VERSION);

    loop {
        let choices = vec!["Encrypt/Decrypt file/folder", "Parameters", "Help", "Browse", "Exit"];

        let choice = Select::new("What do you want to do?", choices)
            .with_starting_cursor(0)
            .prompt();

        match choice {
            Ok("Exit") => {
                println!("\n{}", "Goodbye!".green());
                break;
            }

            Ok(value) => {
                if let Some(action) = EnkryptitTuiAction::from_str(value) {
                    if let Err(e) = action.execute() {
                        log_error!(e);
                    }
                }
            }

            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
        }
    }
}