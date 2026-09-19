//! Regression tests for the `Display`/`String` conversions of the enum types.
//!
//! These used to recurse infinitely: `Display::fmt` called `self.to_string()`
//! which dispatched straight back to `Display::fmt` (via the blanket
//! `ToString` impl). A stack overflow is fatal to the process, so any of these
//! tests running to completion proves the recursion is gone.

use eck::types::{CompressionType, KeyType, ParallelismType};

#[test]
fn key_type_display_and_string_conversion_match() {
    assert_eq!(KeyType::Password.to_string(), "password");
    assert_eq!(String::from(KeyType::Password), "password");
    assert_eq!(KeyType::FromFile.to_string(), "from file");
    assert_eq!(KeyType::FromOS.to_string(), "from os keyring");
    assert_eq!(
        KeyType::None.to_string(),
        "no keytype used (should not happen if the file is encrypted)"
    );
}

#[test]
fn key_type_pwd256_displays_salt() {
    let salt = [0xDEu8; 16];
    let rendered = KeyType::Pwd256(salt).to_string();
    assert!(rendered.starts_with("hashed password, with the following salt : "));
    assert!(
        rendered.ends_with("dededededededededededededededede"),
        "got: {rendered}"
    );
}

#[test]
fn compression_type_display_and_string_conversion_match() {
    assert_eq!(CompressionType::Auto.to_string(), "Automatic");
    assert_eq!(CompressionType::Lz4.to_string(), "Lz4 (fastest)");
    assert_eq!(CompressionType::Xz.to_string(), "Xz (slowest but most efficient)");
    assert_eq!(CompressionType::NoComp.to_string(), "No compression");
    assert_eq!(CompressionType::Zstd.to_string(), "Zstd (Balanced)");
    assert_eq!(String::from(CompressionType::Zstd), "Zstd (Balanced)");
}

#[test]
fn parallelism_type_display_and_string_conversion_match() {
    assert_eq!(ParallelismType::Auto.to_string(), "Automatic");
    assert_eq!(ParallelismType::Single.to_string(), "SingleThread");
    assert_eq!(
        ParallelismType::MultiThread(8).to_string(),
        "MultiThreading with 8 threads"
    );
    let into: String = ParallelismType::MultiThread(4).into();
    assert_eq!(into, "MultiThreading with 4 threads");
}

#[test]
fn description_methods_agree_with_display() {
    for k in [
        KeyType::Password,
        KeyType::FromFile,
        KeyType::FromOS,
        KeyType::None,
    ] {
        assert_eq!(k.to_string(), k.description());
    }
    for c in [
        CompressionType::Auto,
        CompressionType::Lz4,
        CompressionType::Xz,
        CompressionType::NoComp,
        CompressionType::Zstd,
    ] {
        assert_eq!(c.to_string(), c.description());
    }
    for p in [
        ParallelismType::Auto,
        ParallelismType::Single,
        ParallelismType::MultiThread(16),
    ] {
        assert_eq!(p.to_string(), p.description());
    }
}