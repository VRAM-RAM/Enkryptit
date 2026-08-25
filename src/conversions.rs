use crate::errors::EnkryptitError;

/// Simple public helper for converting a `key` from a `Vec<u8>` to an `[u8; 32]` array safely.
pub fn convert_key(vec: Vec<u8>) -> Result<[u8; 32], EnkryptitError> {
    Ok(vec
        .as_slice()
        .try_into()
        .map_err(|_| EnkryptitError::InvalidKeyLength)?)
}
