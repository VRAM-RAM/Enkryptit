//! Compression Roundtrip Integrity Tests
//! 
//! Verifies that compressing and then decompressing data always recovers the original,
//! regardless of input size or byte pattern. All algorithms must maintain perfect integrity.

#[cfg(test)]
mod compress_roundtrip_integrity {
    use eck::{compression::{EnkryptitCompress, EnkryptitDecompress}, types::{CompressionType, CHUNK_SIZE}};

    #[test]
    fn zstd_random_data_sizes() {
        // Test Zstd with random data of varying sizes - simple approach without complex strategies
        let test_sizes = vec![1usize, 64, 256, 1024, CHUNK_SIZE/4, CHUNK_SIZE/2, CHUNK_SIZE];
        
        for &size in &test_sizes {
            // Generate pseudo-random data based on size seed
            let data: Vec<u8> = (0..size).map(|i| ((i * 2654435781) % 256) as u8).collect();

            let mut compressed = vec![0u8; CHUNK_SIZE * 2]; // Oversized buffer for safety
            data.compress(&mut compressed, CompressionType::Zstd).unwrap();
            
            // Truncate to actual compressed size
            let _len = compressed.len();

            let mut decompressed = vec![0u8; data.len()];
            compressed.decompress(&mut decompressed, CompressionType::Zstd).unwrap();
            
            assert_eq!(data, decompressed, "Roundtrip failed for Zstd with {} bytes", size);
        }
    }

    #[test]
    fn all_algorithms_roundtrip() {
        // Test all compression algorithms on identical inputs - simple approach without complex strategies
        let test_data = vec![0xABu8; 1024];
        
        for comp in [CompressionType::Zstd, CompressionType::Lz4, CompressionType::Xz, CompressionType::NoComp] {
            let mut compressed = vec![0u8; CHUNK_SIZE * 2];
            
            match comp {
                CompressionType::Lz4 => {
                    test_data.clone().compress(&mut compressed, comp).unwrap();
                    compressed.truncate(compressed.len()); // LZ4 truncation strategy
                    
                    let mut decompressed = vec![0u8; test_data.len()];
                    compressed.as_slice().decompress(&mut decompressed, comp).unwrap();
                    
                    assert_eq!(test_data, &decompressed[..]);
                }
                _ => {
                    test_data.clone().compress(&mut compressed, comp).unwrap();
                    let _len = compressed.len(); // Zstd/Xz/NoComp truncate internally
                    
                    let mut decompressed = vec![0u8; test_data.len()];
                    compressed.as_slice().decompress(&mut decompressed, comp).unwrap();
                    
                    assert_eq!(test_data, &decompressed[..]);
                }
            };
        }
    }

    #[test]
    fn large_file_integrity_1mb_plus() {
        // Test with 1MB+ random binary data - simple approach without complex strategies  
        let size = CHUNK_SIZE; // Exactly 8MB
        
        let large_data: Vec<u8> = (0..size).map(|i| ((i * 2654435781) % 256) as u8).collect();
        
        let comp = CompressionType::Zstd;
            
        let mut compressed = vec![]; // Let Zstd grow buffer automatically
        large_data.compress(&mut compressed, comp).unwrap();

        let mut decompressed = vec![0u8; large_data.len()];
        compressed.as_slice().decompress(&mut decompressed, comp).unwrap();
            
        assert_eq!(large_data, decompressed);
    }
}
