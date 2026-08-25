//! CLI Command Parsing Tests
//!
//! Test command-line interface behavior: arguments, subcommands, error handling

use crate::mocks::helpers::{TestConfigGuard, encrypted_path_for};
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::fs;
use std::process::{Command, Stdio};
use tempfile::{NamedTempFile, TempDir};

/// Build a `eck` command pointing at the freshly built binary (never PATH).
fn eck_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("eck").unwrap();
    cmd.args(args);
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_no_args_launches_tui() {
        // Test that running 'eck' without args launches TUI (will timeout)
        let mut child = Command::cargo_bin("eck")
            .unwrap()
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start eck");

        // Kill the process after a short delay (TUI waits for input)
        std::thread::sleep(std::time::Duration::from_millis(100));

        let _ = child.kill();
    }

    #[test]
    fn cli_ui_subcommand_launches_tui() {
        // Test that 'eck ui' explicitly launches TUI
        let mut child = Command::cargo_bin("eck")
            .unwrap()
            .arg("ui")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start eck ui");

        std::thread::sleep(std::time::Duration::from_millis(100));

        let _ = child.kill();
    }

    #[test]
    fn cli_params_command_shows_config() {
        let _guard = TestConfigGuard::new("PassWord", "Zstd");

        let output = eck_cmd(&["params"])
            .output()
            .expect("Failed to execute eck params");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Actual parameters"), "stdout: {stdout}");
        assert!(stdout.contains("Key Type : PassWord"), "stdout: {stdout}");
        assert!(stdout.contains("Compression : Zstd"), "stdout: {stdout}");
    }

    #[test]
    fn cli_params_set_compression_zstd() {
        let guard = TestConfigGuard::new("PassWord", "NoComp");

        let output = eck_cmd(&["params", "--compression", "zstd"])
            .output()
            .expect("Failed to execute eck params");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Params were changed"), "stdout: {stdout}");

        // The change must be persisted to the isolated config file
        let saved = fs::read_to_string(guard.path()).unwrap();
        assert!(saved.contains("\"Zstd\""), "config: {saved}");

        // And a subsequent read reflects it
        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Compression : Zstd"), "stdout: {stdout}");
    }

    #[test]
    fn cli_params_set_keytype_os() {
        let guard = TestConfigGuard::new("PassWord", "Zstd");

        let output = eck_cmd(&["params", "--keytype", "os"])
            .output()
            .expect("Failed to execute eck params");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Params were changed"), "stdout: {stdout}");

        let saved = fs::read_to_string(guard.path()).unwrap();
        assert!(saved.contains("\"Os\""), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Key Type : Os"), "stdout: {stdout}");
    }

    #[test]
    fn cli_encrypt_file_with_password_flag() {
        let _guard = TestConfigGuard::new("PassWord", "NoComp");
        let temp_file = NamedTempFile::new().unwrap();

        fs::write(temp_file.path(), b"Test content for CLI encryption.").unwrap();

        let encrypted_path = encrypted_path_for(temp_file.path());

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(temp_file.path())
            .assert()
            .success();
        assert!(
            encrypted_path.exists(),
            "archive should exist after encrypt"
        );

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(&encrypted_path)
            .assert()
            .success();

        let restored = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(restored, "Test content for CLI encryption.");
    }

    #[test]
    fn cli_folder_encrypt_decrypt_roundtrip() {
        let _guard = TestConfigGuard::new("PassWord", "Zstd");

        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("cli_folder");
        fs::create_dir_all(folder.join("nested")).unwrap();
        fs::write(folder.join("a.txt"), b"alpha content").unwrap();
        fs::write(folder.join("nested/b.txt"), b"beta content").unwrap();

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("fold3r")
            .arg(&folder)
            .assert()
            .success();

        let archive_path = encrypted_path_for(&folder);
        assert!(archive_path.exists(), "folder archive should exist");

        fs::remove_dir_all(&folder).unwrap();

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("fold3r")
            .arg(&archive_path)
            .assert()
            .success();

        assert_eq!(
            fs::read_to_string(folder.join("a.txt")).unwrap(),
            "alpha content"
        );
        assert_eq!(
            fs::read_to_string(folder.join("nested/b.txt")).unwrap(),
            "beta content"
        );
    }

    #[test]
    fn corrupt_config_falls_back_to_defaults() {
        let _guard = TestConfigGuard::with_raw_content("{ definitely :: not json ]");

        let output = eck_cmd(&["params"]).output().unwrap();

        assert!(
            output.status.success(),
            "corrupt config must degrade to defaults, stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Key Type : PassWord"), "stdout: {stdout}");
        assert!(stdout.contains("Compression : Zstd"), "stdout: {stdout}");
    }

    #[test]
    fn repeated_param_updates_keep_config_valid_json() {
        let guard = TestConfigGuard::new("PassWord", "NoComp");

        for algo in ["lz4", "xz", "zstd"] {
            let output = eck_cmd(&["params", "--compression", algo])
                .output()
                .unwrap();
            assert!(output.status.success());
        }

        let saved = fs::read_to_string(guard.path()).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&saved).expect("config must remain parseable after updates");
        assert_eq!(parsed["compression"], "Zstd");
        assert_eq!(parsed["key_params"], "PassWord");
    }
}
