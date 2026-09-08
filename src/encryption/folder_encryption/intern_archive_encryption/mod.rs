use std::path::Path;
use crate::encryption::chunk_job::{decrypt::DecryptChunkJob, encrypt::EncryptChunkJob};
use crate::parallelism::pool::EnkryptitPool;
use crate::types::ParallelismType;
use crate::{context::EnkryptitContext, errors::EnkryptitError, key::EnkryptitKey, metadatas::FileEntry};
use crate::encryption::folder_encryption::intern_archive_encryption::{single::{encrypt_single_file_into_archive, decrypt_single_file_from_archive}, multithread::{encrypt_multithreading_file_into_archive, decrypt_multithreading_file_from_archive}};
pub mod single;
pub mod multithread;
use std::fs::File;

pub fn treat_entry_encryption(pool: &mut Option<EnkryptitPool<EncryptChunkJob>>, folder_path: &str, entry: &mut FileEntry, current_offset: u64, context: &EnkryptitContext, archive_path: &str, key: &EnkryptitKey) -> Result<u64, EnkryptitError> {
    entry.offset = current_offset;

    let full_path = Path::new(folder_path).join(&entry.relative_path);

    let compression = match context.resolve_compression(full_path.to_str().unwrap_or(&entry.relative_path)) {
        Ok(compression) => compression,
        Err(_) => {eprintln!("Error in COMPRESSION TYPE RESOLUTION !!!!!!"); return Ok(0);}
        // TODO! Add a smooth skipping + a warning message and logging for advanced users
    };

    let parallelism = match context.resolve_parallelism(full_path.to_str().unwrap_or(&entry.relative_path)) {
        Ok(parallelism) => parallelism,
        Err(_) => {
            eprintln!("Error in Parallelism TYPE RESOLUTION !!!!!!");
            // TODO! Add a smooth skipping + a warning message and logging for advanced users
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
                archive_path
            )?,
            ParallelismType::MultiThread(num_threads) => {
                if pool.as_ref().map_or(true, |pool| pool.size() != num_threads as usize) {
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
            ParallelismType::Auto => unreachable!("`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue.")

        };

    Ok(bytes_written)
}


pub fn treat_entry_decryption(pool: &mut Option<EnkryptitPool<DecryptChunkJob>>, version: u8, dest_folder: &str, entry: &FileEntry, context: &mut EnkryptitContext, archive_path: &str, key: &EnkryptitKey, payload_offset: u64) -> Result<(), EnkryptitError> {
    let offset = if version >= 2 {
        entry.offset
    } else {
        payload_offset
    };

    let parallelism = match context.resolve_parallelism_with_size(entry.offset) {
        Ok(parallelism) => parallelism,
        Err(_) => {
            eprintln!("Error in Parallelism TYPE RESOLUTION !!!!!!");
            // TODO! Add a smooth skipping + a warning message and logging for advanced users
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
                if pool.as_ref().map_or(true, |pool| pool.size() != num_threads as usize) {
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
                    num_threads
                )
            }
            ParallelismType::Auto => unreachable!("`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue.")
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
        }
    }

    Ok(())
}