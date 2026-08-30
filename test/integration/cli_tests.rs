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
    /// Test that tests if the `eck params` command actually prints the parameters.
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

    // -- Compression type settings tests 

    #[test]
    /// Tests changing the compression type to `zstd`
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
    /// Tests changing the compression type to `lz4`
    fn cli_params_set_compression_lz4() {
        let guard = TestConfigGuard::new("PassWord", "NoComp");

        let output = eck_cmd(&["params", "--compression", "lz4"])
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
        assert!(saved.contains("\"Lz4\""), "config: {saved}");

        // And a subsequent read reflects it
        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Compression : Lz4"), "stdout: {stdout}");
    }

    #[test]
    /// Tests changing the compression type to `xz`
    fn cli_params_set_compression_xz() {
        let guard = TestConfigGuard::new("PassWord", "NoComp");

        let output = eck_cmd(&["params", "--compression", "xz"])
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
        assert!(saved.contains("\"Xz\""), "config: {saved}");

        // And a subsequent read reflects it
        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Compression : Xz"), "stdout: {stdout}");
    }

    // --- KeyType settings tests

    #[test]
    /// Tests changing the keytype to `os`
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
    /// Tests changing the keytype to `file`
    fn cli_params_set_keytype_file() {
        let guard = TestConfigGuard::new("PassWord", "Zstd");

        let output = eck_cmd(&["params", "--keytype", "file"])
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
        assert!(saved.contains("\"File\""), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Key Type : File"), "stdout: {stdout}");
    }

    #[test]
    /// Tests changing the keytype to `os`
    fn cli_params_set_keytype_pwd() {
        let guard = TestConfigGuard::new("PassWord", "Zstd");

        let output = eck_cmd(&["params", "--keytype", "pwd"])
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
        assert!(saved.contains("\"PassWord\""), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Key Type : PassWord"), "stdout: {stdout}");
    }

    #[test]
    /// Tests changing the keytype to `password`
    fn cli_params_set_keytype_password() {
        let guard = TestConfigGuard::new("PassWord", "Zstd");

        let output = eck_cmd(&["params", "--keytype", "password"])
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
        assert!(saved.contains("\"PassWord\""), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Key Type : PassWord"), "stdout: {stdout}");
    }

    // --- Encryption / decryption tests

    #[test]
    /// Tests encrypting & decrypting a `file` with a password (-p flag).
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
    /// Tests encrypting & decrypting a folder
    fn cli_folder_encrypt_decrypt_roundtrip() {
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
    /// Tests encrypting & decrypting a folder and a file with the same command
    fn cli_folders_and_files_encrypt_decrypt_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let temp_file = NamedTempFile::new().unwrap();

        let folder = tmp.path().join("cli_folder");
        fs::write(temp_file.path(), b"Test content for CLI encryption.").unwrap();

        fs::create_dir_all(folder.join("nested")).unwrap();
        fs::write(folder.join("a.txt"), b"alpha content").unwrap();
        fs::write(folder.join("nested/b.txt"), b"beta content").unwrap();

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(temp_file.path())
            .arg(&folder)
            .assert()
            .success();
        
        let archive_path = encrypted_path_for(&folder);
        let file_path = encrypted_path_for(temp_file.path());

        assert!(archive_path.exists(), "folder archive should exist");
        assert!(file_path.exists(), "file should exist");

        fs::remove_dir_all(&folder).unwrap();

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(archive_path)
            .arg(file_path)
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

        let restored = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(restored, "Test content for CLI encryption.");
    }

    // --- Parameters config tests

    #[test]
    /// Tests that a corrupted config file (`config.json`) automatically fallsback to default config.
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
    /// Tests that repeatingly changing parameters doesn't corrupt the file.
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

    // --- Cli (more tests)

    #[test]
    fn cli_unknown_argument_fails() {
        let output = eck_cmd(&["--this-does-not-exist"])
            .output()
            .unwrap();

        assert!(!output.status.success());
    }

    // --- Parallelism settings tests

    #[test]
    fn cli_params_set_parallelism_single() {
        use serde_json::json;
        let guard = TestConfigGuard::with_parallelism("PassWord", "NoComp", json!("Single"));

        let output = eck_cmd(&["params", "--parallelism", "single"])
            .output()
            .expect("Failed to execute eck params");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let saved = fs::read_to_string(guard.path()).unwrap();
        assert!(saved.contains("\"Single\""), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Parallelism : Single"), "stdout: {stdout}");
    }

    #[test]
    fn cli_params_set_parallelism_multi() {
        use serde_json::json;
        let guard = TestConfigGuard::with_parallelism("PassWord", "NoComp", json!("Single"));

        let output = eck_cmd(&["params", "--parallelism", "multi:6"])
            .output()
            .expect("Failed to execute eck params");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let saved = fs::read_to_string(guard.path()).unwrap();
        assert!(saved.contains("\"MultiThread\": 6"), "config: {saved}");

        let output = eck_cmd(&["params"]).output().unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("MultiThread(6)"), "stdout: {stdout}");
    }

    #[test]
    fn cli_params_set_parallelism_zero_threads_fails() {
        use serde_json::json;
        let guard = TestConfigGuard::with_parallelism("PassWord", "NoComp", json!("Single"));

        let output = eck_cmd(&["params", "--parallelism", "multi:0"])
            .output()
            .unwrap();

        assert!(!output.status.success());
        // Config must remain untouched on failure
        let saved = fs::read_to_string(guard.path()).unwrap();
        assert!(saved.contains("\"Single\""), "config: {saved}");
    }

    #[test]
    fn cli_multithread_encrypt_decrypt_roundtrip() {
        use serde_json::json;

        let _guard = TestConfigGuard::with_parallelism(
            "PassWord",
            "NoComp",
            json!({ "MultiThread": 4 }),
        );
        let temp_file = NamedTempFile::new().unwrap();

        fs::write(temp_file.path(), b"multithread cli roundtrip content").unwrap();

        let encrypted_path = encrypted_path_for(temp_file.path());

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(temp_file.path())
            .assert()
            .success();
        assert!(encrypted_path.exists(), "archive should exist after encrypt");

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(&encrypted_path)
            .assert()
            .success();

        let restored = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(restored, "multithread cli roundtrip content");
    }

    #[test]
    fn cli_multithread_encrypt_decrypt_large_file_roundtrip() {
        use serde_json::json;

        let _guard = TestConfigGuard::with_parallelism(
            "PassWord",
            "Zstd",
            json!({ "MultiThread": 8 }),
        );
        let temp_file = NamedTempFile::new().unwrap();

        // Large enough to span several chunks across the worker pool.
        let content = vec![b'q'; 3 * 1024 * 1024];
        fs::write(temp_file.path(), &content).unwrap();

        let encrypted_path = encrypted_path_for(temp_file.path());

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(temp_file.path())
            .assert()
            .success();
        assert!(encrypted_path.exists());

        Command::cargo_bin("eck")
            .unwrap()
            .arg("-p")
            .arg("test123")
            .arg(&encrypted_path)
            .assert()
            .success();

        let restored = fs::read(temp_file.path()).unwrap();
        assert_eq!(restored, content);
    }
    
}