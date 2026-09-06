mod parameters;
use crate::{context::EnkryptitContext, encryption::{chunk_job::{decrypt::DecryptChunkJob, encrypt::EncryptChunkJob}, folder_encryption::{intern_archive_encryption::{multithread::{decrypt_multithreading_file_from_archive, encrypt_multithreading_file_into_archive}, single::{decrypt_single_file_from_archive, encrypt_single_file_into_archive}}, multithreading::parameters::FolderParallelizationParams}}, errors::EnkryptitError, key::EnkryptitKey, metadatas::FileEntry, parallelism::pool::EnkryptitPool, types::ParallelismType};

pub fn encrypt_folder_multithreading(folder_path: &str, key: EnkryptitKey, entries: &mut Vec<FileEntry>, offset: u64, context: &mut EnkryptitContext, archive_path: &str) -> Result<(), EnkryptitError> {
    let mut current_offset = offset;
    let mut entry_pool: Option<EnkryptitPool<EncryptChunkJob>> = None;

    let parameters = FolderParallelizationParams::compute();
    let mut folder_pool = 
    for entry in entries {

    };

    Ok(())
}

pub fn decrypt_folder_multithreading(archive_path: &str, dest_folder: &str, entries: &Vec<FileEntry>, key: EnkryptitKey, payload_offset: u64, version: u8, context: &mut EnkryptitContext) -> Result<(), EnkryptitError> {
    let mut pool: Option<EnkryptitPool<DecryptChunkJob>> = None;

    for entry in entries {
        
    }

    Ok(())
}