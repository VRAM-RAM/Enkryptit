pub mod chunk_job;
pub mod encryption_flow;
pub mod encryption_primitives;
pub mod file;
pub mod file_encryption;
pub mod folder_encryption;

use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use std::sync::Arc;

/// Small helper that returns an `Arc<XChaCha20Poly1305<T>>` where `<T>` is the *key*.
pub fn shared_cipher(key: &[u8; 32]) -> Arc<XChaCha20Poly1305> {
    Arc::new(XChaCha20Poly1305::new(key.into()))
}
