use keyring::Entry;
use libc::{mlock, munlock};
use rand::{RngCore, rngs::OsRng};
use std::path::PathBuf;
use zeroize::Zeroize;

/// The service name for OS's keyring key storage
const SERVICE: &str = "ENKRYPTIT";

use crate::{
    conversions::convert_key, errors::EnkryptitError,
    parameters::argon2id_parameters::argon2id_parameters,
};

use argon2::{Algorithm, Argon2, Version};

/// LockedKey Structure, that implements `unsafe` memorylock and memoryunlock (not used in the current state of Enkryptit, I created it at the beginning, but never used it).
struct LockedKey([u8; 32]);

impl LockedKey {
    /// Creates a new `LockedKey` from an existing key.
    fn new(key: [u8; 32]) -> Result<Self, EnkryptitError> {
        let locked = Self(key);

        unsafe {
            if mlock(locked.0.as_ptr() as *const _, locked.0.len()) != 0 {
                return Err(EnkryptitError::MemoryLockError);
            }
        }

        Ok(locked)
    }

    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for LockedKey {
    fn drop(&mut self) {
        unsafe {
            munlock(self.0.as_ptr() as *const _, self.0.len());
        }

        self.0.zeroize();
    }
}

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

/// Hashes the given `password` with argon2id and defined parameters, and returns `(hash, salt)` (`hash` will be used as the key, and `salt` will be inserted in metadata).
pub fn key_from_password(password: &str) -> Result<([u8; 32], [u8; 16]), EnkryptitError> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let params = argon2id_parameters()?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 32];

    argon2.hash_password_into(password.as_bytes(), &salt, &mut key)?;

    Ok((key, salt))
}

// Computes the argon2id hash of [`salt & password`] and returns the hash as the key.
pub fn derive_key(password: &str, salt: [u8; 16]) -> Result<[u8; 32], EnkryptitError> {
    let params = argon2id_parameters()?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 32];

    argon2.hash_password_into(password.as_bytes(), &salt, &mut key)?;

    Ok(key)
}

/// Public helper for generating a key and writing the key_file, given the name of the file to encrypt.
/// \
/// This function generates the key, but delegates the saving & writing to `save_key_in_file()`.
pub fn generate_key_and_write_file(filename: &str) -> Result<[u8; 32], EnkryptitError> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    save_key_in_file(filename, key)?;

    Ok(key)
}

/// Public helper for generating the key and storing it in the OS's keyring.
/// \
/// This function generates the key, but delegates the saving to `save_key_in_os`
pub fn generate_key_and_store_in_os(filename: &str) -> Result<[u8; 32], EnkryptitError> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    save_key_in_os(filename, key)?;

    Ok(key)
}

/// Public helper for using the master key... but unused in the current state of the program. (This function was extracted from `ZetaNet`)
pub fn with_master_key<F, R>(f: F, filename: &str) -> Result<R, EnkryptitError>
where
    F: FnOnce(&[u8; 32]) -> R,
{
    let key = LockedKey::new(load_key_from_os(filename)?)?;

    Ok(f(key.as_ref()))
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
