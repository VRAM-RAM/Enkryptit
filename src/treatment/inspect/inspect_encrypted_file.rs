use std::{fs::{metadata},  path::Path};

use crate::{context::parallelism::infer_parallelism};
use postcard::from_bytes;

use crate::{metadatas::MetaDatas, treatment::inspect::InspectionReport};

pub fn inspect_encrypted_file(path: &str, meta: &[u8], version: u8) -> InspectionReport {
    let pathstd = Path::new(path);

    let mut size = None;
    let mut parallelism = None;
    let mut compression = None;
    let mut keytype = None;
    let mut nonce = None;

    match from_bytes::<MetaDatas>(meta) {
        Ok(m) => {
            compression = Some(m.compression);
            keytype = Some(m.key_type);
            nonce = Some(m.nonce);
        },
        Err(e) => tracing::warn!("{}", e)
    };

    match metadata(path) {
        Ok(metadata) => {
            size = Some(metadata.len() as u64);
            match infer_parallelism(metadata.len() as u64) {
                Ok(p) => parallelism = Some(p),
                Err(e) => tracing::warn!("{}", e),
            }
        },
        Err(e) => tracing::warn!("{}", e)
    };

    let name = match pathstd.file_name() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    let directory = match pathstd.parent() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    InspectionReport::EncryptedFile { name, directory, size, version, compression_type: compression, predicted_parallelism_type: parallelism, keytype, nonce }
}