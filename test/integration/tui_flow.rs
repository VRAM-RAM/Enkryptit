//! TUI / Browsing flow tests
//!
//! Exercises the interactive TUI logic (params menu, browse selection, object
//! treatment, main-menu dispatch) headlessly via `MockTuiInput`, including real
//! filesystem encrypt/decrypt roundtrips through the Tui treatment path.

use std::fs;

use eck::frontend::tui::browse::{
    browse_files, browse_files_then_folders, browse_folders, launch_browser,
};
use eck::frontend::tui::launch_ui;
use eck::frontend::tui::parameters::launch_params;

use crate::mocks::helpers::{TestConfigGuard, encrypted_path_for};
use crate::mocks::tui_input::MockTuiInput;

fn read_config(path: &std::path::Path) -> serde_json::Value {
    let raw = fs::read_to_string(path).expect("read config");
    serde_json::from_str(&raw).expect("parse config")
}

fn guard_password() -> TestConfigGuard {
    TestConfigGuard::with_parallelism("PassWord", "Zstd", serde_json::json!("Single"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use eck::frontend::tui::treatment::treat_objects_encryption;
    use tempfile::{NamedTempFile, tempdir};

    /// Params menu logic

    #[test]
    fn params_menu_changes_compression_and_preserves_parallelism() {
        let guard = guard_password();

        let mut input = MockTuiInput::new()
            .with_select("Change compression type")
            .with_select("Lz4 (fastest)")
            .with_select("Back to main menu");

        launch_params(&mut input).unwrap();

        let cfg = read_config(guard.path());
        assert_eq!(cfg["compression"], "Lz4");
        assert_eq!(cfg["key_params"], "PassWord");
        assert_eq!(cfg["parallelism"], "Single");
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn params_menu_changes_key_type() {
        let guard = guard_password();

        let mut input = MockTuiInput::new()
            .with_select("Change key type")
            .with_select("OS Keyring")
            .with_select("Back to main menu");

        launch_params(&mut input).unwrap();

        let cfg = read_config(guard.path());
        assert_eq!(cfg["key_params"], "Os");
        assert_eq!(cfg["compression"], "Zstd");
        assert_eq!(cfg["parallelism"], "Single");
    }

    #[test]
    fn params_menu_sets_multithread_parallelism_with_count() {
        let guard = guard_password();

        let mut input = MockTuiInput::new()
            .with_select("Change parallelism type")
            .with_select("MultiThread")
            .with_counter(6)
            .with_select("Back to main menu");

        launch_params(&mut input).unwrap();

        let cfg = read_config(guard.path());
        assert_eq!(cfg["parallelism"], serde_json::json!({"MultiThread": 6}));
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn params_menu_sets_single_parallelism_from_multithread() {
        let guard = TestConfigGuard::with_parallelism(
            "PassWord",
            "Zstd",
            serde_json::json!({"MultiThread": 8}),
        );

        let mut input = MockTuiInput::new()
            .with_select("Change parallelism type")
            .with_select("Single")
            .with_select("Back to main menu");

        launch_params(&mut input).unwrap();

        let cfg = read_config(guard.path());
        assert_eq!(cfg["parallelism"], "Single");
    }

    /// Browse selection logic

    #[test]
    fn browse_files_encrypts_the_chosen_file() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"browse me").unwrap();

        let mut input = MockTuiInput::new()
            .with_files(vec![original.path().to_str().unwrap()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");

        browse_files(&mut input, Some("pwd".into())).unwrap();

        assert!(
            encrypted_path_for(original.path()).exists(),
            "chosen file should have been encrypted"
        );
        assert_eq!(input.pending_files(), 0);
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn browse_folders_encrypts_the_chosen_folder() {
        let _guard = guard_password();
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("inside.txt"), b"nested").unwrap();

        let mut input = MockTuiInput::new()
            .with_folders(vec![dir.path().to_str().unwrap()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");

        browse_folders(&mut input, Some("pwd".into())).unwrap();

        // Folder archive is written next to the folder with `.encky` suffix.
        let archive = format!("{}.encky", dir.path().display());
        assert!(std::path::Path::new(&archive).exists());
        assert_eq!(input.pending_folders(), 0);
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn browse_both_merges_files_and_folders() {
        let _guard = guard_password();
        let file = NamedTempFile::new().unwrap();
        fs::write(&file, b"file side").unwrap();
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("inside.txt"), b"dir side").unwrap();

        let mut input = MockTuiInput::new()
            .with_files(vec![file.path().to_str().unwrap()])
            .with_folders(vec![dir.path().to_str().unwrap()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");

        browse_files_then_folders(&mut input, Some("pwd".into())).unwrap();

        assert!(encrypted_path_for(file.path()).exists());
        let archive = format!("{}.encky", dir.path().display());
        assert!(std::path::Path::new(&archive).exists());
        assert_eq!(input.pending_files(), 0, "files pick should be consumed");
        assert_eq!(
            input.pending_folders(),
            0,
            "folders pick should be consumed"
        );
        assert_eq!(input.pending_selects(), 0, "treatment menu should be drained");
    }

    #[test]
    fn browse_both_works_with_only_files() {
        let _guard = guard_password();
        let file = NamedTempFile::new().unwrap();
        fs::write(&file, b"only file").unwrap();

        let mut input = MockTuiInput::new()
            .with_files(vec![file.path().to_str().unwrap()])
            .with_folders(vec![])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");

        browse_files_then_folders(&mut input, Some("pwd".into())).unwrap();

        assert!(encrypted_path_for(file.path()).exists());
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn launch_browser_exits_on_back_to_main_menu() {
        let mut input = MockTuiInput::new().with_select("Back to main menu");

        launch_browser(&mut input).unwrap();
        assert_eq!(input.pending_selects(), 0);
    }

    /// TUI treatment path + roundtrips

    #[test]
    fn browse_then_inspect_prints_report() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"inspect me").unwrap();

        let mut input = MockTuiInput::new()
            .with_files(vec![original.path().to_str().unwrap()])
            .with_select("Inspect")
            .with_select("Go Back");

        browse_files(&mut input, None).unwrap();

        assert_eq!(input.pending_files(), 0, "file pick should be consumed");
        assert_eq!(input.pending_selects(), 0, "treatment menu should be drained");
        assert!(
            !encrypted_path_for(original.path()).exists(),
            "inspection must not encrypt the object"
        );
    }

    #[test]
    fn browse_then_inspect_encrypted_archive() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"inspect the archive").unwrap();

        let mut encrypt_input = MockTuiInput::new()
            .with_files(vec![original.path().to_str().unwrap()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");
        browse_files(&mut encrypt_input, Some("pwd".into())).unwrap();
        let encrypted = encrypted_path_for(original.path());
        assert!(encrypted.exists(), "setup: file should be encrypted first");

        let mut inspect_input = MockTuiInput::new()
            .with_files(vec![encrypted.to_str().unwrap()])
            .with_select("Inspect")
            .with_select("Go Back");
        browse_files(&mut inspect_input, None).unwrap();

        assert_eq!(inspect_input.pending_files(), 0);
        assert_eq!(inspect_input.pending_selects(), 0);
    }

    #[test]
    fn launch_treatment_go_back_does_not_encrypt() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"no treatment").unwrap();

        let mut input = MockTuiInput::new()
            .with_files(vec![original.path().to_str().unwrap()])
            .with_select("Go Back");

        browse_files(&mut input, Some("pwd".into())).unwrap();

        assert!(!encrypted_path_for(original.path()).exists());
        assert_eq!(input.pending_files(), 0);
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn tui_treatment_encrypts_then_decrypts_real_file() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        let content = b"round trip through the Tui treatment path";
        fs::write(&original, content).unwrap();

        let plain = original.path().to_str().unwrap().to_string();
        let encrypted = encrypted_path_for(original.path())
            .to_str()
            .unwrap()
            .to_string();

        // Encrypt: browse chooses the file, then the treatment menu runs it.
        let mut enc = MockTuiInput::new()
            .with_files(vec![plain.as_str()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");
        browse_files(&mut enc, Some("pwd".into())).unwrap();
        assert!(std::path::Path::new(&encrypted).exists());
        assert_eq!(enc.pending_files(), 0);
        assert_eq!(enc.pending_selects(), 0, "treatment menu should be drained");

        // Decrypt back into the plain path (strips the .encky suffix)
        let mut dec = MockTuiInput::new()
            .with_files(vec![encrypted.as_str()])
            .with_select("Encrypt/Decrypt")
            .with_select("Go Back");
        browse_files(&mut dec, Some("pwd".into())).unwrap();
        assert_eq!(dec.pending_selects(), 0);

        let restored = fs::read(original.path()).unwrap();
        assert_eq!(restored, content);
    }

    #[test]
    fn tui_treatment_roundtrip_across_compressions() {
        for comp in ["Zstd", "Lz4", "Xz", "NoComp"] {
            let guard = TestConfigGuard::with_parallelism("PassWord", comp, serde_json::json!("Single"));

            let dir = tempdir().unwrap();
            let plain = dir.path().join("data.bin");
            fs::write(&plain, b"compression aware content").unwrap();
            let encrypted_path = format!("{}.encky", plain.display());
            let encrypted = std::path::PathBuf::from(&encrypted_path);

            let plain_s = plain.to_str().unwrap().to_string();
            let enc_s = encrypted.to_str().unwrap().to_string();
            let password = Some("pwd".into());

            treat_objects_encryption(&vec![plain_s], &password).unwrap();
            assert!(encrypted.exists(), "encrypt with {comp}");

            treat_objects_encryption(&vec![enc_s], &password).unwrap();
            assert_eq!(fs::read(&plain).unwrap(), b"compression aware content");
            drop(guard);
        }
    }

    /// Main menu dispatch loop

    #[test]
    fn launch_ui_help_then_exit() {
        let mut input = MockTuiInput::new().with_select("Help").with_select("Exit");

        launch_ui(&mut input);
        assert_eq!(
            input.pending_selects(),
            0,
            "menu should fully drain before exit"
        );
    }

    #[test]
    fn launch_ui_browse_then_exit() {
        let mut input = MockTuiInput::new()
            .with_select("Browse")
            .with_select("Back to main menu")
            .with_select("Exit");

        launch_ui(&mut input);
        assert_eq!(input.pending_selects(), 0);
    }

    #[test]
    fn launch_ui_browse_then_inspect() {
        let _guard = guard_password();
        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"full ui flow").unwrap();
        let path = original.path().to_str().unwrap().to_string();

        // Browse -> Browse Files -> pick -> Inspect exercises the whole menu
        // stack through `launch_ui`. Encryption is not reachable here because
        // `launch_browser` passes `password: None` down to the treatment menu
        // (encryption with a Password key would open a headless `/dev/tty`
        // prompt); it is covered directly in `tui_treatment_encrypts_then_decrypts_real_file`.
        let mut input = MockTuiInput::new()
            .with_select("Browse")
            .with_select("Browse Files")
            .with_files(vec![path.as_str()])
            .with_select("Inspect")
            .with_select("Go Back")
            .with_select("Back to main menu")
            .with_select("Exit");

        launch_ui(&mut input);

        assert!(
            !encrypted_path_for(original.path()).exists(),
            "inspection must not encrypt the object"
        );
        assert_eq!(input.pending_selects(), 0, "whole menu stack should drain");
    }
}
