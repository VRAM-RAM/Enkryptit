use std::{fs::{metadata},  path::Path};


use crate::{errors::EnkryptitError, treatment::inspect::InspectionReport};

pub fn inspect_plain_folder(path: &str) -> Result<InspectionReport, EnkryptitError> {
    let pathstd = Path::new(path);

    let metadata = metadata(path)?;
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

    let name = match pathstd.file_name() {
        Some(p) => p.to_string_lossy(),
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };

    let directory = match pathstd.parent() {
        Some(p) => p,
        None => return Err(EnkryptitError::PathIsIncorrect(path.to_string()))
    };
    
    Ok(InspectionReport::Folder { name: name.to_string() , directory: directory.to_string_lossy().to_string(), size: metadata.len() as usize, permissions: perms})
}