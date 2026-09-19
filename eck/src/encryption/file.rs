use std::{fs::File, io::BufReader, path::PathBuf};


use crate::{errors::EnkryptitError, types::CHUNK_SIZE};


/// A simple structure that contains the `BufReader` of a file, and its len.
pub struct EnkryptitFile {
    pub reader: BufReader<File>,
    pub len: u64,
    pub estimated_steps: u64,
}

/// An helper that reads a file and returns its `EnkryptitFile` :
/// - Retrieves its length
/// - Computes its `estimated_steps`
pub fn read_file(path: impl Into<PathBuf>) -> Result<EnkryptitFile, EnkryptitError> {
    let file = File::open(path.into())?;
    let len = file.metadata()?.len();
    Ok(EnkryptitFile {
        reader: BufReader::new(file),
        len,
        estimated_steps: len / CHUNK_SIZE as u64
    })
}

