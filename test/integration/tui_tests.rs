//! Tui Tests
//!
//! Doesn't directly test the TUI itself, but tests the highest level possible
//! (`EnkryptitTuiAction`) using a `MockTuiInput` so no real terminal is needed.
//! Object treatment (encrypt/decrypt/inspect) is not reachable from the action
//! enum anymore: it happens inside `launch_treatment` (see `tui_flow.rs`).

#[cfg(test)]
mod tests {
    use crate::mocks::tui_input::MockTuiInput;
    use eck::frontend::tui::action::EnkryptitTuiAction;

    #[test]
    fn test_from_str_conversion_tui_action() {
        let values = ["Parameters", "Help", "Browse"];

        for value in values {
            if EnkryptitTuiAction::from_str(value).is_none() {
                panic!(
                    "All EnkryptitActions should be converted correctly. This one fails : {}",
                    value
                )
            }
        }
    }

    #[test]
    fn from_str_maps_all_actions() {
        let cases = [
            ("Parameters", "LaunchParams"),
            ("Help", "ShowHelp"),
            ("Browse", "Browse"),
        ];

        for (input, expected) in cases {
            let action = EnkryptitTuiAction::from_str(input);

            assert!(action.is_some(), "Expected action for {input}");

            match (expected, action.unwrap()) {
                ("LaunchParams", EnkryptitTuiAction::LaunchParams)
                | ("ShowHelp", EnkryptitTuiAction::ShowHelp)
                | ("Browse", EnkryptitTuiAction::Browse) => {}
                _ => panic!("Wrong action for {input}"),
            }
        }
    }

    #[test]
    fn from_str_rejects_unknown_action() {
        assert!(EnkryptitTuiAction::from_str("NotAnAction").is_none());
        assert!(EnkryptitTuiAction::from_str("").is_none());
        assert!(EnkryptitTuiAction::from_str("Exit").is_none());
        // The old direct-encryption action was removed in the DAY-15 frontend.
        assert!(EnkryptitTuiAction::from_str("Encrypt/Decrypt file/folder").is_none());
        assert!(EnkryptitTuiAction::from_str("EncryptObject").is_none());
    }

    #[test]
    fn show_help_action_succeeds() {
        let action = EnkryptitTuiAction::ShowHelp;
        let mut input = MockTuiInput::new();

        assert!(action.execute(&mut input).is_ok());
        // ShowHelp is purely a print; it must not consume any interactive input.
        assert_eq!(input.pending_selects(), 0);
        assert_eq!(input.pending_texts(), 0);
    }

    #[test]
    fn launch_params_action_routes_to_params_menu() {
        // LaunchParams enters its own loop, so queue a "Back to main menu" to exit.
        let action = EnkryptitTuiAction::LaunchParams;
        let mut input = MockTuiInput::new().with_select("Back to main menu");

        assert!(action.execute(&mut input).is_ok());
        assert_eq!(
            input.pending_selects(),
            0,
            "params menu should consume the menu choice"
        );
    }

    #[test]
    fn browse_action_routes_to_browser_menu() {
        // Browse enters its own loop, so queue a "Back to main menu" to exit.
        let action = EnkryptitTuiAction::Browse;
        let mut input = MockTuiInput::new().with_select("Back to main menu");

        assert!(action.execute(&mut input).is_ok());
        assert_eq!(
            input.pending_selects(),
            0,
            "browser should consume the menu choice"
        );
    }
}
