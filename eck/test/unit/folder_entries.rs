//! Folder entry collection tests (DAY-11)
//!
//! Exercises `collect_entry` (including its error variants) and
//! `collect_folder_entries`, with a focus on the per-entry `CompressionType`
//! resolution that replaced the old folder-level compression field.

use eck::context::EnkryptitContext;
use eck::encryption::folder_encryption::entry::collect_entries_from_folder::collect_folder_entries;
use eck::encryption::folder_encryption::entry::collect_entry::collect_entry;
use eck::errors::EnkryptitError;
use eck::types::{CompressionType, Interface, ParallelismType};
use std::fs;
use tempfile::TempDir;
use walkdir::WalkDir;

const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0DIHDR";
const WAV: &[u8] = b"RIFF\x00\x00\x00\x00WAVE";
const XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><root/>";

#[cfg(test)]
mod tests {
    use super::*;

    fn root_dir() -> (TempDir, String) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("root");
        fs::create_dir(&root).unwrap();
        let path = root.to_str().unwrap().to_string();
        (tmp, path)
    }

    // --- collect_entry ---

    #[test]
    fn collect_entry_returns_relative_path_and_permissions() {
        let (_tmp, root) = root_dir();
        fs::write(format!("{}/alpha.txt", root), b"x").unwrap();

        for entry in WalkDir::new(&root) {
            let entry = entry.unwrap();
            if entry.path().to_str().unwrap() == root {
                continue;
            }
            let (rel, perms) = collect_entry(&entry, &root).unwrap();
            assert_eq!(rel, "alpha.txt");
            #[cfg(unix)]
            assert!(perms.is_some(), "unix mode must be captured");
            return;
        }
        panic!("file entry never visited");
    }

    #[test]
    fn collect_entry_root_dir_is_directory_is_folder_error() {
        let (_tmp, root) = root_dir();
        let root_entry = WalkDir::new(&root).into_iter().next().unwrap().unwrap();
        match collect_entry(&root_entry, &root) {
            Err(EnkryptitError::DirectoryIsFolder) => {}
            other => panic!("expected DirectoryIsFolder, got {other:?}"),
        }
    }

    #[test]
    fn collect_entry_rejects_directory_entries() {
        let (_tmp, root) = root_dir();
        fs::create_dir(format!("{}/subdir", root)).unwrap();

        for entry in WalkDir::new(&root) {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() && entry.path().to_str().unwrap() != root {
                match collect_entry(&entry, &root) {
                    Err(EnkryptitError::FileIsASymLink) => {}
                    other => panic!("expected FileIsASymLink for a directory, got {other:?}"),
                }
                return;
            }
        }
        panic!("directory entry never visited");
    }

    #[cfg(unix)]
    #[test]
    fn collect_entry_accepts_symlink_to_file() {
        use std::os::unix::fs::symlink;

        let (_tmp, root) = root_dir();
        let target = format!("{root}.target");
        fs::write(&target, b"payload").unwrap();
        symlink(&target, format!("{root}/link.bin")).unwrap();

        for entry in WalkDir::new(&root).follow_links(true) {
            let entry = entry.unwrap();
            if entry.path().to_str().unwrap() == root {
                continue;
            }
            let (rel, _perms) = collect_entry(&entry, &root).unwrap();
            assert_eq!(rel, "link.bin");
            return;
        }
        panic!("symlink entry never visited");
    }

    // --- collect_folder_entries ---

    #[test]
    fn collect_folder_entries_resolves_concrete_compression_per_entry() {
        let (_tmp, root) = root_dir();
        fs::write(format!("{root}/doc.xml"), XML).unwrap();
        fs::write(format!("{root}/sound.wav"), WAV).unwrap();
        fs::write(format!("{root}/img.png"), PNG).unwrap();

        let context = EnkryptitContext::new(Interface::Cli, None, CompressionType::Auto, ParallelismType::Auto);

        let entries = collect_folder_entries(&root, &context).unwrap();
        assert_eq!(entries.len(), 3);

        // Each entry must carry a concrete (non-Auto) compression, matching what
        // the context would infer for that exact file (this is the behaviour that
        // replaces the removed folder-level compression field).
        let mut compressions = Vec::new();
        for entry in &entries {
            assert_ne!(
                entry.compression,
                CompressionType::Auto,
                "Auto must never leak into an entry: {}",
                entry.relative_path
            );
            let full = std::path::Path::new(&root).join(&entry.relative_path);
            let expected = context.resolve_compression(full.to_str().unwrap()).unwrap();
            assert_eq!(entry.compression, expected, "entry: {}", entry.relative_path);

            assert_eq!(entry.offset, 0, "offset is set during encryption");
            assert_eq!(entry.file_nonce.len(), 24, "one master nonce per entry");
            #[cfg(unix)]
            assert!(entry.permissions.is_some());
            compressions.push(entry.compression);
        }

        // XML -> Zstd, WAV -> Lz4, PNG -> NoComp under the documented inference.
        let mut set: Vec<CompressionType> = compressions;
        set.sort_by_key(|c| format!("{c:?}"));
        assert_eq!(set, vec![CompressionType::Lz4, CompressionType::NoComp, CompressionType::Zstd]);
    }

    #[test]
    fn collect_folder_entries_empty_folder_is_empty() {
        let (_tmp, root) = root_dir();
        let context = EnkryptitContext::new(Interface::Cli, None, CompressionType::Zstd, ParallelismType::Single);
        assert!(collect_folder_entries(&root, &context).unwrap().is_empty());
    }

    #[test]
    fn collect_folder_entries_keeps_nested_paths_relative() {
        let (_tmp, root) = root_dir();
        fs::create_dir_all(format!("{}/a/b", root)).unwrap();
        fs::write(format!("{root}/a/b/c.txt"), b"x").unwrap();

        let context = EnkryptitContext::new(Interface::Cli, None, CompressionType::Xz, ParallelismType::Single);
        let entries = collect_folder_entries(&root, &context).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].relative_path, "a/b/c.txt");
    }
}