use crate::encryption::chunk_job::{decrypt::DecryptChunkJob, encrypt::EncryptChunkJob};
use crate::encryption::folder_encryption::intern_archive_encryption::{
    multithread::{
        decrypt_multithreading_file_from_archive, encrypt_multithreading_file_into_archive,
    },
    single::{decrypt_single_file_from_archive, encrypt_single_file_into_archive},
};
use crate::parallelism::pool::EnkryptitPool;
use crate::types::ParallelismType;
use crate::{
    context::EnkryptitContext, errors::EnkryptitError, key::EnkryptitKey, metadatas::FileEntry,
};
use std::path::Path;
pub mod multithread;
pub mod single;
use crate::diagnostic::EnkryptitOutput;
use std::fs::File;

pub fn treat_entry_encryption(
    pool: &mut Option<EnkryptitPool<EncryptChunkJob>>,
    folder_path: &str,
    entry: &mut FileEntry,
    current_offset: u64,
    context: &EnkryptitContext,
    archive_path: &str,
    key: &EnkryptitKey,
) -> Result<u64, EnkryptitError> {
    entry.offset = current_offset;

    let full_path = Path::new(folder_path).join(&entry.relative_path);

    let compression =
        match context.resolve_compression(full_path.to_str().unwrap_or(&entry.relative_path)) {
            Ok(compression) => compression,
            Err(e) => {
                tracing::warn!("Failed to infer compression type for an entry: {}", e);

                EnkryptitOutput::warning(
                "Could not determine the optimal compression type. Falling back to no compression."
            )
            .display();

                crate::types::CompressionType::NoComp
            }
        };

    let parallelism =
        match context.resolve_parallelism(full_path.to_str().unwrap_or(&entry.relative_path)) {
            Ok(parallelism) => parallelism,
            Err(e) => {
                tracing::warn!("Failed to infer parallelism type for an entry: {}", e);

                EnkryptitOutput::warning(
                    "Could not determine the optimal compression type. Falling back to single.",
                )
                .display();

                ParallelismType::Single
            }
        };

    let bytes_written = match parallelism {
        ParallelismType::Single => encrypt_single_file_into_archive(
            folder_path,
            &entry.relative_path,
            entry.file_nonce,
            compression,
            key.key_as_ref(),
            archive_path,
        )?,
        ParallelismType::MultiThread(num_threads) => {
            if pool
                .as_ref()
                .is_none_or(|pool| pool.size() != num_threads as usize)
            {
                *pool = Some(EnkryptitPool::new(num_threads as usize)?);
            }

            let pool = pool.as_ref().unwrap();

            encrypt_multithreading_file_into_archive(
                folder_path,
                &entry.relative_path,
                entry.file_nonce,
                compression,
                key.key_as_ref(),
                archive_path,
                pool,
                num_threads,
            )?
        }
        ParallelismType::Auto => unreachable!(
            "`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue."
        ),
    };

    Ok(bytes_written)
}

/// Everything `treat_entry_decryption` needs to decrypt a single archive entry.
pub struct EntryDecryptionContext<'a> {
    pub version: u8,
    pub dest_folder: &'a str,
    pub entry: &'a FileEntry,
    pub context: &'a mut EnkryptitContext,
    pub archive_path: &'a str,
    pub key: &'a EnkryptitKey,
    pub payload_offset: u64,
}

pub fn treat_entry_decryption(
    pool: &mut Option<EnkryptitPool<DecryptChunkJob>>,
    cx: EntryDecryptionContext,
) -> Result<(), EnkryptitError> {
    let EntryDecryptionContext {
        version,
        dest_folder,
        entry,
        context,
        archive_path,
        key,
        payload_offset,
    } = cx;

    let offset = if version >= 2 {
        entry.offset
    } else {
        payload_offset
    };

    let parallelism = match context.resolve_parallelism_with_size(entry.offset) {
        Ok(parallelism) => parallelism,
        Err(e) => {
            tracing::warn!("Failed to infer parallelism type for an entry: {}", e);

            EnkryptitOutput::warning(
                "Could not determine the optimal compression type. Falling back to single.",
            )
            .display();

            ParallelismType::Single
        }
    };

    let decrypt_result = match parallelism {
        ParallelismType::Single => decrypt_single_file_from_archive(
            archive_path,
            dest_folder,
            entry.permissions,
            &entry.relative_path,
            entry.file_nonce,
            entry.compression,
            key.key_as_ref(),
            offset,
        ),
        ParallelismType::MultiThread(num_threads) => {
            if pool
                .as_ref()
                .is_none_or(|pool| pool.size() != num_threads as usize)
            {
                *pool = Some(EnkryptitPool::new(num_threads as usize)?);
            }

            let pool = pool.as_ref().unwrap();

            decrypt_multithreading_file_from_archive(
                archive_path,
                dest_folder,
                entry.permissions,
                &entry.relative_path,
                entry.file_nonce,
                entry.compression,
                key.key_as_ref(),
                offset,
                pool,
                num_threads,
            )
        }
        ParallelismType::Auto => unreachable!(
            "`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue."
        ),
    };

    match decrypt_result {
        Ok(bytes_consumed) => {
            if version < 2 {
                // v1: we don't know the exact offset, but we tried.
                // For v1 archives this path is inherently unreliable.
                let _ = bytes_consumed;
            }
        }
        Err(e) => {
            tracing::warn!("Failed to encrypt an entry: {}", e);

            EnkryptitOutput::warning("Could not encrypt or decrypt an entry. Skipping.").display();

            // Create 0-byte placeholder file with original filename
            let placeholder = Path::new(dest_folder).join(&entry.relative_path);

            if let Some(parent) = placeholder.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let _ = File::create(placeholder);
        }
    }

    Ok(())
}
