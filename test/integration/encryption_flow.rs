//! Full Encryption Workflow Tests
//!
//! Test complete file encryption/decryption pipeline from CLI to disk storage

use crate::mocks::helpers::{TestConfigGuard, encrypted_path_for};
use assert_cmd::assert::{Assert, OutputAssertExt};
use assert_cmd::cargo::CommandCargoExt;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

/// Run CLI encryption command with password flag  
fn run_cli_encrypt(
    path: &std::path::Path,
    password: Option<&str>,
) -> Result<Assert, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("eck")?;

    if let Some(pwd) = password {
        cmd.arg("-p").arg(pwd);
    }
    cmd.arg(path);

    Ok(cmd.assert())
}

/// Run CLI decryption command with password flag  
fn run_cli_decrypt(
    path: &std::path::Path,
    password: Option<&str>,
) -> Result<Assert, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("eck")?;

    if let Some(pwd) = password {
        cmd.arg("-p").arg(pwd);
    }
    cmd.arg(path);

    Ok(cmd.assert())
}

fn stderr_of(assertion: &Assert) -> String {
    String::from_utf8_lossy(&assertion.get_output().stderr).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_password_keytype_no_comp_roundtrip() {
        let _guard = TestConfigGuard::new("PassWord", "NoComp");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"Testing full workflow without compression.").unwrap();

        let encrypted_path = encrypted_path_for(original.path());

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        let decrypted = fs::read_to_string(original.path()).unwrap();
        assert_eq!(decrypted, "Testing full workflow without compression.");
    }

    #[test]
    fn encrypt_decrypt_empty_file() {
        let _guard = TestConfigGuard::new("PassWord", "Zstd");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"").unwrap(); // Empty file

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();

        let encrypted_path = encrypted_path_for(original.path());
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        assert_eq!(fs::metadata(original.path()).unwrap().len(), 0);
    }

    #[test]
    fn encrypt_decrypt_password_keytype_zstd_roundtrip() {
        let _guard = TestConfigGuard::new("PassWord", "Zstd");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"Testing full workflow with Zstd compression.").unwrap();

        let encrypted_path = encrypted_path_for(original.path());

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        let decrypted = fs::read_to_string(original.path()).unwrap();
        assert_eq!(decrypted, "Testing full workflow with Zstd compression.");
    }

    #[test]
    fn encrypt_decrypt_password_keytype_lz4_roundtrip() {
        let _guard = TestConfigGuard::new("PassWord", "Lz4");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"Testing full workflow with Lz4 compression.").unwrap();

        let encrypted_path = encrypted_path_for(original.path());

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        let decrypted = fs::read_to_string(original.path()).unwrap();
        assert_eq!(decrypted, "Testing full workflow with Lz4 compression.");
    }

    #[test]
    fn encrypt_decrypt_password_keytype_xz_roundtrip() {
        let _guard = TestConfigGuard::new("PassWord", "Xz");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"XZ compression with full workflow validation.").unwrap();

        let encrypted_path = encrypted_path_for(original.path());

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        let decrypted = fs::read_to_string(original.path()).unwrap();
        assert_eq!(decrypted, "XZ compression with full workflow validation.");
    }

    #[test]
    #[ignore = "OS keyring requires manual testing - see TODO below"]
    fn encrypt_decrypt_os_keytype_roundtrip() {
        // TODO: Implement when OS credential store mocking is available

        let original = NamedTempFile::new().unwrap();

        fs::write(&original, b"OS credential store encryption test.").unwrap();
        assert!(original.path().exists());
    }

    #[test]
    fn encrypt_decrypt_large_file() {
        let _guard = TestConfigGuard::new("PassWord", "Zstd");

        let original = NamedTempFile::new().unwrap();

        // Create 20MB of repetitive data (faster than random, still tests chunking)
        let large_content = vec![b'x'; 20 * 1024 * 1024];
        fs::write(&original, &large_content).unwrap();

        run_cli_encrypt(original.path(), Some("test123"))
            .unwrap()
            .success();

        // Verify encrypted file exists and is different size (due to headers)
        let encrypted_path = encrypted_path_for(original.path());
        assert!(encrypted_path.exists());

        run_cli_decrypt(encrypted_path.as_ref(), Some("test123"))
            .unwrap()
            .success();

        let decrypted = fs::read(original.path()).unwrap();
        assert_eq!(decrypted.len(), large_content.len());
        assert_eq!(decrypted, large_content);
    }

    #[test]
    fn wrong_password_decryption_fails_cleanly() {
        let _guard = TestConfigGuard::new("PassWord", "NoComp");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"secret content for wrong-password test").unwrap();

        run_cli_encrypt(original.path(), Some("right-pass"))
            .unwrap()
            .success();

        let encrypted_path = encrypted_path_for(original.path());
        let archive_before = fs::read(&encrypted_path).unwrap();

        let wrong = run_cli_decrypt(encrypted_path.as_ref(), Some("wrong-pass")).unwrap();

        // Failure is reported on stderr (exit code stays 0 by design)
        let stderr = stderr_of(&wrong);
        assert!(stderr.contains("[ERROR]"), "stderr: {stderr}");

        // The archive must be left untouched and no plaintext restored
        assert!(encrypted_path.exists());
        assert_eq!(fs::read(&encrypted_path).unwrap(), archive_before);
        assert_ne!(
            fs::read(original.path()).unwrap_or_default(),
            b"secret content for wrong-password test".to_vec()
        );
    }

    #[test]
    fn tampered_archive_is_detected() {
        let _guard = TestConfigGuard::new("PassWord", "NoComp");

        let original = NamedTempFile::new().unwrap();
        fs::write(&original, b"integrity protected content").unwrap();

        run_cli_encrypt(original.path(), Some("tamper-test"))
            .unwrap()
            .success();

        let encrypted_path = encrypted_path_for(original.path());
        let mut archive = fs::read(&encrypted_path).unwrap();

        // Flip a byte near the end of the payload (inside the final AEAD chunk)
        let last = archive.len() - 2;
        archive[last] ^= 0xFF;
        let tampered_archive = archive.clone();
        fs::write(&encrypted_path, &tampered_archive).unwrap();

        let result = run_cli_decrypt(encrypted_path.as_ref(), Some("tamper-test")).unwrap();

        let stderr = stderr_of(&result);
        assert!(
            !stderr.is_empty(),
            "tampering must produce an error message"
        );

        // The tampered archive is not silently repaired or consumed
        assert_eq!(fs::read(&encrypted_path).unwrap(), tampered_archive);
        assert_ne!(
            fs::read(original.path()).unwrap_or_default(),
            b"integrity protected content".to_vec()
        );
    }
}