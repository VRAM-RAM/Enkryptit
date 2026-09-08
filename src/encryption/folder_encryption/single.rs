use gradient_bar::GradientProgressBar;

use crate::{context::EnkryptitContext, encryption::{chunk_job::{decrypt::DecryptChunkJob, encrypt::EncryptChunkJob}, folder_encryption::intern_archive_encryption::{treat_entry_decryption, treat_entry_encryption}}, errors::EnkryptitError, key::EnkryptitKey, metadatas::FileEntry, parallelism::pool::EnkryptitPool};

pub fn encrypt_folder_single(folder_path: &str, key: EnkryptitKey, entries: &mut Vec<FileEntry>, offset: u64, context: &mut EnkryptitContext, archive_path: &str) -> Result<(), EnkryptitError> {
    let mut current_offset = offset;
    let mut pool: Option<EnkryptitPool<EncryptChunkJob>> = None;

    let pb = GradientProgressBar::with_total_steps(entries.len() as u64, "Encrypting folder...");

    for entry in entries {
        current_offset += treat_entry_encryption(&mut pool, folder_path, entry, current_offset, context, archive_path, &key)?;
        pb.inc(1);
    }

    pb.finish();

    Ok(())
}

pub fn decrypt_folder_single(archive_path: &str, dest_folder: &str, entries: &Vec<FileEntry>, key: EnkryptitKey, payload_offset: u64, version: u8, context: &mut EnkryptitContext) -> Result<(), EnkryptitError> {
    let mut pool: Option<EnkryptitPool<DecryptChunkJob>> = None;

    let pb = GradientProgressBar::with_total_steps(entries.len() as u64, "Decrypting archive...");

    for entry in entries {
        treat_entry_decryption(&mut pool, version, dest_folder, &entry, context, archive_path, &key, payload_offset)?;
        pb.inc(1);
    }

    pb.finish();

    Ok(())
}