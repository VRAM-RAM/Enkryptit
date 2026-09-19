//! Compression Algorithm Fuzz Testing
//! 
//! Property-based tests for all compression algorithms (Zstd, Lz4, Xz, NoComp)
//! verifying roundtrip integrity, edge case handling, and algorithm-specific behaviors.

mod roundtrip_integrity;  // Group A: Roundtrip integrity tests

mod edge_cases;           // Group B: Edge case boundary tests  

mod algorithm_specifics;  // Group C: Algorithm-specific behavior tests
