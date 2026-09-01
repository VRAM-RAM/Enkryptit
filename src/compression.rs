use crate::errors::EnkryptitError;
use crate::types::CHUNK_SIZE;
use crate::types::CompressionType::{self, Lz4, NoComp, Xz, Zstd, Auto};
use lz4_flex::block::{compress_into as lz4compress, decompress_into as lz4decompress};
use std::io::Read;
use xz2::read::{XzDecoder, XzEncoder};
use zstd::bulk::compress_to_buffer as zstdcompress;
use zstd::bulk::decompress_to_buffer as zstddecompress;

/// Compression trait for `Enkryptit`. Implemented for `&'a [u8]` and `Vec<u8>`.
/// \
/// Implements the `compress` method :
///
/// ```text
/// fn compress(&self, output: &mut Vec<u8>, compression: CompressionType) -> Result<(), EnkryptitError>;
/// ```
pub trait EnkryptitCompress {
    fn compress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError>;
}

/// Decompression trait for `Enkryptit`. Implemented for `&'a [u8]` and `Vec<u8>`.
/// \
/// Implements the `decompress` method :
///
/// ```text
/// fn decompress(&self, output: &mut Vec<u8>, compression: CompressionType) -> Result<(), EnkryptitError>;
/// ```
pub trait EnkryptitDecompress {
    fn decompress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError>;
}

impl EnkryptitCompress for Vec<u8> {
    fn compress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError> {
        self.as_slice().compress(output, compression)
    }
}

impl<'a> EnkryptitCompress for &'a [u8] {
    fn compress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError> {
        match compression {
            Auto => unreachable!("`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue."),
            Zstd => compress_with_zstd(self, output),
            Lz4 => compress_with_lz4(self, output),
            Xz => compress_with_xz(self, output),
            NoComp => {
                output.clear();
                output.extend_from_slice(self);
                Ok(())
            }
        }
    }
}

impl EnkryptitDecompress for Vec<u8> {
    fn decompress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError> {
        self.as_slice().decompress(output, compression)
    }
}

impl<'a> EnkryptitDecompress for &'a [u8] {
    fn decompress(
        &self,
        output: &mut Vec<u8>,
        compression: CompressionType,
    ) -> Result<(), EnkryptitError> {
        match compression {
            Auto =>  unreachable!("`Auto` should never be reached here, and always infered before reaching this function. There is an error in the code. If you are reading this as an user, please open an Issue."),
            Zstd => decompress_with_zstd(self, output),
            Lz4 => decompress_with_lz4(self, output),
            Xz => decompress_with_xz(self, output),
            NoComp => {
                output.clear();
                output.extend_from_slice(self);
                Ok(())
            }
        }
    }
}

// Private helpers for compression

/// Private helper function for compressing an input block `&[u8]` in-place with **lz4** algorithm, in a given output `&mut Vec<u8>`.
/// \
/// It automatically ensures that the output has enough capacity, resize it, and then compress the input block
/// in the pre-allocated buffer, before truncating the output to the actual compressed size.
fn compress_with_lz4(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    // Get maximum possible compressed size
    let max_compressed_size = lz4_flex::block::get_maximum_output_size(input.len());

    // Ensure output has enough capacity
    output.clear();
    output.reserve(max_compressed_size);

    // Resize to max size so compress_into can write into it
    output.resize(max_compressed_size, 0);

    // Compress into the pre-allocated buffer
    let actual_size = lz4compress(input, output)?;

    // Truncate to actual compressed size
    output.truncate(actual_size);

    Ok(())
}

