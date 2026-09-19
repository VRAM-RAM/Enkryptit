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
            size = Some(metadata.len());
            match infer_parallelism(metadata.len()) {
                Ok(p) => parallelism = Some(p),
                Err(e) => tracing::warn!("{}", e),
            }
        },
        Err(e) => tracing::warn!("{}", e)
    };

    let name = pathstd.file_name().map(|p| p.to_string_lossy().to_string());

    let directory = pathstd.parent().map(|p| p.to_string_lossy().to_string());

    InspectionReport::EncryptedFile { name, directory, size, version, compression_type: compression, predicted_parallelism_type: parallelism, keytype, nonce }
}