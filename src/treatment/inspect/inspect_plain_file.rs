use std::fs::File;
use std::path::Path;
use infer::get_from_path;

use crate::{context::{compression::infer_compression, parallelism::infer_parallelism}, errors::EnkryptitError, treatment::inspect::InspectionReport};

pub fn inspect_plain_file(path: &str) -> Result<InspectionReport, EnkryptitError> {
    let pathstd = Path::new(path);
    let file = File::open(path)?;

    let type_ = get_from_path(path)?;

    let metadata = file.metadata()?;

    let extension = match type_.is_some() {
        true => type_.unwrap().extension(),
        false => {
            match pathstd.extension() {
                Some(ext) => &ext.to_string_lossy(),
                None => "No extension found."
            }
        }
    };

    let perms: Option<u32> = {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            Some(metadata.permissions().mode())
        }

        #[cfg(not(unix))]
        {
            None // No permissions on Windows
        } 
    };

    let mime = match type_.is_some() {
        true => type_.unwrap().mime_type().to_string(),
        false => "No mime type found".to_string(),
    };

    let name = match pathstd.file_name() {
        Some(p) => p.to_string_lossy(),
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    let directory = match pathstd.parent() {
        Some(p) => p,
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    
    let compression = infer_compression(path)?;
    let parallelism = infer_parallelism(metadata.len())?;

    Ok(InspectionReport::PlaintextFile { name: name.to_string(), directory: directory.to_string_lossy().to_string(), size: metadata.len() as usize, permissions: perms, extension: extension.to_string(), mime_extension: mime, predicted_compression_type: compression, predicted_parallelism_type: parallelism })
}