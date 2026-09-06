use std::path::Path;
use std::fs::File;
use crate::{context::EnkryptitContext, encryption::folder_encryption::intern_archive_encryption::single::{decrypt_single_file_from_archive, encrypt_single_file_into_archive}, errors::EnkryptitError, key::EnkryptitKey, metadatas::FileEntry};

pub fn encrypt_folder_single(folder_path: &str, key: EnkryptitKey, entries: &mut Vec<FileEntry>, offset: u64, context: &mut EnkryptitContext, archive_path: &str) -> Result<(), EnkryptitError> {
    
    let mut current_offset = offset;
    
    for entry in entries {
        entry.offset = current_offset;

        let full_path = Path::new(folder_path).join(&entry.relative_path);

        let compression = match context.resolve_compression(full_path.to_str().unwrap_or(&entry.relative_path)) {
            Ok(compression) => compression,
            Err(_) => {eprintln!("Error in COMPRESSION TYPE RESOLUTION !!!!!!"); continue}
            // TODO! Add a smooth skipping + a warning message and logging for advanced users
        };

        let bytes_written = encrypt_single_file_into_archive(
            folder_path, 
            &entry.relative_path, 
            entry.file_nonce, 
            compression, 
            key.key_as_ref(), 
            archive_path
        )?;

        current_offset += bytes_written;
    }

    Ok(())
}

pub fn decrypt_folder_single(archive_path: &str, dest_folder: &str, entries: &Vec<FileEntry>, key: EnkryptitKey, payload_offset: u64, version: u8) -> Result<(), EnkryptitError> {
    for entry in entries {
        let offset = if version >= 2 {
            entry.offset
        } else {
            payload_offset
        };

        let decrypt_result = decrypt_single_file_from_archive(
            archive_path,
            dest_folder,
            entry.permissions,
            &entry.relative_path,
            entry.file_nonce,
            entry.offset, // used for progress bar display
            entry.compression,
            key.key_as_ref(),
            offset,
        );

        match decrypt_result {
            Ok(bytes_consumed) => {
                if version < 2 {
                    // v1: we don't know the exact offset, but we tried.
                    // For v1 archives this path is inherently unreliable.
                    let _ = bytes_consumed;
                }
            }
            Err(e) => {
                // TODO! Add a clean skipping system, with logging for advanced users 
                eprintln!(
                    "[WARNING] Failed to decrypt {}: {} - creating placeholder",
                    entry.relative_path, e
                );

                // Create 0-byte placeholder file with original filename
                let placeholder = Path::new(dest_folder).join(&entry.relative_path);
                if let Some(parent) = placeholder.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                let _ = File::create(placeholder);

                continue;
            }
        }
    }

    Ok(())
}