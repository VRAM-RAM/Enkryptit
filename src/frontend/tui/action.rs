use crate::errors::EnkryptitError;
use crate::frontend::tui::{browse::launch_browser, treatment::handle_object_treatment, parameters::launch_params, help::show_help};

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
            Self::EncryptObject => handle_object_treatment(),
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