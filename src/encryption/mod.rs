pub mod chunk_job;
pub mod encryption_flow;
pub mod encryption_primitives;
pub mod file_encryption;
pub mod folder_encryption;
pub mod file;

use std::sync::Arc;
use chacha20poly1305::{XChaCha20Poly1305, KeyInit};

/// Small helper that returns an `Arc<XChaCha20Poly1305<T>>` where `<T>` is the *key*.
pub fn shared_cipher(key: &[u8; 32]) -> Arc<XChaCha20Poly1305> {
    Arc::new(XChaCha20Poly1305::new(key.into()))
}
