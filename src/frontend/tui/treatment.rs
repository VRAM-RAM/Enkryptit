use crate::context::EnkryptitContext;
use crate::diagnostic::EnkryptitOutput;
use crate::errors::EnkryptitError;
use crate::frontend::tui::input::TuiInput;
use crate::parameters::params::load_params;
use crate::treatment::inspect::inspect_object;
use crate::treatment::object_treatment::treat_object;
use crate::types::Interface;
use colored::Colorize;

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
            Ok(choice) if choice == "Inspect" => treat_objects_inspection(&objects),
            Ok(choice) if choice == "Go Back" => break,
            Err(_) => {
                EnkryptitOutput::info("Selection cancelled").display();
                continue;
            }
            _ => continue,
        }
    }

    Ok(())
}

/// Treatment loop over a list of chosen object paths.
pub fn treat_objects_encryption(objects: &Vec<String>, password: &Option<String>) -> Result<(), EnkryptitError> {
    let parameters = load_params()?;

    let mut context = EnkryptitContext::new(Interface::Tui, password.clone(), parameters.compression, parameters.parallelism);

    for path_str in objects {
        treat_object(&parameters, path_str, &mut context).display();
    }
    Ok(())
}

pub fn treat_objects_inspection(objects: &Vec<String>) {
    for path_str in objects {
        inspect_object(path_str).display();
    }
}

