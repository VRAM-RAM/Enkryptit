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
use rfd::FileDialog;

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
        eprintln!("\n[ERROR] {}", $msg.to_string().red())
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

// TUI

/// Abstraction for Tui's actions 
/// \
/// Used by `launch_ui()` and integration tests.
/// \
/// Contains 4 actions :
/// - EncryptObject
/// - LaunchParams
/// - ShowHelp
/// - Browse
/// \
/// Only implements the `execute(&self)` method, that `matches` the *action* and calls the corresponding function, and the `from_str(&str)` method, that maps an &str to an EnkryptitTuiAction (and returns `None` if the &str does not correspond to any action.)
/// \
/// The `Exit` action is directly handled by the `launch_ui()` function itself.
/// \
/// In fact, the content of the enum isn't used here. It is only used in `/tests/`. That's why we need to keep it.
#[allow(dead_code)]
pub enum EnkryptitTuiAction {
    EncryptObject,
    LaunchParams,
    ShowHelp,
    Browse,
}

impl EnkryptitTuiAction {

    #[allow(dead_code)]
    pub fn execute(&self) -> Result<(), EnkryptitError> {
        match self {
            Self::EncryptObject => handle_encrypt_object(),
            Self::LaunchParams => launch_params(),
            Self::ShowHelp => {
                show_help();
                Ok(())
            }
            Self::Browse => launch_browser(),
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Encrypt/Decrypt file/folder" => Some(Self::EncryptObject),
            "Parameters" => Some(Self::LaunchParams),
            "Help" => Some(Self::ShowHelp),
            "Browse" => Some(Self::Browse),
            _ => None,
        }
    }
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


/// Private helper for printing the help in TUI mode
fn show_help() {
    println!("\n{}", "Available Commands".cyan().bold());
    println!("   Encrypt/Decrypt  -> Process a file");
    println!("   Parameters       -> Configure settings");
    println!("   Help             -> Show this help");
    println!("   Browse           -> Browse files, folders or both to encrypt / decrypt");
    println!("   Exit             -> Quit application");
}

/// Private helper for encrypting an object.
fn handle_encrypt_object() -> Result<(), EnkryptitError> {
    let path = match Text::new("Enter file path:")
        .with_help_message("Path to the file/folder to encrypt/decrypt")
        .prompt() {
        Ok(path) => path,
        Err(_) => {
            log_error!("Selection cancelled");
            return Ok(());
        } 
    };
        

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
    }
}

/// Browse UI

/// Launch the browsing UI
pub fn launch_browser() -> Result<(), EnkryptitError> {
    println!("\n{}", "Browser Panel".cyan().bold());

    loop {
        let choices = vec![
            "Browse Files",
            "Browse Folders",
            "Browse both Files & Folders",
            "Back to main menu",
        ];

        let choice = Select::new("What do you want to Browse?", choices).prompt();

        match choice {
            Ok("Browse Files") => browse_files()?,
            Ok("Browse Folders") => browse_folders()?,
            Ok("Browse both Files & Folders") => browse_files_then_folders()?,
            Ok("Back to main menu") => break,
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }

    Ok(())
}

/// Private helper for browsing file, and calling the object treatment
fn browse_files() -> Result<(), EnkryptitError> {
    let files = FileDialog::new()
        .set_title("Choose file(s) to encrypt / decrypt")
        .pick_files();

    let objects = match files {
        Some(vector_of_pathbuf) => vector_of_pathbuf,
        None => {
            log_error!("You didn't choose any file !");
            return Ok(());
        }
    };

    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, None);

    for path in objects {
        let path_str = match path.to_str() {
            Some(str) => str,
            None => {
                log_error!("Error while converting path to string !");
                continue;
            }
        };

        match treat_object(&parameters, path_str, &mut context)? {
            Output::Success { message } => {
                success!(message);
            }
            Output::Error { error } => {
                log_error!(error)
            },
            Output::CorruptedFile => {
                log_error!("File is corrupted or doesn't exist");
            }
        }
    }
    Ok(())
}

// Private helper for browsing folders and encrypting those folders
fn browse_folders() -> Result<(), EnkryptitError> {
    let folders = FileDialog::new()
        .set_title("Choose folder(s) to encrypt.")
        .pick_folders();

    let objects = match folders {
        Some(vector_of_pathbuf) => vector_of_pathbuf,
        None => {
            log_error!("You didn't choose any file !");
            return Ok(());
        }
    };

    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, None);

    for path in objects {
        let path_str = match path.to_str() {
            Some(str) => str,
            None => {
                log_error!("Error while converting path to string !");
                continue;
            }
        };

        match treat_object(&parameters, path_str, &mut context)? {
            Output::Success { message } => {
                success!(message);
            }
            Output::Error { error } => {
                log_error!(error)
            },
            Output::CorruptedFile => {
                log_error!("File is corrupted or doesn't exist");
            }
        }
        
    }

    Ok(())
}

/// Private helper that... 
fn browse_files_then_folders() -> Result<(), EnkryptitError> {
    // First, the user picks the files he wants to encrypt / decrypt
    let files = FileDialog::new()
        .set_title("Choose file(s) to encrypt / decrypt")
        .pick_files();

    // Then, he picks the folders he wants to encrypt
    let folders = FileDialog::new()
        .set_title("Choose folder(s) to encrypt.")
        .pick_folders();

    // This is ugly, but makes sense
    let objects = match folders {
        // First case, if the user choosed folders, we have two possibilities
        Some(mut vector_of_pathbuf) => {
            match files {
                // First possibility : the user also choosed files, so we append the vector of choosen folders with the vector of choosen files
                Some(mut vector_of_pathbuf2) => {
                    vector_of_pathbuf.append(&mut vector_of_pathbuf2);
                    vector_of_pathbuf
                }
                // Second possibility : the user didn't choosed files, so we only return the vector of choosen folders.
                None => vector_of_pathbuf
            }
        },
        // Second case, if the user didn't choose folders, we have two possibilities
        None => {
            match files {
                // First possibility : the user choosed files, so ze return the vector of choosen files
                Some(vector_of_pathbuf) => vector_of_pathbuf,
                // Second possibility : the user didn't choose anything, so we log an error, and return.
                None => {
                    log_error!("You didn't choose any file or folder !");
                    return Ok(());
                }
            }
        }
    };

    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, None);

    // Then, we iterate over objects, like in `browse_files` and `browse_folders`.
    for path in objects {
        let path_str = match path.to_str() {
            Some(str) => str,
            None => {
                log_error!("Error while converting path to string !");
                continue;
            }
        };

        match treat_object(&parameters, path_str, &mut context)? {
            Output::Success { message } => {
                success!(message);
            }
            Output::Error { error } => {
                log_error!(error)
            },
            Output::CorruptedFile => {
                log_error!("File is corrupted or doesn't exist");
            }
        }
        
    }

    Ok(())
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

