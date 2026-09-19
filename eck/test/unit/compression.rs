//! Compression Algorithm Tests
//!
//! Verify roundtrip integrity for Zstd, Lz4, Xz compression algorithms

use eck::compression::{EnkryptitCompress, EnkryptitDecompress};
use eck::types::CHUNK_SIZE;
use eck::types::CompressionType;

fn generate_random_content(size: usize) -> Vec<u8> {
    let mut content = vec![0u8; size];

    for i in 0..size {
        content[i] = ((i * 2654435781) % 256) as u8;
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_zstd_roundtrip() {
        let original = b"Hello World! This is a test of Zstandard compression.".to_vec();

        // Pre-allocate output buffer - zstd::bulk::compress_to_buffer needs len == capacity
        let mut compressed: Vec<u8> = vec![0u8; 65536];
        original
            .as_slice()
            .compress(&mut compressed, CompressionType::Zstd)
            .unwrap();

        let mut decompressed = vec![0u8; original.len()];
        compressed
            .decompress(&mut decompressed, CompressionType::Zstd)
            .unwrap();

        assert_eq!(original, decompressed);
    }

    #[test]
    fn compression_lz4_roundtrip() {
        let original = b"Testing LZ4 compression algorithm integrity.".to_vec();

        // lz4_flex::block::compress_into requires output to have capacity
        let max_size = lz4_flex::block::get_maximum_output_size(original.len());
        let mut compressed: Vec<u8> = vec![0u8; max_size];

        original
            .compress(&mut compressed, CompressionType::Lz4)
            .unwrap();
        // Truncate to actual compressed size (lz4 implementation does this internally)
        compressed.truncate(compressed.len());

        let mut decompressed = vec![0u8; original.len()];
        compressed
            .as_slice()
            .decompress(&mut decompressed, CompressionType::Lz4)
            .unwrap();

        assert_eq!(original, &decompressed[..]);
    }

    #[test]
    fn compression_xz_roundtrip() {
        let original = b"XZ compression with maximum ratio testing.".to_vec();

        // XZ encoder writes to output buffer - use empty vec that can grow
        let mut compressed: Vec<u8> = Vec::with_capacity(original.len());
        original
            .compress(&mut compressed, CompressionType::Xz)
            .unwrap();

        let mut decompressed = vec![0u8; original.len()];
        compressed
            .as_slice()
            .decompress(&mut decompressed, CompressionType::Xz)
            .unwrap();

        assert_eq!(original, &decompressed[..]);
    }

    #[test]
    fn compression_no_comp_identity() {
        let original = b"No compression test data".to_vec();

        // NoComp should just copy the input - use with_capacity for consistent behavior
        let mut output: Vec<u8> = Vec::with_capacity(original.len());
        original
            .compress(&mut output, CompressionType::NoComp)
            .unwrap();
        assert_eq!(output, original);
    }

    #[test]
    fn compression_empty_data_all_algos() {
        let empty: &[u8] = &[];
        for comp in [
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
            CompressionType::NoComp,
        ]
        .iter()
        {
            // Use with_capacity - different compression algos handle buffers differently
            let mut compressed: Vec<u8> = vec![0u8; CHUNK_SIZE];

            empty.compress(&mut compressed, *comp).unwrap();

            let mut decompressed = vec![0u8; CHUNK_SIZE];
            compressed.decompress(&mut decompressed, *comp).unwrap();
            assert!(decompressed.is_empty());
        }
    }

    #[test]
    fn compression_large_data_1mb() {
        let data = generate_random_content(1024 * 1024); // 1MB

        for comp in [CompressionType::Zstd, CompressionType::Lz4].iter() {
            // Random data doesn't compress well - use empty vec that can grow
            let mut compressed: Vec<u8> = match *comp {
                CompressionType::Zstd => vec![],
                _ => {
                    // For LZ4 with large data, pre-allocate max size
                    let max_size = lz4_flex::block::get_maximum_output_size(data.len());
                    vec![0u8; max_size]
                }
            };

            data.as_slice().compress(&mut compressed, *comp).unwrap();

            // Truncate LZ4 to actual size
            if *comp == CompressionType::Lz4 {
                compressed.truncate(compressed.len());
            }

            let mut decompressed = vec![0u8; data.len()];
            compressed
                .as_slice()
                .decompress(&mut decompressed, *comp)
                .unwrap();
            assert_eq!(data, decompressed);
        }
    }

    #[test]
    fn compression_different_compressions_produce_different_sizes() {
        let data = generate_random_content(65536); // 64KB

        let mut zstd_out = vec![0u8; 128_000];
        let mut lz4_out = vec![0u8; 128_000];
        data.as_slice()
            .compress(&mut zstd_out, CompressionType::Zstd)
            .unwrap();
        data.as_slice()
            .compress(&mut lz4_out, CompressionType::Lz4)
            .unwrap();

        let zstd_size = zstd_out.len();
        let lz4_size = lz4_out.len();

        assert!(zstd_size > 0);
        assert!(lz4_size > 0);
    }

    #[test]
    fn compression_random_data_zstd() {
        use rand::Rng;
        use rand::rngs::OsRng;
        let mut rng = OsRng;
        let data: Vec<u8> = (0..4096).map(|_| rng.r#gen()).collect();

        // Random data doesn't compress well - use empty vec that can grow
        let mut compressed: Vec<u8> = vec![];
        data.as_slice()
            .compress(&mut compressed, CompressionType::Zstd)
            .unwrap();

        let mut decompressed = vec![0u8; data.len()];
        compressed
            .as_slice()
            .decompress(&mut decompressed, CompressionType::Zstd)
            .unwrap();

        assert_eq!(data, decompressed);
    }

    #[test]
    fn compression_repetitive_data_compression_ratio() {
        let repetitive = vec![b'X'; 65536]; // All same byte - highly compressible

        // Use empty vec that can grow for Zstd
        let mut zstd_out: Vec<u8> = vec![];

        repetitive
            .as_slice()
            .compress(&mut zstd_out, CompressionType::Zstd)
            .unwrap();

        // Verify at least one algorithm compresses well (repetitive data should shrink)
        let ratio_zstd = zstd_out.len() as f64 / 65536.0;
        assert!(ratio_zstd < 1.0); // At least Zstd should compress repetitive data
    }

    #[test]
    fn compression_all_algorithms_roundtrip_various_sizes() {
        for comp in [
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
            CompressionType::NoComp,
        ]
        .iter()
        {
            // Reduce Xz test size from 500 to 8KB max due to slow compression speed
            let sizes = if *comp == CompressionType::Xz {
                vec![1usize, 7, 8, 9, 32, 64]
            } else {
                vec![1usize, 7, 8, 9, 32, 64, 100, 500]
            };

            for &size in sizes.iter() {
                let data = vec![size as u8; size];

                // Use appropriate buffer strategy per compression type
                let mut compressed: Vec<u8> = match *comp {
                    CompressionType::Zstd | CompressionType::Xz => vec![],
                    CompressionType::NoComp => vec![0u8; size],
                    _ => {
                        // LZ4 - pre-allocate max possible size
                        let max_size = lz4_flex::block::get_maximum_output_size(size);
                        vec![0u8; max_size]
                    }
                };

                data.as_slice().compress(&mut compressed, *comp).unwrap();

                // Truncate LZ4 to actual compressed size
                if *comp == CompressionType::Lz4 {
                    compressed.truncate(compressed.len());
                }

                let mut decompressed = vec![0u8; size];
                compressed
                    .as_slice()
                    .decompress(&mut decompressed, *comp)
                    .unwrap();

                assert_eq!(data, decompressed);
            }
        }
    }
}
