use crate::encryption::encryption_primitives::generate_nonce;
use crate::errors::EnkryptitError;
use crate::metadatas::FileEntry;
use std::path::Path;
use walkdir::WalkDir;

/// Collect all files from directory tree using walkdir (follow symlinks via follow_links on Unix)
pub fn collect_folder_entries(folder_path: &str) -> Result<Vec<FileEntry>, EnkryptitError> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(folder_path).follow_links(true) {
        match entry {
            Ok(dir_entry) => {
                // Skip the root directory itself, only process files and subdirs
                if dir_entry.path() == Path::new(folder_path) {
                    continue;
                }

                let metadata = match dir_entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue, // Skip files we can't read metadata for
                };

                // Only include regular files (skip directories - they'll be created on decrypt)
                if !metadata.is_file() && !metadata.file_type().is_symlink() {
                    continue;
                }

                // Calculate relative path from folder root
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
                                None
                            } // No permissions on Windows
                        };

                        entries.push(FileEntry {
                            relative_path: relative_path_str,
                            offset: 0, // Set during encryption
                            permissions: perms,
                            file_nonce: generate_nonce(), // Unique per FILE -> one master nonce per file
                        });
                    }
                    Err(_) => continue, // Skip if can't calculate relative path
                }
            }
            Err(_) => continue, // Skip inaccessible files during collection
        }
    }

    Ok(entries)
}
