//! `read_file` helper tests (DAY-12)
//!
//! Exercises `encryption::file::read_file` / `EnkryptitFile` : length probing
//! and the `estimated_steps` computation used by the multithreaded
//! folder/file intern archive encryption.

use eck::encryption::file::read_file;
use eck::types::CHUNK_SIZE;
use std::fs;
use tempfile::NamedTempFile;

const MIB: usize = 1024 * 1024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_reports_len() {
        let tmp = NamedTempFile::new().unwrap();
        fs::write(tmp.path(), vec![0xAB; 10 * MIB as usize]).unwrap();

        let file = read_file(tmp.path()).unwrap();
        assert_eq!(file.len, 10 * MIB as u64);
    }

    #[test]
    fn read_file_estimates_steps_by_chunk_size() {
        let tmp = NamedTempFile::new().unwrap();
        // Two and a half chunks worth of content -> integer division.
        fs::write(tmp.path(), vec![0u8; 2 * MIB as usize]).unwrap();

        let file = read_file(tmp.path()).unwrap();
        assert_eq!(file.estimated_steps, (2 * MIB as u64) / CHUNK_SIZE as u64);
    }

    #[test]
    fn read_file_exact_chunk_multiple_is_one_step() {
        let tmp = NamedTempFile::new().unwrap();
        fs::File::create(tmp.path())
            .unwrap()
            .set_len(CHUNK_SIZE as u64)
            .unwrap();

        let file = read_file(tmp.path()).unwrap();
        assert_eq!(file.len, CHUNK_SIZE as u64);
        assert_eq!(file.estimated_steps, 1);
    }

    #[test]
    fn read_file_zero_length() {
        let tmp = NamedTempFile::new().unwrap();

        let file = read_file(tmp.path()).unwrap();
        assert_eq!(file.len, 0);
        assert_eq!(file.estimated_steps, 0);
    }

    #[test]
    fn read_file_missing_path_errors() {
        assert!(read_file("/definitely/not/a/real/path.encky").is_err());
    }
}