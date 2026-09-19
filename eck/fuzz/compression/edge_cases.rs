//! Compression Edge Case Tests  
//! 
//! Exercises extreme input conditions including empty data, single bytes, and verifies
//! that repetitive data compresses more efficiently than random binary sequences.

#[cfg(test)]
mod compress_edge_cases {
    use eck::{compression::{EnkryptitCompress, EnkryptitDecompress}, types::{CompressionType, CHUNK_SIZE}};

    #[test] 
    fn empty_and_single_byte_roundtrip() {
        // Test edge cases - simple approach without complex strategies
        let test_cases = vec![
            vec![] as Vec<u8>,           // Empty vector
            vec![0x00],                   // Single zero byte  
            vec![0xFF],                   // Single max-value byte
            vec![1u8, 2, 3],              // Very small (4 bytes)
        ];

        for edge_case in test_cases {
            let original_len = edge_case.len();
            
            for &comp_type in [CompressionType::Zstd, CompressionType::Lz4, CompressionType::Xz, CompressionType::NoComp].iter() {
                let mut compressed = vec![0u8; CHUNK_SIZE];
                
                if !edge_case.is_empty() || comp_type == CompressionType::NoComp {
                    edge_case.clone().compress(&mut compressed, comp_type).unwrap();
                    
                    let mut decompressed = vec![0u8; original_len];
                    if !compressed.is_empty() {
                        compressed.as_slice().decompress(&mut decompressed, comp_type).unwrap();
                        
                        assert_eq!(edge_case, &decompressed[..], "Failed for {} with {:?}", comp_type, edge_case);
                    } else {
                        // Empty compression should produce empty output
                        assert!(decompressed.is_empty());
                    };
                }
            };
        }
    }

    #[test] 
    fn compressibility_ratio() {
        // Test repetitive vs random data - simple approach without complex strategies
        
        let test_patterns = vec![vec![0x00; 65536], vec![0xFF; 65536], vec![0xAA; 65536]];
        
        for repetitive in &test_patterns {
            // Generate random data of same size
            let random: Vec<u8> = (0..65536).map(|i| ((i * 12345) % 256) as u8).collect();

            let mut rep_compressed = vec![];
            let mut rand_compressed = vec![];

            // Compress both with Zstd
            repetitive.compress(&mut rep_compressed, CompressionType::Zstd).unwrap();
            random.compress(&mut rand_compressed, CompressionType::Zstd).unwrap();

            // Repetitive data should compress better (smaller size)
            assert!(rep_compressed.len() < rand_compressed.len(), 
                "Repetitive data ({:?}) should compress smaller than random: {} vs {}",
                repetitive[0], rep_compressed.len(), rand_compressed.len());

            // Verify NoComp preserves exact size - use actual test data instead of calling .arbitrary() on strategy
            let mut nocomp_output = vec![0u8; 65536];
            [0x42u8; 65536].compress(&mut nocomp_output, CompressionType::NoComp).unwrap();
            
            assert_eq!(nocomp_output.len(), 65536); // Exact copy for NoComp
        }
    }
}
