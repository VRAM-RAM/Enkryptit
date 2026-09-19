//! Test Helper Functions
//!
//! Runtime-only utility functions for generating test data without fixture files

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use tempfile::NamedTempFile;

static CONFIG_ENV_LOCK: Mutex<()> = Mutex::new(());

/// Points `ECK_CONFIG_PATH` at an isolated config file for the duration of a
/// test. Holds `CONFIG_ENV_LOCK` so parallel tests never interleave env
/// mutations, and the file stays alive until the guard is dropped.
pub struct TestConfigGuard {
    _lock: MutexGuard<'static, ()>,
    config_file: NamedTempFile,
}

impl TestConfigGuard {
    pub fn new(key_params: &str, compression: &str) -> Self {
        let json = serde_json::json!({
            "key_params": key_params,
            "compression": compression,
        });
        let pretty = serde_json::to_string_pretty(&json).unwrap();
        Self::with_raw_content(&pretty)
    }

    /// Creates a config guard explicitly setting the parallelism, using the
    /// same externally-tagged representation as `serde` derives for the
    /// `ParallelismType` enum (e.g. `"MultiThread": 4` or `"Single"`).
    pub fn with_parallelism(
        key_params: &str,
        compression: &str,
        parallelism: serde_json::Value,
    ) -> Self {
        let json = serde_json::json!({
            "key_params": key_params,
            "compression": compression,
            "parallelism": parallelism,
        });
        let pretty = serde_json::to_string_pretty(&json).unwrap();
        Self::with_raw_content(&pretty)
    }

    pub fn with_raw_content(content: &str) -> Self {
        let lock = CONFIG_ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

        let config_file = NamedTempFile::new().expect("Failed to create temp config file");
        fs::write(config_file.path(), content).expect("Failed to write temp config");

        unsafe {
            std::env::set_var("ECK_CONFIG_PATH", config_file.path());
        }

        Self {
            _lock: lock,
            config_file,
        }
    }

    pub fn path(&self) -> &Path {
        self.config_file.path()
    }
}

impl Drop for TestConfigGuard {
    fn drop(&mut self) {
        unsafe {
            std::env::remove_var("ECK_CONFIG_PATH");
        }
    }
}

/// Path of the `.encky` archive produced next to `original`.
pub fn encrypted_path_for(original: &Path) -> PathBuf {
    PathBuf::from(format!("{}.encky", original.display()))
}

/// Generate random content of specified size (bytes)
pub fn generate_random_content(size: usize) -> Vec<u8> {
    let mut content = vec![0u8; size];

    for i in 0..size {
        // Simple deterministic pseudo-random generation for reproducibility
        content[i] = ((i * 2654435781) % 256) as u8;
    }

    content
}

/// Create temporary file with random content (auto-cleanup on drop)
pub fn create_temp_file_with_content(size: usize) -> NamedTempFile {
    let temp = NamedTempFile::new().expect("Failed to create temp file");
    let content = generate_random_content(size);

    fs::write(temp.path(), &content).expect("Failed to write random content");
    temp
}

/// Create temporary directory with nested structure for folder encryption tests
pub fn create_test_directory_structure() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("Failed to create test directory");

    // Create files at root level
    fs::write(dir.path().join("root_file.txt"), b"Root file content").unwrap();

    // Create nested directories with files
    fs::create_dir_all(dir.path().join("level1/level2")).unwrap();
    fs::write(
        dir.path().join("level1/file_in_level1.txt"),
        b"Level 1 file",
    )
    .unwrap();
    fs::write(
        dir.path().join("level1/level2/deep_file.txt"),
        b"Deep nested file",
    )
    .unwrap();

    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_content_size_0() {
        let content = generate_random_content(0);
        assert!(content.is_empty());
    }

    #[test]
    fn generate_random_content_size_1kb() {
        let content = generate_random_content(1024);
        assert_eq!(content.len(), 1024);
    }

    #[test]
    fn create_temp_file_with_content_returns_valid_path() {
        let temp = create_temp_file_with_content(512);

        assert!(temp.path().exists());
        assert!(fs::metadata(temp.path()).unwrap().len() == 512);
    }

    #[test]
    fn create_test_directory_structure_has_nested_files() {
        let dir = create_test_directory_structure();

        // Verify all expected files exist
        assert!(dir.path().join("root_file.txt").exists());
        assert!(dir.path().join("level1/file_in_level1.txt").exists());
        assert!(dir.path().join("level1/level2/deep_file.txt").exists());
    }
}
