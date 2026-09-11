use crate::errors::EnkryptitError;
use crate::frontend::tui::input::TuiInput;
use crate::frontend::tui::treatment::launch_treatment;
use crate::log_error;
use colored::*;

/// Browse UI

/// Launch the browsing UI
pub fn launch_browser(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    println!("\n{}", "Browser Panel".cyan().bold());

    loop {
        let choices = vec![
            "Browse Files",
            "Browse Folders",
            "Browse both Files & Folders",
            "Back to main menu",
        ];

        match input.select("What do you want to Browse?", &choices) {
            Ok(choice) if choice == "Browse Files" => browse_files(input, None)?,
            Ok(choice) if choice == "Browse Folders" => browse_folders(input, None)?,
            Ok(choice) if choice == "Browse both Files & Folders" => {
                browse_files_then_folders(input, None)?
            }
            Ok(choice) if choice == "Back to main menu" => break,
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }

    Ok(())
}

/// Helper for browsing file, and calling the object treatment
pub fn browse_files(input: &impl TuiInput, password: Option<String>) -> Result<(), EnkryptitError> {
    let objects = input.pick_files("Choose file(s)");

    if objects.is_empty() {
        log_error!("You didn't choose any file !");
        return Ok(());
    }

    launch_treatment(input, objects, password)
}

// Private helper for browsing folders and treating those folders
pub fn browse_folders(
    input: &impl TuiInput,
    password: Option<String>,
) -> Result<(), EnkryptitError> {
    let folders = input.pick_folders("Choose folder(s)");

    if folders.is_empty() {
        log_error!("You didn't choose any folder !");
        return Ok(());
    }

    launch_treatment(input, folders, password)
}

/// Private helper that merges the chosen files and folders, then treats them.
pub fn browse_files_then_folders(
    input: &impl TuiInput,
    password: Option<String>,
) -> Result<(), EnkryptitError> {
    // First, the user picks the files he wants to treat
    let files = input.pick_files("Choose file(s)");

    // Then, he picks the folders he wants to encrypt
    let folders = input.pick_folders("Choose folder(s)");

    let mut objects = folders;
    objects.extend(files);

    if objects.is_empty() {
        log_error!("You didn't choose any file or folder !");
        return Ok(());
    }

    launch_treatment(input, objects, password)
}


