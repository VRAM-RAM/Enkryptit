#![no_main]

use eck::compression::EnkryptitDecompress;
use eck::types::CompressionType::{Lz4, NoComp, Xz, Zstd};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some(&selector) = data.first() else {
        return;
    };

    let compression = match selector {
        0..=63 => NoComp,
        64..=127 => Lz4,
        128..=191 => Zstd,
        _ => Xz,
    };

    let mut output = Vec::new();
    let _ = data[1..].decompress(&mut output, compression);
});