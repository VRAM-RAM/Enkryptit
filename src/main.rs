use crate::{
    cli::Output,
    errors::EnkryptitError,
    parameters::params::load_params,
    tui::launch_ui,
    types::CompressionType,
    types::{KeyParams, Version},
};
use clap::{Parser, Subcommand};
mod cli;
mod compression;
mod conversions;
mod encryption;
mod errors;
mod keygen;
mod metadatas;
mod parameters;
mod treatment;
mod tui;
mod types;

/// The version of `Enkryptit!`
pub const VERSION: Version = 2;

use crate::keygen::{
    generate_key_and_store_in_os, generate_key_and_write_file, load_key_from_file, load_key_from_os,
};
use crate::parameters::params::save_params;
use crate::treatment::object_treatment::treat_object;
use crate::types::KeyParams::{File, Os, PassWord};
use crate::types::KeyType;

#[derive(Parser)]
#[command(name = "eck")]
#[command(author = "Olruix")]
#[command(version = "1.0")]
#[command(about = "Fast & Secure File Encryption Manager")]
/// The `Cli` structure for **Enkryptit!** cli-tool
/// \
/// Contains :
/// \
/// - The command
/// - The path (for encrypting/decrypting a file or folder)
/// - The password (for encrypting/decrypting a file or folder w/ password)
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Chemin vers le fichier ou dossier
    #[arg(value_name = "PATH")]
    path: Option<String>,

    /// Mot de passe (optionnel)
    #[arg(short = 'p', long = "password")]
    password: Option<String>,
}

#[derive(Subcommand)]
/// All the commands available :
/// Ui --> open the UI
/// Params / Parameters
/// |-> no arg : show current parameters
/// |-> compression + <ALGO> : change compression algorithm
/// |-> key type + <KEY_TYPE> : change key type
enum Commands {
    Ui,
    Params {
        /// Change le type de compression
        #[arg(short = 'c', long = "compression", value_name = "ALGO")]
        compression: Option<String>,

        /// Change le type de clé
        #[arg(short = 'k', long = "keytype", value_name = "TYPE")]
        key_type: Option<String>,
    },
    Parameters {
        /// Change le type de compression
        #[arg(short = 'c', long = "compression", value_name = "ALGO")]
        compression: Option<String>,

        /// Change le type de clé
        #[arg(long = "keytype", visible_alias = "kt", value_name = "TYPE")]
        key_type: Option<String>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Some(Commands::Ui) => {
            // If the command is `Ui`, we launch the ui.
            launch_ui();
        }
        Some(Commands::Params {
            compression,
            key_type,
        }) => {
            if compression.is_none() && key_type.is_none() {
                // If we don't have any arg, we show the current parameters
                show_params()
            } else {
                // Else, we update the parameters
                update_params(compression, key_type);
            }
        }
        Some(Commands::Parameters {
            compression,
            key_type,
        }) => {
            // Same we `Parameters`
            if compression.is_none() && key_type.is_none() {
                show_params()
            } else {
                update_params(compression, key_type);
            }
        }
        None => {
            // If we have a path, we treat it using `treat_object_with_path()`
            if let Some(path) = cli.path {
                match treat_object_with_path(&path, cli.password) {
                    Ok(output) => treat_output(output),
                    Err(e) => eprintln!("[ERROR] {}", e),
                }
            } else {
                // If we don't have any path, we launch the UI
                launch_ui();
            }
        }
    }
}

/// Helper for treating a path.
/// First, we load the parameters, before converting the path from `&str` to `&Path`.
/// Then, if the path doesn't exist, we return an error.
/// If it exist, we continue, by getting the `key` and the `keytype`.
/// Finally, we delegate the object treatment to `treat_object()`.
fn treat_object_with_path(
    path_str: &str,
    cli_password: Option<String>,
) -> Result<Output, EnkryptitError> {
    let parameters = load_params()?;
    let path = std::path::Path::new(path_str);

    if !path.exists() {
        return Err(EnkryptitError::FileError);
    }

    let (key, keytype) = match parameters.key_params {
        PassWord => {
            let password = match cli_password {
                Some(p) => p,
                None => {
                    enter_password!();
                    let mut pwd = String::new();
                    std::io::stdin().read_line(&mut pwd)?;
                    pwd.trim().to_string()
                }
            };
            ([0u8; 32], KeyType::Password(password))
        }
        File => match load_key_from_file(path_str) {
            Ok(k) => (k, KeyType::FromFile),
            Err(_) => (generate_key_and_write_file(path_str)?, KeyType::FromFile),
        },
        Os => match load_key_from_os(path_str) {
            Ok(k) => (k, KeyType::FromOS),
            Err(_) => (generate_key_and_store_in_os(path_str)?, KeyType::FromOS),
        },
    };

    treat_object(&parameters, path_str.to_string(), key, keytype)
}

/// Helper for treating an `Output`. Given an `Output`, it prints a text in the terminal
fn treat_output(output: Output) {
    match output {
        Output::Success { message } => println!("Success : {}", message),
        Output::Error { error } => eprintln!("[ERROR] {}", error),
        Output::CorruptedFile => eprintln!("[ERROR] File is corrupted, or doesn't exist."),
        Output::SomethingWentWrong => eprintln!("[ERROR] Something went wrong"),
        Output::ParamsChanged => println!("\n Params were changed ! \n"),
        _ => {}
    }
}

/// Helper for showing the parameters.
/// It loads the current parameters before priting the parameters, or printing an error.
fn show_params() {
    match load_params() {
        Ok(params) => {
            println!("\n Actual parameters :");
            println!(" Key Type : {:?}", params.key_params);
            println!(" Compression : {:?}", params.compression);
            println!();
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}

/// Helper for updating the parameters.
fn update_params(compression: Option<String>, key_type: Option<String>) {
    match load_params() {
        Ok(mut params) => {
            if let Some(c) = compression {
                match c.to_lowercase().as_str() {
                    "zstd" | "1" => params.compression = CompressionType::Zstd,
                    "lz4" | "2" => params.compression = CompressionType::Lz4,
                    "xz" | "3" => params.compression = CompressionType::Xz,
                    "none" | "no" | "4" => params.compression = CompressionType::NoComp,
                    other => {
                        eprintln!("[ERROR] Unknown compression: {}", other);
                        std::process::exit(1);
                    }
                }
            }

            if let Some(kt) = key_type {
                match kt.to_lowercase().as_str() {
                    "password" | "pwd" | "1" => params.key_params = KeyParams::PassWord,
                    "os" | "2" => params.key_params = KeyParams::Os,
                    "file" | "3" => params.key_params = KeyParams::File,
                    other => {
                        eprintln!("[ERROR] Unknown key type: {}", other);
                        std::process::exit(1);
                    }
                }
            }

            match save_params(&params) {
                Ok(_) => println!("\n Params were changed ! \n"),
                Err(e) => {
                    eprintln!("[ERROR] Failed to save: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}
