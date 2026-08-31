pub mod action;
pub mod browse;
pub mod help;
pub mod input;
pub mod parameters;
pub mod treatment;

use self::action::EnkryptitTuiAction;
use self::input::TuiInput;
use crate::VERSION;
use colored::*;

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
pub fn launch_ui(input: &impl TuiInput) {
    println!("\n{}", "Enkryptit".cyan().bold());
    println!("   Fast & Secure File Encryption Manager v0.{}", VERSION);

    loop {
        let choices = vec![
            "Encrypt/Decrypt file/folder",
            "Parameters",
            "Help",
            "Browse",
            "Exit",
        ];

        match input.select("What do you want to do?", &choices) {
            Ok(choice) if choice == "Exit" => {
                println!("\n{}", "Goodbye!".green());
                break;
            }

            Ok(value) => {
                if let Some(action) = EnkryptitTuiAction::from_str(&value) {
                    if let Err(e) = action.execute(input) {
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
