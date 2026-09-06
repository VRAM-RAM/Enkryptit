pub mod entry;
pub mod multithreading;
pub mod intern_archive_encryption;
pub mod single;

use crate::context::EnkryptitContext;
use crate::encryption::folder_encryption::entry::collect_entries_from_folder::collect_folder_entries;
use crate::encryption::folder_encryption::single::{decrypt_folder_single, encrypt_folder_single};
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, FolderMetadata};
use crate::types::KeyType;
use crate::types::Mode;
use postcard::from_bytes;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};

/// Encrypt a folder into a single .encky archive file (v2 format: metadata at the end)
pub fn encrypt_folder(
    folder_path: &str,
    context: &mut EnkryptitContext,
    keytype: &KeyType,
) -> Result<String, EnkryptitError> {
    // Creates the Enkryptit key (resolves both key and keytype)
    let enkryptit_key = EnkryptitKey::resolve(Mode::Encrypting, keytype, context, folder_path)?;

    // Step 1: Collect all file entries from directory tree (follow symlinks)
    let mut entries = collect_folder_entries(folder_path, context)?;

    if entries.is_empty() {
        return Err(EnkryptitError::FileError);
    }

    // Step 2: Build FolderMetadata (offsets will be filled after encryption)
    let mut folder_meta = FolderMetadata::new(
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
    encrypt_folder_single(folder_path, enkryptit_key, &mut entries, data_start, context, &archive_path)?;

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
    let entries = metadatas.entries;

    // Then, we resolve the key & keytype and create a new EnkryptitKey
    let enkryptit_key = EnkryptitKey::resolve(Mode::Decrypting, &metadatas.key_type, context, archive_path)?;

    // Step 3: Create destination directory structure
    let dest_folder = archive_path.strip_suffix(".encky").unwrap_or(archive_path);
    std::fs::create_dir_all(dest_folder)?;

    // Step 4: Decrypt each file independently - continue on failure!
    decrypt_folder_single(archive_path, dest_folder, &entries, enkryptit_key, payload_offset, version, context)?;

    Ok(dest_folder.to_string())
}
