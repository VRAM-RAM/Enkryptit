mod report;
mod inspect_encrypted_archive;
mod inspect_encrypted_file;
mod inspect_plain_file;
mod inspect_plain_folder;
use std::path::Path;
use crate::diagnostic::EnkryptitOutput;
use crate::treatment::object_treatment::read_file;
use crate::treatment::inspect::inspect_encrypted_archive::inspect_encrypted_archive;
use crate::treatment::inspect::inspect_encrypted_file::inspect_encrypted_file;
use crate::treatment::inspect::inspect_plain_file::inspect_plain_file;
use crate::treatment::inspect::inspect_plain_folder::inspect_plain_folder;
use crate::treatment::object_treatment::ParsedFile;
pub use report::InspectionReport;

pub fn inspect_object(path: &str) -> EnkryptitOutput {
    if Path::new(path).is_dir() {
        match inspect_plain_folder(path).display() {
            Ok(()) => (),
            Err(e) => return e.into()
        }
    }

    match read_file(path) {
        Ok(ParsedFile::Enkryptit { meta, version, is_folder_archive, .. }) => {
            if is_folder_archive {
                match inspect_encrypted_archive(path, &meta, version).display() {
                    Ok(()) => (),
                    Err(e) => return e.into()
                }
            } else {
                match inspect_encrypted_file(path, &meta, version).display() {
                    Ok(()) => (),
                    Err(e) => return e.into()
                }
            }
        }

        Ok(ParsedFile::Plain) => {
            match inspect_plain_file(path).display() {
                Ok(()) => (),
                Err(e) => return e.into()
            }
        }

        Err(e) => return e
    }

    EnkryptitOutput::phantom()
}

