use crate::context::EnkryptitContext;
use crate::errors::EnkryptitError;
use crate::frontend::{Output};
use crate::frontend::tui::input::TuiInput;
use crate::log_error;
use crate::parameters::params::load_params;
use crate::success;
use crate::treatment::inspect::inspect_object;
use crate::treatment::object_treatment::treat_object;
use crate::types::Interface;
use colored::Colorize;
use crate::frontend::treat_output::treat_output;

/// Launch the treatment UI
pub fn launch_treatment(input: &impl TuiInput, objects: Vec<String>, password: Option<String>) -> Result<(), EnkryptitError> {
    println!("\n{}", "Browser Panel".cyan().bold());

    loop {
        let choices = vec![
            "Encrypt/Decrypt",
            "Inspect",
            "Go Back",
        ];

        match input.select("What do you want to do?", &choices) {
            Ok(choice) if choice == "Encrypt/Decrypt" => treat_objects_encryption(&objects, &password)?,
            Ok(choice) if choice == "Inspect" => treat_objects_inspection(&objects)?,
            Ok(choice) if choice == "Go Back" => break,
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }

    Ok(())
}

/// Treatment loop over a list of chosen object paths.
fn treat_objects_encryption(objects: &Vec<String>, password: &Option<String>) -> Result<(), EnkryptitError> {
    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, password.clone(), parameters.compression, parameters.parallelism);

    for path_str in objects {
        match treat_object(&parameters, path_str, &mut context)? {
            Output::Success { message } => {
                success!(message);
            }
            Output::Error { error } => {
                log_error!(error)
            }
            Output::CorruptedFile => {
                log_error!("File is corrupted or doesn't exist");
            }
            Output::InspectionReport(report) => report.display()?
        }
    }
    Ok(())
}

fn treat_objects_inspection(objects: &Vec<String>) -> Result<(), EnkryptitError> {
    for path_str in objects {
        treat_output(inspect_object(path_str)?);
    }

    Ok(())
}

