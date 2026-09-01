use infer::Type;
use crate::types::CompressionType;
use std::{fs::File};
use crate::errors::EnkryptitError;
use infer::get_from_path;

const LOW_BOUNDARY: u64 = 50 << 20;                // 50 MiB
const MID_INFERIOR_BOUNDARY: u64 = 250 << 20;      // 250 MiB
const MID_SUPERIOR_BOUNDARY: u64 = 1 << 30;        // 1 GiB
const SUPERIOR_BOUNDARY: u64 = 5 << 30;            // 5 GiB

pub fn infer_compression(path: &str) -> Result<CompressionType, EnkryptitError> {
    let file = File::open(path)?;
    let len = file.metadata()?.len();
    let hint = CompressionHint::compute(get_from_path(path)?);
    
    Ok(match hint {
        CompressionHint::AlreadyCompressed => CompressionType::NoComp,
        CompressionHint::Compressible => {
            match len {
                0..LOW_BOUNDARY => CompressionType::Lz4,
                LOW_BOUNDARY..MID_INFERIOR_BOUNDARY => CompressionType::Zstd,
                MID_INFERIOR_BOUNDARY..MID_SUPERIOR_BOUNDARY => CompressionType::Xz,
                MID_SUPERIOR_BOUNDARY..SUPERIOR_BOUNDARY => CompressionType::Zstd,
                _ => CompressionType::Lz4,
            }
        }
        CompressionHint::HighlyCompressible => {
            match len {
                0..LOW_BOUNDARY => CompressionType::Zstd,
                LOW_BOUNDARY..SUPERIOR_BOUNDARY => CompressionType::Xz,
                _ => CompressionType::Zstd,
            }   
        }

        CompressionHint::Unknown => {
            match len {
                0..LOW_BOUNDARY => CompressionType::NoComp,
                LOW_BOUNDARY..SUPERIOR_BOUNDARY => CompressionType::Lz4,
                _ => CompressionType::NoComp,
            }
        }
    })
}


enum CompressionHint {
    HighlyCompressible,
    Compressible,
    AlreadyCompressed,
    Unknown,
}

impl CompressionHint {
    pub fn compute(kind: Option<Type>) -> Self {
        match kind {
            Some(type_) => {
                let mime = type_.mime_type();

                // Plain text, markup & scripts are highly redundant.
                if mime.starts_with("text/") {
                    return CompressionHint::HighlyCompressible;
                }

                match mime {
                    // Archives & already-compressed containers.
                    "application/zip"
                    | "application/gzip"
                    | "application/zstd"
                    | "application/x-lz4"
                    | "application/x-xz"
                    | "application/x-lzip"
                    | "application/x-bzip2"
                    | "application/x-7z-compressed"
                    | "application/vnd.rar"
                    | "application/vnd.ms-cab-compressed"
                    | "application/x-tar"
                    | "application/x-unix-archive"
                    | "application/x-cpio"
                    | "application/x-rpm"
                    // Documents & e-books (already compressed inside).
                    | "application/pdf"
                    | "application/postscript"
                    | "application/epub+zip"
                    | "application/x-mobipocket-ebook"
                    | "application/msword"
                    | "application/vnd.oasis.opendocument.text"
                    | "application/vnd.oasis.opendocument.spreadsheet"
                    | "application/vnd.oasis.opendocument.presentation"
                    | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                    | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                    
                    // Executables & binaries (largely incompressible, random).
                    | "application/wasm"
                    | "application/x-llvm"
                    | "application/x-nintendo-nes-rom"
                    | "application/x-mach-binary"
                    | "application/x-executable"
                    | "application/vnd.microsoft.portable-executable"
                    | "application/vnd.android.dex"
                    | "application/vnd.debian.binary-package"
                    // Fonts (already optimized / woff is web-compressed).
                    | "application/font-woff"
                    | "application/font-sfnt"
                    // Compressed & lossy images.
                    | "image/jpeg"
                    | "image/png"
                    | "image/gif"
                    | "image/webp"
                    | "image/avif"
                    | "image/heif"
                    | "image/jp2"
                    | "image/jxl"
                    | "image/vnd.djvu"
                    | "image/vnd.ms-photo"
                    | "image/openraster"
                    | "image/vnd.adobe.photoshop"
                    // Lossy or lossless-compressed audio.
                    | "audio/aac"
                    | "audio/amr"
                    | "audio/m4a"
                    | "audio/mpeg"
                    | "audio/ogg"
                    | "audio/opus"
                    | "audio/x-flac"
                    | "audio/x-ape"
                    // Video codecs always compress their payload.
                    | "video/mp4"
                    | "video/webm"
                    | "video/x-m4v"
                    | "video/x-matroska"
                    | "video/quicktime"
                    | "video/x-msvideo"
                    | "video/x-ms-wmv"
                    | "video/x-flv"
                    | "video/mpeg"
                    => CompressionHint::AlreadyCompressed,

                    // Raw / uncompressed media that compress heavily, plus a few
                    // structured or mixed binary formats worth attempting.
                    "application/vnd.sqlite3"
                    | "application/x-qemu-disk"
                    | "application/x-ole-storage"
                    | "application/rtf"
                    | "application/dicom"
                    | "image/bmp"
                    | "image/tiff"
                    | "image/x-canon-cr2"
                    | "image/vnd.dwg"
                    | "audio/x-wav"
                    | "audio/x-aiff"
                    | "audio/x-dsf"
                    | "audio/midi"
                    => CompressionHint::Compressible,

                    // Anything else that `infer` recognized but we don't treat
                    // specially.
                    _ => CompressionHint::Unknown,
                }
            }
            None => CompressionHint::Unknown,
        }
    }
}