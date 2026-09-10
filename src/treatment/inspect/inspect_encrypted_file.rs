use std::{fs::File, path::Path};

use crate::context::parallelism::infer_parallelism;
use postcard::from_bytes;

use crate::{errors::EnkryptitError, metadatas::MetaDatas, treatment::inspect::InspectionReport};

pub fn inspect_encrypted_file(path: &str, meta: &[u8], version: u8) -> Result<InspectionReport, EnkryptitError> {
    let metadata: MetaDatas = from_bytes(meta)?;

    let pathstd = Path::new(path);
    let file = File::open(path)?;
    let file_metadata = file.metadata()?;

    let name = match pathstd.file_name() {
        Some(p) => p.to_string_lossy(),
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    let directory = match pathstd.parent() {
        Some(p) => p,
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    let parallelism = infer_parallelism(file_metadata.len())?;


    Ok(InspectionReport::EncryptedFile { name: name.to_string(), directory: directory.to_string_lossy().to_string(), size: file_metadata.len() as usize, version, compression_type: metadata.compression, predicted_parallelism_type: parallelism, keytype: metadata.key_type, nonce: metadata.nonce })
}