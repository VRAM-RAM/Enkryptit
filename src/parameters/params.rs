use crate::errors::EnkryptitError;
use crate::types::{
    CompressionType::{self},
    KeyParams, ParallelismType,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serializable structure that contains the parameters :
/// - KeyParams
/// - Compression
pub struct EnkryptitParams {
    pub key_params: KeyParams,
    pub compression: CompressionType,
    pub parallelism: ParallelismType,
}

impl EnkryptitParams {
    pub fn new(
        key_params: KeyParams,
        compression: CompressionType,
        parallelism: ParallelismType,
    ) -> Self {
        Self {
            key_params,
            compression,
            parallelism,
        }
    }
}

impl Default for EnkryptitParams {
    fn default() -> Self {
        Self {
            key_params: KeyParams::PassWord,
            compression: CompressionType::Auto,
            parallelism: ParallelismType::Auto,
        }
    }
}

/// Private helper that returns the config path.
fn config_path() -> Result<PathBuf, EnkryptitError> {
    // Check for test environment variable first (per-test isolation)
    if let Ok(test_config_path) = std::env::var("ECK_CONFIG_PATH") {
        return Ok(PathBuf::from(test_config_path));
    }

    // Default: use system config directory
    let dirs =
        ProjectDirs::from("com", "olruix", "Enkryptit").ok_or(EnkryptitError::ConfigError)?;

    let mut path = dirs.config_dir().to_path_buf();

    std::fs::create_dir_all(&path)?;

    path.push("config.json");

    Ok(path)
}

/// Public helper for saving parameters
pub fn save_params(params: &EnkryptitParams) -> Result<(), EnkryptitError> {
    let path = config_path()?;

    let json = serde_json::to_string_pretty(params)?;

    let tmp_path = path.with_extension("json.tmp");

    std::fs::write(&tmp_path, json)?;

    #[cfg(windows)]
    let _ = std::fs::remove_file(&path);

    std::fs::rename(&tmp_path, &path)?;

    Ok(())
}

/// Public helper for loading params.
pub fn load_params() -> Result<EnkryptitParams, EnkryptitError> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(EnkryptitParams::default());
    }

    let content = std::fs::read_to_string(&path)?;

    match serde_json::from_str(&content) {
        Ok(params) => Ok(params),
        Err(e) => {
            eprintln!(
                "[WARN] Config file {} is corrupted ({}), falling back to default parameters.",
                path.display(),
                e
            );
            let backup_path = path.with_extension("json.bak");
            let _ = std::fs::rename(&path, &backup_path);

            Ok(EnkryptitParams::default())
        }
    }
}
