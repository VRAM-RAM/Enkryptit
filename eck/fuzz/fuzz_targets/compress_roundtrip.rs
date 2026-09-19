#![no_main]

use eck::compression::{EnkryptitCompress, EnkryptitDecompress};
use eck::types::CompressionType::{Lz4, NoComp, Xz, Zstd};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }

    let compression = match data[0] {
        0..=63 => NoComp,
        64..=127 => Lz4,
        128..=191 => Zstd,
        _ => Xz,
    };

    let mut compressed = Vec::new();
    data[1..]
        .compress(&mut compressed, compression)
        .expect("compressing arbitrary data must never fail");

    let mut decompressed = Vec::new();
    compressed
        .decompress(&mut decompressed, compression)
        .expect("decompressing our own output must never fail");

    assert_eq!(
        decompressed,
        data[1..],
        "compress/decompress roundtrip must preserve the exact input bytes"
    );
});