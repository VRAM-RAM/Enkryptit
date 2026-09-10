use std::{fs::{metadata},  path::Path};
use postcard::from_bytes;

use crate::{errors::EnkryptitError, metadatas::FolderMetadata, treatment::inspect::InspectionReport};

pub fn inspect_encrypted_archive(path: &str, meta: &[u8], version: u8) -> Result<InspectionReport, EnkryptitError> {
    let pathstd = Path::new(path);
    let metadata = metadata(path)?;

    let folder_meta: FolderMetadata = from_bytes(meta)?;

    let name = match pathstd.file_name() {
        Some(p) => p.to_string_lossy(),
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    let directory = match pathstd.parent() {
        Some(p) => p,
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    Ok(InspectionReport::EncryptedArchive { name: name.to_string(), directory: directory.to_string_lossy().to_string(), size: metadata.len() as usize, version, entries_number: folder_meta.entries.len() as u64, keytype: folder_meta.key_type })
}