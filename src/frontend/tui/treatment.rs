use crate::context::EnkryptitContext;
use crate::errors::EnkryptitError;
use crate::frontend::cli::Output;
use crate::frontend::tui::input::TuiInput;
use crate::log_error;
use crate::parameters::params::load_params;
use crate::success;
use crate::treatment::object_treatment::treat_object;
use crate::types::Interface;

/// Private helper for encrypting an object.
/// Asks the user for the path through the given `input`, then treats the object.
pub fn handle_object_treatment(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    handle_object_treatment_with_password(input, None)
}

/// Same as `handle_object_treatment`, but lets the caller supply a password so the
/// interactive password prompt can be bypassed (used by tests).
pub fn handle_object_treatment_with_password(
    input: &impl TuiInput,
    password: Option<String>,
) -> Result<(), EnkryptitError> {
    let path = match input.text(
        "Enter file path:",
        "Path to the file/folder to encrypt/decrypt",
    ) {
        Ok(path) => path,
        Err(_) => {
            log_error!("Selection cancelled");
            return Ok(());
        }
    };

    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, password, parameters.compression, parameters.parallelism);

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
