//! Compression Algorithm-Specific Tests
//! 
/// Validates implementation details specific to each compression algorithm, including LZ4's
/// maximum output size requirements, Zstd's buffer handling strategies, and Xz's slow but
/// correct compression behavior (with reduced iteration count due to performance characteristics).

#[cfg(test)]
mod compress_algorithm_specifics {
    use eck::{compression::{EnkryptitCompress, EnkryptitDecompress}, types::{CompressionType, CHUNK_SIZE}};
    
    #[test]
    fn lz4_max_allocation_safety() {
        // Test various buffer sizes - simple approach without complex strategies
        let test_sizes = vec![1usize, CHUNK_SIZE-1, CHUNK_SIZE, CHUNK_SIZE+1024];
        
        for &size in &test_sizes {
            
            let data = vec![0xABu8; size];
            
            // Pre-allocate using LZ4's documented maximum output formula
            let max_output_size = lz4_flex::block::get_maximum_output_size(size);
            let mut compressed = vec![0u8; max_output_size];

            // This should never panic or overflow the pre-allocated buffer
            data.compress(&mut compressed, CompressionType::Lz4).unwrap();
            
            // Truncate to actual size (LZ4 implementation does this internally)
            compressed.truncate(compressed.len());

            // Verify decompression recovers original exactly
            let mut decompressed = vec![0u8; data.len()];
            compressed.as_slice().decompress(&mut decompressed, CompressionType::Lz4).unwrap();
            
            assert_eq!(data, &decompressed[..], "LZ4 roundtrip failed for {} bytes", size);
        }
    }

    #[test]
    fn xz_correctness_reduced_iterations() {
        // Test Xz with small inputs due to slowness - use simple loop instead of proptest strategy
        let test_sizes = vec![1usize, 8, 32, 64];
        
        for &size in &test_sizes {
            let data: Vec<u8> = (0..size).map(|i| i as u8).collect();
            
            let mut compressed = vec![];
            
            // Xz is very slow - limit to small inputs for reasonable test time
            data.compress(&mut compressed, CompressionType::Xz).unwrap();

            let mut decompressed = vec![0u8; data.len()];
            compressed.as_slice().decompress(&mut decompressed, CompressionType::Xz).unwrap();
            
            assert_eq!(data, &decompressed[..], "Xz roundtrip failed");
        }
    }

    #[test]
    fn zstd_buffer_growing() {
        // Test Zstd with growing buffer - use simple loop instead of proptest strategy  
        let test_sizes = vec![CHUNK_SIZE / 2, CHUNK_SIZE * 3/4, CHUNK_SIZE];
        
        for &size in &test_sizes {
            let large_data: Vec<u8> = (0..size).map(|i| i as u8).collect();
            
            let mut compressed = vec![]; // Empty, will grow as needed
            
            large_data.compress(&mut compressed, CompressionType::Zstd).unwrap();

            assert!(compressed.len() > 0); // Should have produced some output
            
            let mut decompressed = vec![0u8; large_data.len()];
            compressed.as_slice().decompress(&mut decompressed, CompressionType::Zstd).unwrap();
            
            assert_eq!(large_data, decompressed);
        }
    }
}
