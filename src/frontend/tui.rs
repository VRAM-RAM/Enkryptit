use crate::VERSION;
use crate::context::EnkryptitContext;
use crate::frontend::cli::Output;
use crate::errors::EnkryptitError;
use crate::parameters::params::{EnkryptitParams, load_params, save_params};
use crate::treatment::object_treatment::treat_object;
use crate::types::CompressionType::{self, Lz4, NoComp, Xz};
use crate::types::KeyParams::{File, Os, PassWord};
use crate::types::{Interface};
use colored::*;
use inquire::{Select, Text};

// Macros with colors

/// Success macro helper
macro_rules! success {
    ($message:expr) => {
        println!("\n[OK] {}", $message.to_string().green());
    };
}

/// Log macro helper
macro_rules! log_error {
    ($msg:expr) => {
        eprintln!("\n[ERROR] {}", $msg.to_string().red());
    };
}

/// Show Params macro helper
macro_rules! show_params {
    ($kt:expr, $c:expr) => {
        println!("\n{}", "Current Parameters".cyan().bold());
        println!("   Key Type:    {}", format!("{:?}", $kt).yellow());
        println!("   Compression: {}", format!("{:?}", $c).yellow());
    };
}

// Basic UI

/// Public function that launches the TUI
pub fn launch_ui() {
    println!("\n{}", "Enkryptit".cyan().bold());
    println!("   Fast & Secure File Encryption Manager v{}", VERSION);

    loop {
        let choices = vec!["Encrypt/Decrypt file/folder", "Parameters", "Help", "Exit"];

        let choice = Select::new("What do you want to do?", choices)
            .with_starting_cursor(0)
            .prompt();

        match choice {
            Ok("Encrypt/Decrypt file/folder") => {
                if let Err(e) = handle_encrypt_object() {
                    log_error!(e);
                }
            }
            Ok("Parameters") => {
                if let Err(e) = launch_params() {
                    log_error!(e);
                }
            }
            Ok("Help") => show_help(),
            Ok("Exit") => {
                println!("\n{}", "Goodbye!".green());
                break;
            }
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }
}

/// Private helper for printing the help in TUI mode
fn show_help() {
    println!("\n{}", "Available Commands".cyan().bold());
    println!("   Encrypt/Decrypt  -> Process a file");
    println!("   Parameters       -> Configure settings");
    println!("   Help             -> Show this help");
    println!("   Exit             -> Quit application");
}

/// Private helper for encrypting an object.
fn handle_encrypt_object() -> Result<(), EnkryptitError> {
    let path = Text::new("Enter file path:")
        .with_help_message("Path to the file/folder to encrypt/decrypt")
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, None);

    match treat_object(&parameters, &path, &mut context)? {
        Output::Success { message } => {
            success!(message);
            Ok(())
        }
        Output::Error { error } => Err(error),
        Output::CorruptedFile => {
            log_error!("File is corrupted or doesn't exist");
            Ok(())
        }
        Output::SomethingWentWrong => {
            log_error!("Something went wrong");
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Parameters UI

/// Launch the params UI
pub fn launch_params() -> Result<(), EnkryptitError> {
    println!("\n{}", "Parameters Panel".cyan().bold());

    loop {
        let choices = vec![
            "Change compression type",
            "Change key type",
            "Switch Ui",
            "Show current parameters",
            "Back to main menu",
        ];

        let choice = Select::new("What do you want to configure?", choices).prompt();

        match choice {
            Ok("Change compression type") => change_compression()?,
            Ok("Change key type") => change_key_type()?,
            Ok("Show current parameters") => show_current_params()?,
            Ok("Back to main menu") => break,
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }

    success!("Parameters updated successfully!");
    Ok(())
}

/// Private helper for changing compression
fn change_compression() -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec![
        "Zstd (balanced)",
        "Lz4 (fastest)",
        "Xz (best compression)",
        "No compression",
    ];

    let choice = Select::new("Select compression type:", choices)
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let compression = match choice {
        "Zstd (balanced)" => CompressionType::Zstd,
        "Lz4 (fastest)" => Lz4,
        "Xz (best compression)" => Xz,
        "No compression" => NoComp,
        _ => CompressionType::Zstd,
    };

    let params = EnkryptitParams::new(old_params.key_params, compression);
    save_params(&params)?;

    success!(format!("Compression changed to {:?}", compression));
    Ok(())
}

/// Private helper for changing key type
fn change_key_type() -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec!["Password (Argon2id)", "OS Keyring", "Key File"];

    let choice = Select::new("Select key type:", choices)
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let kt = match choice {
        "Password (Argon2id)" => PassWord,
        "OS Keyring" => Os,
        "Key File" => File,
        _ => PassWord,
    };

    let params = EnkryptitParams::new(kt.clone(), old_params.compression);
    save_params(&params)?;

    success!(format!("Key type changed to {:?}", kt));
    Ok(())
}

/// Private helper for printing current parameters
fn show_current_params() -> Result<(), EnkryptitError> {
    let params = load_params()?;
    show_params!(params.key_params, params.compression);
    Ok(())
}

