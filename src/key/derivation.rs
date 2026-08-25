use rand::{RngCore, rngs::OsRng};
use crate::{
    errors::EnkryptitError,
    parameters::argon2id_parameters::argon2id_parameters,
};

use argon2::{Algorithm, Argon2, Version};

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
