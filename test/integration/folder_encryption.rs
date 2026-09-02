#[cfg(test)]
mod tests {
    use eck::context::EnkryptitContext;
    use eck::encryption::folder_encryption::entries::collect_folder_entries;
    use eck::encryption::folder_encryption::{decrypt_folder, encrypt_folder};
    use eck::metadatas::{ArchiveHeader, FolderMetadata};
    use eck::parameters::params::EnkryptitParams;
    use eck::types::{CompressionType, KeyParams, KeyType};
    use postcard::from_bytes;
    use std::fs;
    use std::io::{BufReader, Read, Seek, SeekFrom};
    use tempfile::TempDir;

    fn test_params_single(compression: CompressionType) -> EnkryptitParams {
        EnkryptitParams {
            compression,
            key_params: KeyParams::File,
            parallelism: eck::types::ParallelismType::Single,
        }
    }

    fn test_params_multi(compression: CompressionType) -> EnkryptitParams {
        EnkryptitParams {
            compression,
            key_params: KeyParams::File,
            parallelism: eck::types::ParallelismType::MultiThread(8),
        }
    }

    fn read_archive_meta(archive_path: &str) -> (u8, Vec<u8>) {
        let file = std::fs::File::open(archive_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut len_buf = [0u8; 1];
        reader.read_exact(&mut len_buf).unwrap();
        let header_len = len_buf[0] as usize;
        let mut header_bytes = vec![0u8; header_len];
        reader.read_exact(&mut header_bytes).unwrap();
        let header: ArchiveHeader = from_bytes(&header_bytes).unwrap();
        let meta_len = header.meta_len as usize;

        let file_len = std::fs::metadata(archive_path).unwrap().len();
        let meta_start = file_len - meta_len as u64;
        reader.seek(SeekFrom::Start(meta_start)).unwrap();
        let mut meta = vec![0u8; meta_len];
        reader.read_exact(&mut meta).unwrap();
        (header.version, meta)
    }

    #[test]
    fn encrypt_decrypt_single_file_folder_nocomp_singlethread() {
        encrypt_decrypt_single_file_folder_nocomp(test_params_single(CompressionType::NoComp));
    }

    #[test]
    fn encrypt_decrypt_single_file_folder_nocomp_multithread() {
        encrypt_decrypt_single_file_folder_nocomp(test_params_multi(CompressionType::NoComp));
    }

    fn encrypt_decrypt_single_file_folder_nocomp(params: EnkryptitParams) {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("testfolder");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("hello.txt"), b"Hello from single file!").unwrap();

        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, params.compression, params.parallelism);
        let archive_path = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        )
        .unwrap();
        assert!(std::path::Path::new(&archive_path).exists());

        let (version, meta_bytes) = read_archive_meta(&archive_path);
        assert_eq!(version, 2);
        let folder_meta: FolderMetadata = from_bytes(&meta_bytes).unwrap();
        assert_eq!(folder_meta.entries.len(), 1);
        assert_eq!(folder_meta.entries[0].relative_path, "hello.txt");
        assert!(folder_meta.entries[0].offset > 0);

        fs::remove_dir_all(&folder).unwrap();
        let dest = decrypt_folder(&archive_path, &meta_bytes, 0, version, &mut context).unwrap();

        let decrypted = fs::read_to_string(std::path::Path::new(&dest).join("hello.txt")).unwrap();
        assert_eq!(decrypted, "Hello from single file!");
    }

    #[test]
    fn encrypt_decrypt_multiple_file_folder_nocomp_singlethread() {
        encrypt_decrypt_multi_file_folder_nocomp(test_params_single(CompressionType::NoComp));
    }

    #[test]
    fn encrypt_decrypt_multiple_file_folder_nocomp_multithread() {
        encrypt_decrypt_multi_file_folder_nocomp(test_params_multi(CompressionType::NoComp));
    }

    fn encrypt_decrypt_multi_file_folder_nocomp(params: EnkryptitParams) {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("multifolder");
        fs::create_dir_all(folder.join("subdir")).unwrap();
        fs::write(folder.join("file1.txt"), b"Content of file 1").unwrap();
        fs::write(folder.join("subdir/file2.txt"), b"Content of file 2").unwrap();
        fs::write(folder.join("subdir/file3.txt"), b"Third file content here").unwrap();
        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, params.compression, params.parallelism);

        let archive_path = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        )
        .unwrap();

        let (version, meta_bytes) = read_archive_meta(&archive_path);
        let folder_meta: FolderMetadata = from_bytes(&meta_bytes).unwrap();
        assert_eq!(folder_meta.entries.len(), 3);

        let mut offsets: Vec<u64> = folder_meta.entries.iter().map(|e| e.offset).collect();
        let sorted_offsets = offsets.clone();
        offsets.sort();
        assert_eq!(offsets, sorted_offsets);

        fs::remove_dir_all(&folder).unwrap();
        let dest = decrypt_folder(&archive_path, &meta_bytes, 0, version, &mut context).unwrap();

        let base = std::path::Path::new(&dest);
        assert_eq!(
            fs::read_to_string(base.join("file1.txt")).unwrap(),
            "Content of file 1"
        );
        assert_eq!(
            fs::read_to_string(base.join("subdir/file2.txt")).unwrap(),
            "Content of file 2"
        );
        assert_eq!(
            fs::read_to_string(base.join("subdir/file3.txt")).unwrap(),
            "Third file content here"
        );
    }

    #[test]
    fn encrypt_decrypt_zstd_roundtrip_singlethread() {
        encrypt_decrypt_zstd_roundtrip(test_params_single(CompressionType::Zstd));
    }

    #[test]
    fn encrypt_decrypt_zstd_roundtrip_multithread() {
        encrypt_decrypt_zstd_roundtrip(test_params_multi(CompressionType::Zstd));
    }

    fn encrypt_decrypt_zstd_roundtrip(params: EnkryptitParams) {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("zstdfolder");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("a.txt"), b"Zstd compressed content").unwrap();
        fs::write(folder.join("b.bin"), vec![0xAB; 1024 * 100]).unwrap();
        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, params.compression, params.parallelism);
        let archive_path = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        )
        .unwrap();

        let (version, meta_bytes) = read_archive_meta(&archive_path);

        fs::remove_dir_all(&folder).unwrap();
        let dest = decrypt_folder(&archive_path, &meta_bytes, 0, version, &mut context).unwrap();

        let base = std::path::Path::new(&dest);
        assert_eq!(
            fs::read_to_string(base.join("a.txt")).unwrap(),
            "Zstd compressed content"
        );
        assert_eq!(
            fs::read(base.join("b.bin")).unwrap(),
            vec![0xAB; 1024 * 100]
        );
    }

    #[test]
    fn encrypt_decrypt_lz4_roundtrip_singlethread() {
        encrypt_decrypt_lz4_roundtrip(test_params_single(CompressionType::Lz4));
    }

    #[test]
    fn encrypt_decrypt_lz4_roundtrip_multithread() {
        encrypt_decrypt_lz4_roundtrip(test_params_multi(CompressionType::Lz4));
    }

    fn encrypt_decrypt_lz4_roundtrip(params: EnkryptitParams) {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("lz4folder");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("data.bin"), vec![0x42; 50_000]).unwrap();
        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, params.compression, params.parallelism);
        let archive_path = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        )
        .unwrap();

        let (version, meta_bytes) = read_archive_meta(&archive_path);

        fs::remove_dir_all(&folder).unwrap();
        let dest = decrypt_folder(&archive_path, &meta_bytes, 0, version, &mut context).unwrap();

        assert_eq!(
            fs::read(std::path::Path::new(&dest).join("data.bin")).unwrap(),
            vec![0x42; 50_000]
        );
    }

    #[test]
    fn encrypt_decrypt_xz_roundtrip_singlethread() {
        encrypt_decrypt_xz_roundtrip(test_params_single(CompressionType::Xz));
    }

    #[test]
    fn encrypt_decrypt_xz_roundtrip_multithread() {
        encrypt_decrypt_xz_roundtrip(test_params_multi(CompressionType::Xz));
    }

    fn encrypt_decrypt_xz_roundtrip(params: EnkryptitParams) {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("xzfolder");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("readme.txt"), b"XZ compression test content").unwrap();
        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, params.compression, params.parallelism);

        let archive_path = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        )
        .unwrap();

        let (version, meta_bytes) = read_archive_meta(&archive_path);

        fs::remove_dir_all(&folder).unwrap();
        let dest = decrypt_folder(&archive_path, &meta_bytes, 0, version, &mut context).unwrap();

        assert_eq!(
            fs::read_to_string(std::path::Path::new(&dest).join("readme.txt")).unwrap(),
            "XZ compression test content"
        );
    }

    #[test]
    fn empty_folder_returns_error() {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("empty");
        fs::create_dir(&folder).unwrap();
        let mut context = EnkryptitContext::new(eck::types::Interface::Cli, None, CompressionType::NoComp, eck::types::ParallelismType::Auto);

        let result = encrypt_folder(
            folder.to_str().unwrap(),
            &mut context,
            &KeyType::FromFile,
        );
        assert!(result.is_err());
    }

    #[test]
    fn collect_entries_finds_nested_files() {
        let tmp = TempDir::new().unwrap();
        let folder = tmp.path().join("walk");
        fs::create_dir_all(folder.join("a/b/c")).unwrap();
        fs::write(folder.join("root.txt"), b"r").unwrap();
        fs::write(folder.join("a/mid.txt"), b"m").unwrap();
        fs::write(folder.join("a/b/c/deep.txt"), b"d").unwrap();
        let context = &EnkryptitContext { interface: eck::types::Interface::Cli, password: None, compression_type: CompressionType::Auto, parallelism: eck::types::ParallelismType::Auto };

        let entries = collect_folder_entries(folder.to_str().unwrap(), context).unwrap();
        assert_eq!(entries.len(), 3);

        let mut paths: Vec<&str> = entries.iter().map(|e| e.relative_path.as_str()).collect();
        paths.sort();
        assert_eq!(paths, vec!["a/b/c/deep.txt", "a/mid.txt", "root.txt"]);
    }
}
