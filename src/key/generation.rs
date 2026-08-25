use rand::{RngCore, rngs::OsRng};
use crate::errors::EnkryptitError;
use crate::key::storage::{save_key_in_file, save_key_in_os};

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