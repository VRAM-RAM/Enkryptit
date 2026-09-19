use crate::context::EnkryptitContext;
use crate::encryption::file_encryption::{decrypt_file, encrypt_file};
use crate::diagnostic::EnkryptitOutput;
use crate::types::KeyType::{self};

/// Public helper for encrypting a file (Converts Ok<>/EnkryptitError to Output)
pub fn encrypt_file_case(
    path: &str,
    context: &mut EnkryptitContext,
    key_type: &KeyType,
) -> EnkryptitOutput {
    match encrypt_file(path, key_type, context) {
        Ok(path) => EnkryptitOutput::success(format!("File was encrypted at {}", path)),
        Err(e) => e.into(),
    }
}

/// Public helper for decrypting a file (Converts Ok<>/EnkryptitError to Output)
pub fn decrypt_file_case(
    path: &str,
    meta: Vec<u8>,
    context: &mut EnkryptitContext,
    payload_offset: u64,
) -> EnkryptitOutput {
    match decrypt_file(path, &meta, payload_offset, context) {
        Ok(path) => EnkryptitOutput::success(format!("File was decrypted at {}", path)),
        Err(e) => e.into(),
    }
}
