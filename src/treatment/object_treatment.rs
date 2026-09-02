use crate::context::EnkryptitContext;
use crate::errors::EnkryptitError;
use crate::frontend::cli::Output;
use crate::metadatas::{ArchiveHeader, MAGIC};
use crate::parameters::params::EnkryptitParams;
use crate::treatment::file_case::{decrypt_file_case, encrypt_file_case};
use crate::treatment::folder_case::{decrypt_folder_case, encrypt_folder_case};
use postcard::from_bytes;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[allow(dead_code)]
/// ParsedFile enum. The result of a file parsing. If the file is encrypted with **Enkryptit!**, we return :
/// - Enkryptit { metadata, version, payload_offset, a boolean that indicates if it is a file or a folder }
/// \
/// Else, we return :
/// - Plain
pub enum ParsedFile {
    Enkryptit {
        meta: Vec<u8>,
        version: u8,
        payload_offset: u64,
        is_folder_archive: bool,
    },
    Plain,
}

/// Exposed function for treating an undetermined object.
/// If the path is a folder, we delegate it to `encrypt_folder_case()`.
/// \
/// Else, we parse the file, and `match` the `ParsedFile` result.
pub fn treat_object(
    parameters: &EnkryptitParams,
    path: &str,
    context: &mut EnkryptitContext,
) -> Result<Output, EnkryptitError> {
    let keytype = parameters.key_params.to_type();

    if Path::new(&path).is_dir() {
        return encrypt_folder_case(path, context, &keytype);
    }

    match read_file(&path) {
        Ok(ParsedFile::Enkryptit {
            meta,
            payload_offset,
            is_folder_archive,
            version,
            ..
        }) => {
            if is_folder_archive {
                decrypt_folder_case(path, context, meta, payload_offset, version)
            } else {
                decrypt_file_case(path, meta, context, payload_offset)
            }
        }

        Ok(ParsedFile::Plain) => encrypt_file_case(path, context, &keytype),

        Err(_) => Ok(Output::CorruptedFile),
    }
}

/// Private function that reads the file of the given path, and parses it.
/// \
/// It :
/// - Tries to read the header's length
/// - Tries to read the header data
/// - Deserialize the header data
/// - Compare the Magic number
/// - Reads the metadata (two different ways : at the beginning of the file if the version is 1, at the end if the version is 2)
/// - Returns the ParsedFile result
fn read_file(path: &str) -> Result<ParsedFile, EnkryptitError> {
    let file = File::open(path)?;
    let file_len = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    // Try to read the header length
    let mut len_buf = [0u8; 1];
    if reader.read_exact(&mut len_buf).is_err() {
        return Ok(ParsedFile::Plain);
    }

    let header_len = len_buf[0] as usize;

    if header_len == 0 || header_len > 1024 {
        return Ok(ParsedFile::Plain);
    }

    let mut header_bytes = vec![0u8; header_len];
    if reader.read_exact(&mut header_bytes).is_err() {
        return Ok(ParsedFile::Plain);
    }

    let archive_header: ArchiveHeader = match from_bytes(&header_bytes) {
        Ok(h) => h,
        Err(_) => return Ok(ParsedFile::Plain),
    };

    if archive_header.magic != MAGIC {
        return Ok(ParsedFile::Plain);
    }

    let meta_len = archive_header.meta_len as usize;
    let mut meta = vec![0u8; meta_len];

    if archive_header.is_folder_archive && archive_header.version >= 2 {
        // v2 folder archive: metadata is at the end of the file
        if file_len < meta_len as u64 {
            return Err(EnkryptitError::CorruptedFile);
        }
        let meta_start = file_len - meta_len as u64;
        reader.seek(SeekFrom::Start(meta_start))?;
        if reader.read_exact(&mut meta).is_err() {
            return Err(EnkryptitError::CorruptedFile);
        }
    } else {
        // v1 or single-file: metadata is right after header
        if reader.read_exact(&mut meta).is_err() {
            return Err(EnkryptitError::CorruptedFile);
        }
    }

    let payload_offset = (1 + header_len as u64 + meta_len as u64) as u64;

    Ok(ParsedFile::Enkryptit {
        meta,
        version: archive_header.version,
        payload_offset,
        is_folder_archive: archive_header.is_folder_archive,
    })
}
