use crate::errors::EnkryptitError;
use inquire::ui::RenderConfig;
use inquire::{CustomType, Select, Text};

/// Abstraction over the interactive TUI inputs.
///
/// The real UI is driven by `inquire` prompts and `rfd` native file dialogs,
/// neither of which can run headlessly inside `cargo test`. By routing every
/// interactive read through this trait, the surrounding logic (menus, params
/// persistence, browse object collection, treatment dispatch) can be tested
/// with a `MockTuiInput` while `RealTuiInput` keeps the exact live behaviour.
pub trait TuiInput {
    /// Show a single-choice menu and return the selected label.
    fn select(&self, message: &str, choices: &[&str]) -> Result<String, EnkryptitError>;

    /// Prompt for free-form text (used for the treatment path).
    fn text(&self, message: &str, help_message: &str) -> Result<String, EnkryptitError>;

    /// Prompt for a numeric counter (used for the number of threads).
    fn custom_counter(&self, message: &str) -> Result<u8, EnkryptitError>;

    /// Open a native dialog to pick one or more files, returning their paths.
    fn pick_files(&self, title: &str) -> Vec<String>;

    /// Open a native dialog to pick one or more folders, returning their paths.
    fn pick_folders(&self, title: &str) -> Vec<String>;
}

/// Default implementation that talks to the real terminal (`inquire`) and the
/// OS native file dialogs (`rfd`). Used by the real `launch_ui()` entrypoint.
pub struct RealTuiInput;

impl TuiInput for RealTuiInput {
    fn select(&self, message: &str, choices: &[&str]) -> Result<String, EnkryptitError> {
        match Select::new(message, choices.to_vec()).prompt() {
            Ok(value) => Ok(value.to_string()),
            Err(_) => Err(EnkryptitError::CommandNotFound),
        }
    }

    fn text(&self, message: &str, help_message: &str) -> Result<String, EnkryptitError> {
        match Text::new(message).with_help_message(help_message).prompt() {
            Ok(value) => Ok(value),
            Err(_) => Err(EnkryptitError::CommandNotFound),
        }
    }

    fn custom_counter(&self, message: &str) -> Result<u8, EnkryptitError> {
        let counter: CustomType<u8> = CustomType::new(message)
            .with_default(4)
            .with_error_message("Please type a valid number, greater than 0.")
            .with_help_message(
                "The number of threads you want to use. Recommanded : 4-8 (must be greater than 0).",
            )
            .with_parser(&|u| match u.trim().parse::<u8>() {
                Ok(val) if val > 0 => Ok(val),
                _ => Err(()),
            })
            .with_render_config(RenderConfig::default());

        match counter.prompt() {
            Ok(value) => Ok(value),
            Err(_) => Err(EnkryptitError::CommandNotFound),
        }
    }

    fn pick_files(&self, title: &str) -> Vec<String> {
        match rfd::FileDialog::new().set_title(title).pick_files() {
            Some(paths) => paths
                .into_iter()
                .filter_map(|p| p.to_str().map(String::from))
                .collect(),
            None => Vec::new(),
        }
    }

    fn pick_folders(&self, title: &str) -> Vec<String> {
        match rfd::FileDialog::new().set_title(title).pick_folders() {
            Some(paths) => paths
                .into_iter()
                .filter_map(|p| p.to_str().map(String::from))
                .collect(),
            None => Vec::new(),
        }
    }
}
