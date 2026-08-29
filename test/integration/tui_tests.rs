//! Tui Tests
//!
//! Doesn't directly test the TUI itself, but tests the highest level possible (`EnkryptitTuiAction`)

#[cfg(test)]
mod tests {
    use eck::frontend::tui::{action::EnkryptitTuiAction};


    #[test]
    fn test_from_str_conversion_tui_action() {
        let values = ["Encrypt/Decrypt file/folder", "Parameters", "Help", "Browse"];

        for value in values {
            if EnkryptitTuiAction::from_str(value).is_none() {
                panic!("All EnkryptitActions should be converted correctly. This one fails : {}", value)
            }
        }
    }

    #[test]
    fn from_str_maps_all_actions() {
        let cases = [
            ("Encrypt/Decrypt file/folder", "EncryptObject"),
            ("Parameters", "LaunchParams"),
            ("Help", "ShowHelp"),
            ("Browse", "Browse"),
        ];

        for (input, expected) in cases {
            let action = EnkryptitTuiAction::from_str(input);

            assert!(action.is_some(), "Expected action for {input}");

            match (expected, action.unwrap()) {
                ("EncryptObject", EnkryptitTuiAction::EncryptObject)
                | ("LaunchParams", EnkryptitTuiAction::LaunchParams)
                | ("ShowHelp", EnkryptitTuiAction::ShowHelp)
                | ("Browse", EnkryptitTuiAction::Browse) => {}
                _ => panic!("Wrong action for {input}"),
            }
        }
    }

    #[test]
    fn show_help_action_succeeds() {
        let action = EnkryptitTuiAction::ShowHelp;

        assert!(action.execute().is_ok());
    }
}