use std::{fs::{metadata},  path::Path};
use infer::get_from_path;

use crate::{context::{compression::infer_compression, parallelism::infer_parallelism}, treatment::inspect::InspectionReport};

pub fn inspect_plain_file(path: &str) -> InspectionReport {
    let pathstd = Path::new(path);

    let mut size = None;
    let mut permissions = None;
    let mut extension = None;
    let mut mime_extension = None;
    let mut parallelism = None;

    match get_from_path(path) {
        Ok(type_) => {
            match type_ {
                Some(t) => {
                    extension = Some(t.extension().to_string());
                    mime_extension = Some(t.mime_type().to_string());
                }
                None => {
                    match pathstd.extension() {
                        Some(ext) => extension = Some(ext.to_string_lossy().to_string()),
                        None => ()
                    }
                }
            }
        }

        Err(e) => tracing::warn!("{}", e)
    }

    match metadata(path) {
        Ok(m) => {
            size = Some(m.len());
            permissions = {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    Some(m.permissions().mode())
                }

                #[cfg(not(unix))]
                {
                    None // No permissions on Windows
                } 
            };
            
            match infer_parallelism(m.len() as u64) {
                Ok(p) => parallelism = Some(p),
                Err(e) => tracing::warn!("{}", e),
            }
        }
        Err(e) => tracing::warn!("{}", e)
    }

    let compression = match infer_compression(path) {
        Ok(c) => Some(c),
        Err(e) => {
            tracing::warn!("{}", e);
            None
        }
    };

    let name = match pathstd.file_name() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    let directory = match pathstd.parent() {
        Some(p) => Some(p.to_string_lossy().to_string()),
        None => None
    };

    
    InspectionReport::PlaintextFile { name, directory, size, permissions, extension, mime_extension, predicted_compression_type: compression, predicted_parallelism_type: parallelism }
}