/// Private helper function for decompressing an input block `&[u8]` in-place with **lz4** algorithm, in a given output `&mut Vec<u8>`.
/// \
/// It automatically tries decompressing with exponentially growing buffers until success (or max retries).
fn decompress_with_lz4(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    // Clear both length and capacity so vector can grow freely
    output.clear();

    // Try decompression with exponentially growing buffers until success or max retries
    let mut current_size = std::cmp::max(input.len() * 4, CHUNK_SIZE / 16);
    for _ in 0..5 {
        output.resize(current_size, 0);

        match lz4decompress(input, output) {
            Ok(actual_size) => {
                output.truncate(actual_size);
                return Ok(());
            }
            Err(_) => {
                // Buffer too small, we try with larger buffer (double size each iteration)
                current_size *= 2;
                if current_size > CHUNK_SIZE * 4 {
                    // Give up after reasonable attempts
                    output.resize(current_size, 0);
                    let actual_size = lz4decompress(input, output)?;
                    output.truncate(actual_size);
                    return Ok(());
                }
            }
        }
    }

    // Final attempt with very large buffer if all else fails
    output.resize(CHUNK_SIZE * 8, 0);
    let actual_size = lz4decompress(input, output)?;
    output.truncate(actual_size);
    Ok(())
}

/// Private helper function for compressing an input block `&[u8]` in-place with **zstd** algorithm, in a given output `&mut Vec<u8>`.
/// \
/// It automatically tries compressing with exponentially growing buffers until success (or max retries).
fn compress_with_zstd(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    output.clear();

    // Try compression with exponentially growing buffers until success or max retries
    let mut current_size = std::cmp::max(input.len() + 20, CHUNK_SIZE / 16);
    for _ in 0..5 {
        output.resize(current_size, 0);

        match zstdcompress(input, output, 6) {
            Ok(size) => {
                output.truncate(size as usize);
                return Ok(());
            }
            Err(_) => {
                // Buffer too small, we try with larger buffer (double size each iteration)
                current_size *= 2;
                if current_size > CHUNK_SIZE * 4 {
                    // Give up after reasonable attempts
                    output.resize(current_size, 0);
                    let size = zstdcompress(input, output, 6)?;
                    output.truncate(size as usize);
                    return Ok(());
                }
            }
        }
    }

    // Final attempt with very large buffer if all else fails
    output.resize(CHUNK_SIZE * 8, 0);
    let size = zstdcompress(input, output, 6)?;
    output.truncate(size as usize);
    Ok(())
}

/// Private helper function for decompressing an input block `&[u8]` in-place with **zstd** algorithm, in a given output `&mut Vec<u8>`.
/// \
/// It automatically tries decompressing with exponentially growing buffers until success (or max retries).
fn decompress_with_zstd(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    output.clear();

    // Try decompression with exponentially growing buffers until success or max retries
    let mut current_size = std::cmp::max(input.len() * 4, CHUNK_SIZE / 16);
    for _ in 0..5 {
        output.resize(current_size, 0);

        match zstddecompress(input, output) {
            Ok(size) => {
                output.truncate(size as usize);
                return Ok(());
            }
            Err(_) => {
                // Buffer too small - try with larger buffer (double size each iteration)
                current_size *= 2;
                if current_size > CHUNK_SIZE * 4 {
                    // Give up after reasonable attempts
                    output.resize(current_size, 0);
                    let size = zstddecompress(input, output)?;
                    output.truncate(size as usize);
                    return Ok(());
                }
            }
        }
    }

    // Final attempt with very large buffer if all else fails
    output.resize(CHUNK_SIZE * 8, 0);
    let size = zstddecompress(input, output)?;
    output.truncate(size as usize);
    Ok(())
}

/// Private helper function for compressing using **xz** algorithm.
/// \
/// It first clears the `output` vector, then creates an `XzEncoder<>` given the `input` and the *compression level*, before
/// compressing and placing the resulting bytes into the `output` vector.
fn compress_with_xz(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    // Clear both length and capacity so vector can grow freely
    output.clear();

    let mut encoder = XzEncoder::new(input, 6);
    encoder.read_to_end(output)?;
    Ok(())
}

/// Private helper function for decompressing using **xz** algorithm.
/// \
/// It first clears the `output` vector, then creates an `XzDecoder<>` given the `input` and the *compression level*, before
/// decompressing and placing the resulting bytes into the `output` vector.
fn decompress_with_xz(input: &[u8], output: &mut Vec<u8>) -> Result<(), EnkryptitError> {
    output.clear();

    let mut decoder = XzDecoder::new(input);
    decoder.read_to_end(output)?;
    Ok(())
}
