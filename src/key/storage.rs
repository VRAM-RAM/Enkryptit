use keyring::Entry;
use std::path::PathBuf;

/// The service name for OS's keyring key storage
const SERVICE: &str = "ENKRYPTIT";

use crate::{conversions::convert_key, errors::EnkryptitError};

/// Private helper that creates and returns the `PathBuf` of the key a file was encrypted with.
fn key_path(filename: &str) -> Result<PathBuf, EnkryptitError> {
    let mut path = dirs::home_dir().ok_or(EnkryptitError::HomeNotFound)?;

    path.push("private_keys");
    std::fs::create_dir_all(&path)?;

    let file_path = PathBuf::from(filename);

    let stem = if file_path.extension().and_then(|e| e.to_str()) == Some("encky") {
        file_path.file_stem()
    } else {
        file_path.file_name()
    }
    .ok_or(EnkryptitError::FileError)?
    .to_string_lossy()
    .into_owned();

    path.push(format!("enkryptit_{}", stem));
    Ok(path)
}

/// Public helper for loading the key from the OS's keyring.
pub fn load_key_from_os(filename: &str) -> Result<[u8; 32], EnkryptitError> {
    let user = format!("enkryptit{}", filename);

    let entry = Entry::new(SERVICE, &user)?;

    let hex_key = entry.get_password()?;

    let bytes = hex::decode(hex_key)?;

    if bytes.len() != 32 {
        return Err(EnkryptitError::InvalidKeyLength);
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes);

    Ok(key)
}

/// Public helper for saving key in Os's keyring
pub fn save_key_in_os(filename: &str, key: [u8; 32]) -> Result<(), EnkryptitError> {
    let user = format!("enkryptit{}", filename);

    let entry = Entry::new("ENKRYPTIT", &user)?;

    entry.set_password(&hex::encode(key))?;

    Ok(())
}

/// Public helper for saving key in key file.
pub fn save_key_in_file(filename: &str, key: [u8; 32]) -> Result<(), EnkryptitError> {
    let path = key_path(filename)?;

    std::fs::write(path, hex::encode(key))?;

    Ok(())
}

/// Public helper for loading key from key file
pub fn load_key_from_file(filename: &str) -> Result<[u8; 32], EnkryptitError> {
    let path = key_path(filename)?;

    let key = std::fs::read(path)?;

    Ok(convert_key(hex::decode(key)?)?)
}
