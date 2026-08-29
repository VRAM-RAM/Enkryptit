use crate::{
    frontend::{params_helpers::{show_params, update_params}, treat_output::treat_output, treatment::treat_objects_with_multiple_paths, tui::launch_ui}, types::Version
};
use clap::{Parser, Subcommand};
mod frontend;
mod compression;
mod conversions;
mod encryption;
mod errors;
mod metadatas;
mod parameters;
mod treatment;
mod types;
pub mod context;
pub mod key;
use crate::frontend::treatment::treat_object_with_path;


/// The version of `Enkryptit!`
pub const VERSION: Version = 2;

#[derive(Parser)]
#[command(name = "eck")]
#[command(author = "Olruix")]
#[command(version = "0.2")]
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

    /// Chemin(s) vers le fichier ou dossier
    #[arg(value_name = "PATH")]
    path: Vec<String>,

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
            let path: Vec<String> = cli.path;
            // If we have a path, we treat it using `treat_object_with_path()`
            match path.len() {
                0 => launch_ui(),
                1 => {
                    match treat_object_with_path(&path[0], cli.password) {
                        Ok(output) => treat_output(output),
                        Err(e) => eprintln!("[ERROR] {}", e),
                    }
                }
                _ => {
                    match treat_objects_with_multiple_paths(&path, cli.password) {
                        Ok(()) => (),
                        Err(e) =>  eprintln!("[ERROR] {}", e),
                    }
                }
            }
        }
    }
}




