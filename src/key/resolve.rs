use crate::key::generation::{generate_key_and_store_in_os, generate_key_and_write_file};
use crate::{errors::EnkryptitError, types::Mode};
use crate::key::storage::{load_key_from_file, load_key_from_os};

pub fn resolve_key_from_file(
    mode: Mode,
    path: &str,
) -> Result<[u8; 32], EnkryptitError> {
    match load_key_from_file(path) {
        Ok(key) => Ok(key),
        Err(_) => match mode {
            Mode::Encrypting => generate_key_and_write_file(path),
            Mode::Decrypting => Err(EnkryptitError::KeyNotFound),
        },
    }
}

pub fn resolve_key_from_os(
    mode: Mode,
    path: &str,
) -> Result<[u8; 32], EnkryptitError> {
    match load_key_from_os(path) {
        Ok(key) => Ok(key),
        Err(_) => match mode {
            Mode::Encrypting => generate_key_and_store_in_os(path),
            Mode::Decrypting => Err(EnkryptitError::KeyNotFound),
        }
    }
}