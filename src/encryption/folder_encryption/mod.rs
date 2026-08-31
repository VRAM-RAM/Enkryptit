pub mod entries;
pub mod intern_archive_encryption;

use crate::context::EnkryptitContext;
use crate::encryption::folder_encryption::entries::collect_folder_entries;
use crate::encryption::folder_encryption::intern_archive_encryption::{
    decrypt_single_file_from_archive, encrypt_single_file_into_archive,
};
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, FolderMetadata};
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType;
use crate::types::Mode;
use postcard::from_bytes;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

/// Encrypt a folder into a single .encky archive file (v2 format: metadata at the end)
pub fn encrypt_folder(
    folder_path: &str,
    parameters: &EnkryptitParams,
    context: &mut EnkryptitContext,
    keytype: &KeyType,
) -> Result<String, EnkryptitError> {
    // Creates the Enkryptit key (resolves both key and keytype)
    let enkryptit_key = EnkryptitKey::resolve(Mode::Encrypting, keytype, context, folder_path)?;

    // Step 1: Collect all file entries from directory tree (follow symlinks)
    let mut entries = collect_folder_entries(folder_path)?;

    if entries.is_empty() {
        return Err(EnkryptitError::FileError);
    }

    // Step 2: Build FolderMetadata (offsets will be filled after encryption)
    let mut folder_meta = FolderMetadata::new(
        parameters.compression,
        enkryptit_key.key_type_as_ref().clone(),
    );

    for entry in &entries {
        folder_meta.entries.push(entry.clone());
    }

    // Step 3: Create .encky archive with a fixed-size header region
    // The header is serialized, then zero-padded to HEADER_REGION_SIZE bytes
    // so that future in-place updates never change the region size.
    const HEADER_REGION_SIZE: usize = 64;
    let archive_path = format!("{}.encky", folder_path);

    {
        // Creates the archive
        let mut archive_file = BufWriter::new(File::create(&archive_path)?);

        // the header placeholder
        let placeholder_header = ArchiveHeader::new(true, 0);
        // We serialize it
        let mut header_bytes = placeholder_header.pack()?;
        // And resize with HEADER_REGION_SIZE
        header_bytes.resize(HEADER_REGION_SIZE, 0);

        // We write the region size
        archive_file.write_all(&[HEADER_REGION_SIZE as u8])?;
        // And the header
        archive_file.write_all(&header_bytes)?;
    }

    // Compute the beginning offset
    let data_start: u64 = 1 + HEADER_REGION_SIZE as u64;

    // Step 4: Encrypt each file, tracking offsets
    let mut current_offset = data_start;

    // We iterate on each entry
    for entry in &mut entries {
        entry.offset = current_offset;

        // For more informations, please refeer to `encrypt_single_file_into_archive()`
        let bytes_written = encrypt_single_file_into_archive(
            folder_path,
            &entry.relative_path,
            entry.file_nonce,
            parameters.compression,
            enkryptit_key.key_as_ref(),
            &archive_path,
        )?;

        // We update the offset
        current_offset += bytes_written;
    }

    // Step 5: Rebuild metadata with correct offsets and write at end of archive
    folder_meta.entries.clear();
    for entry in &entries {
        folder_meta.entries.push(entry.clone());
    }

    let serialized_meta = folder_meta.pack()?;
    let meta_len = serialized_meta.len() as u32;

    {
        let mut archive_file = BufWriter::new(File::options().append(true).open(&archive_path)?);

        archive_file.write_all(&serialized_meta)?;
    }

    // Step 6: Seek back and update meta_len in the fixed-size header region
    {
        let final_header = ArchiveHeader::new(true, meta_len);
        let mut final_header_bytes = final_header.pack()?;
        final_header_bytes.resize(HEADER_REGION_SIZE, 0);

        let mut archive_file = File::options().read(true).write(true).open(&archive_path)?;

        archive_file.seek(SeekFrom::Start(1))?;
        archive_file.write_all(&final_header_bytes)?;
    }

    Ok(archive_path)
}

/// Decrypt a folder archive (.encky file) back to original structure  
pub fn decrypt_folder(
    archive_path: &str,
    meta_bytes: &[u8],
    payload_offset: u64,
    version: u8,
    context: &mut EnkryptitContext,
) -> Result<String, EnkryptitError> {
    // First, we deserialize the metadata
    let metadatas: FolderMetadata = from_bytes(meta_bytes)?;
    let compression_type = metadatas.compression;
    let entries = metadatas.entries;

    // Then, we resolve the key & keytype and create a new EnkryptitKey
    let enkryptit_key =
        EnkryptitKey::resolve(Mode::Decrypting, &metadatas.key_type, context, archive_path)?;

    // Step 3: Create destination directory structure
    let dest_folder = archive_path.strip_suffix(".encky").unwrap_or(archive_path);
    std::fs::create_dir_all(dest_folder)?;

    // Step 4: Decrypt each file independently - continue on failure!
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
            compression_type,
            enkryptit_key.key_as_ref(),
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

    Ok(dest_folder.to_string())
}
