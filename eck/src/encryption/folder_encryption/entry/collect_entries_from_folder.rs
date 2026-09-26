use crate::encryption::encryption_primitives::generate_nonce;
use crate::encryption::folder_encryption::entry::collect_entry::collect_entry;
use crate::errors::EnkryptitError;
use crate::metadatas::FileEntry;
use crate::{context::EnkryptitContext, diagnostic::EnkryptitOutput};
use std::path::PathBuf;
use walkdir::WalkDir;

/// Collect all files from directory tree using walkdir (follow symlinks via follow_links on Unix)
pub fn collect_folder_entries(
    folder_path: &str,
    context: &EnkryptitContext,
) -> Result<Vec<FileEntry>, EnkryptitError> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(folder_path).follow_links(true) {
        match entry {
            Ok(dir_entry) => {
                let (relative_path, permissions) = match collect_entry(&dir_entry, folder_path) {
                    Ok((rp, perm)) => (rp, perm),
                    Err(e) => {
                        tracing::warn!(
                            "An error occurred when collecting an entry. Here's the detailed error : {}",
                            e
                        );
                        EnkryptitOutput::warning(
                            "An error occurred when collecting an entry. We skip it.",
                        )
                        .display();
                        continue;
                    }
                };

                let full_path = PathBuf::from(folder_path).join(&relative_path);
                let full_path_str = match full_path.to_str() {
                    Some(path) => path,
                    None => {
                        tracing::warn!(
                            "An error occurred when converting full path (PathBuf) to &str."
                        );
                        EnkryptitOutput::warning(
                            "An error occurred when treating an entry. We skip it.",
                        )
                        .display();
                        continue;
                    }
                };

                let compression = match context.resolve_compression(full_path_str) {
                    Ok(compression) => compression,
                    Err(e) => {
                        tracing::warn!(
                            "An error occurred when infering the compression type of an entry. Here's the detailed error : {}",
                            e
                        );
                        EnkryptitOutput::warning(
                            "An error occurred when treating an entry. We skip it.",
                        )
                        .display();
                        continue;
                    }
                };

                entries.push(FileEntry {
                    relative_path,
                    offset: 0, // Set during encryption
                    permissions,
                    compression,                  // Set during encryption too
                    file_nonce: generate_nonce(), // Unique per FILE -> one master nonce per file
                });
            }

            Err(e) => {
                tracing::warn!(
                    "An error occurred when infering the compression type of an entry. Here's the detailed error : {}",
                    e
                );
                EnkryptitOutput::warning("An error occurred when treating an entry. We skip it.")
                    .display();
                continue;
            } // Skip inaccessible files during collection
        }
    }

    Ok(entries)
}
