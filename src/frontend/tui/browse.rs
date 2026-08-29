use crate::context::EnkryptitContext;
use crate::frontend::cli::Output;
use crate::errors::EnkryptitError;
use crate::treatment::object_treatment::treat_object;
use crate::types::{Interface};
use colored::*;
use inquire::{Select};
use rfd::FileDialog;
use crate::log_error;
use crate::success;
use crate::parameters::params::load_params;

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