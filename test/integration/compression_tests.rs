//! Context / compression-inference tests
//!
//! Exercises `EnkryptitContext::resolve_compression` with `CompressionType::Auto`,
//! which drives the mime + size based inference in `context::compression`.
//!
//! Files are detected by their magic bytes (via the `infer` crate), and the
//! chosen `CompressionType` also depends on the file size, so we use *sparse*
//! files (`set_len`) to cheaply simulate multi-GiB files without writing them
//! to disk.

use eck::context::EnkryptitContext;
use eck::types::{CompressionType, Interface};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0DIHDR";
    const ZIP: &[u8] = b"PK\x03\x04";
    /// RIFF....WAVE (12 bytes) — a raw/uncompressed PCM container.
    const WAV: &[u8] = b"RIFF\x00\x00\x00\x00WAVE";
    const XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><root/>";
    /// Bytes that match no `infer` signature -> unknown type.
    const UNKNOWN: &[u8] = b"\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9";

    /// Resolve compression for a real file through the public context API.
    fn auto(dir: &TempDir, name: &str, bytes: &[u8]) -> CompressionType {
        let path = dir.path().join(name);
        std::fs::write(&path, bytes).unwrap();
        let ctx = EnkryptitContext::new(Interface::Cli, None, CompressionType::Auto);
        ctx.resolve_compression(path.to_str().unwrap())
            .expect("auto inference must succeed")
    }

    /// Build a *sparse* file: `head` is the real (detectable) leading bytes,
    /// then the file is logically extended to `len` bytes. Cheap even for huge
    /// sizes, because the trailing region is never written to disk.
    fn auto_sparse(dir: &TempDir, name: &str, head: &[u8], len: u64) -> CompressionType {
        let path = dir.path().join(name);
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(head).unwrap();
        f.set_len(len).unwrap();
        let ctx = EnkryptitContext::new(Interface::Cli, None, CompressionType::Auto);
        ctx.resolve_compression(path.to_str().unwrap())
            .expect("auto inference must succeed")
    }

    /// Mime classification (small files, below LOW_BOUNDARY)

    #[test]
    fn png_is_already_compressed() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto(&dir, "img.png", PNG), CompressionType::NoComp);
    }

    #[test]
    fn zip_archive_is_already_compressed() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto(&dir, "bundle.zip", ZIP), CompressionType::NoComp);
    }

    #[test]
    fn raw_wav_is_compressible_small() {
        let dir = TempDir::new().unwrap();
        // Compressible + small (< 50 MiB) -> fast path.
        assert_eq!(auto(&dir, "sound.wav", WAV), CompressionType::Lz4);
    }

    #[test]
    fn text_xml_is_highly_compressible_small() {
        let dir = TempDir::new().unwrap();
        // HighlyCompressible + small (< 50 MiB) -> Zstd.
        assert_eq!(auto(&dir, "doc.xml", XML), CompressionType::Zstd);
    }

    #[test]
    fn unknown_file_small_is_nocomp() {
        let dir = TempDir::new().unwrap();
        // Unknown + small (< 50 MiB) -> NoComp.
        assert_eq!(auto(&dir, "blob.bin", UNKNOWN), CompressionType::NoComp);
    }

    // --- Size-based ramp for the *Compressible* hint (raw WAV) ---
    const LOW: u64 = 50 << 20;          // 50 MiB
    const MID_INF: u64 = 250 << 20;     // 250 MiB
    const MID_SUP: u64 = 1 << 30;       // 1 GiB
    const SUP: u64 = 5 << 30;           // 5 GiB

    #[test]
    fn compressible_just_below_low_boundary_uses_lz4() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "w1.wav", WAV, LOW - 1), CompressionType::Lz4);
    }

    #[test]
    fn compressible_above_low_boundary_uses_zstd() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "w2.wav", WAV, LOW + 1), CompressionType::Zstd);
    }

    #[test]
    fn compressible_above_mid_inferior_uses_xz() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "w3.wav", WAV, MID_INF + 1), CompressionType::Xz);
    }

    #[test]
    fn compressible_above_mid_superior_uses_zstd() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "w4.wav", WAV, MID_SUP + 1), CompressionType::Zstd);
    }

    #[test]
    fn compressible_above_superior_uses_lz4() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "w5.wav", WAV, SUP + 1), CompressionType::Lz4);
    }

    /// HighlyCompressible size ramp (< 50 MiB Zstd, then Xz, then Zstd)

    #[test]
    fn highly_compressible_mid_range_uses_xz() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "d.xml", XML, LOW + 1), CompressionType::Xz);
    }

    #[test]
    fn highly_compressible_very_large_uses_zstd() {
        let dir = TempDir::new().unwrap();
        assert_eq!(auto_sparse(&dir, "d2.xml", XML, SUP + 1), CompressionType::Zstd);
    }

    /// Non-Auto resolution bypasses inference entirely

    #[test]
    fn explicit_compression_short_circuits_inference() {
        let ctx = EnkryptitContext::new(Interface::Cli, None, CompressionType::Xz);
        // Path never touched: non-Auto must not hit the filesystem.
        let resolved = ctx.resolve_compression("/nonexistent/does/not/exist").unwrap();
        assert_eq!(resolved, CompressionType::Xz);
    }
}
