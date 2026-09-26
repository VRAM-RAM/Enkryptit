#![no_main]

use eck::metadatas::{FolderMetadata, MetaDatas};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = postcard::from_bytes::<MetaDatas>(data);
    let _ = postcard::from_bytes::<FolderMetadata>(data);
});