use crate::context::EnkryptitContext;
use crate::frontend::cli::Output;
use crate::errors::EnkryptitError;
use crate::parameters::params::{load_params};
use crate::treatment::object_treatment::treat_object;
use crate::types::{Interface};
use crate::log_error;
use crate::success;
use inquire::{Text};

/// Private helper for encrypting an object.
pub fn handle_object_treatment() -> Result<(), EnkryptitError> {
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