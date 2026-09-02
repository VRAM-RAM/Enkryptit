use walkdir::DirEntry;
use std::path::Path;
use crate::{errors::EnkryptitError};

pub fn collect_entry(dir_entry: &DirEntry, folder_path: &str) -> Result<(String, Option<u32>), EnkryptitError> {
    if dir_entry.path() == Path::new(folder_path) {
        return Err(EnkryptitError::DirectoryIsFolder)
    }

    let metadata = match dir_entry.metadata() {
        Ok(m) => m,
        Err(_) => return Err(EnkryptitError::FailedToReadMetadata(dir_entry.clone().into_path()))
    };

    // Only include regular files (skip directories - they'll be created on decrypt)
    if !metadata.is_file() && !metadata.file_type().is_symlink() {
        return Err(EnkryptitError::FileIsASymLink)
    }

    match dir_entry.path().strip_prefix(folder_path) {
        Ok(relative_path) => {
            let relative_path_str = relative_path.to_string_lossy().to_string();

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

             Ok((relative_path_str, perms))

        }

        Err(e) => return Err(EnkryptitError::StripPrefixError(e))
    }
}