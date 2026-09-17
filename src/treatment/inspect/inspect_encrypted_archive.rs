use std::{fs::{metadata},  path::Path};
use postcard::from_bytes;

use crate::{metadatas::FolderMetadata, treatment::inspect::InspectionReport};

pub fn inspect_encrypted_archive(path: &str, meta: &[u8], version: u8) -> InspectionReport {
    let pathstd = Path::new(path);

    let metadata = match metadata(path) {
        Ok(m) => Some(m),
        Err(e) => {
            tracing::warn!("{}", e);
            None
        }
    };

    let folder_meta: Option<FolderMetadata> = match from_bytes(meta) {
        Ok(fm) => Some(fm),
        Err(e) => {
            tracing::warn!("{}", e);
            None
        }
    };

    let name = match pathstd.file_name() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    let directory = match pathstd.parent() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    let size = match metadata.is_some() {
        true => Some(metadata.unwrap().len()),
        false => None
    };
    
    let mut entries_number = None;
    let mut keytype = None;
    
    match folder_meta.is_some() {
        true => {
            let meta = folder_meta.unwrap();
            entries_number = Some(meta.entries.len() as u64);
            keytype = Some(meta.key_type)
        }
        false => ()
    }

    InspectionReport::EncryptedArchive { name, directory, size, version, entries_number, keytype }
}