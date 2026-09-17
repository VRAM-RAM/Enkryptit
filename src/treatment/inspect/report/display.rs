use fmodeparser::FullPermission;
use hex::ToHex;
use crate::{diagnostic::{EnkryptitOutput, report::{Report, argument::ReportArgument, style::EnkryptitStyle}}, errors::EnkryptitError, treatment::inspect::InspectionReport, types::{CompressionType, KeyType, ParallelismType}};

const NOT_FOUND: &str = "Not Found";

impl InspectionReport {
    pub fn display(self) -> Result<(), EnkryptitError> {
        match self {
            Self::PlaintextFile { name, directory, size, permissions, extension, mime_extension, predicted_compression_type, predicted_parallelism_type } => display_plaintext_file(name, directory, size, permissions, extension, mime_extension, predicted_compression_type, predicted_parallelism_type),
            Self::EncryptedFile { name, directory, size, version, compression_type, predicted_parallelism_type, keytype, nonce } => display_encrypted_file(name, directory, size, version, compression_type, predicted_parallelism_type, keytype, nonce),
            Self::EncryptedArchive { name, directory, size, version, entries_number, keytype } => display_encrypted_archive(name, directory, size, version, entries_number, keytype),
            Self::Folder { name, directory, size, permissions } => display_plaintext_folder(name, directory, size, permissions),
        }
        Ok(())
    }
}

fn display_plaintext_file(name: Option<String>, directory: Option<String>, size: Option<u64>, permissions: Option<u32>, extension: Option<String>, mime_extension: Option<String>, ct: Option<CompressionType>, pt: Option<ParallelismType>) {
    // If everything is `None`, then we return that an error occured.
    if name.is_none() 
        && directory.is_none() 
        && size.is_none() 
        && permissions.is_none() 
        && extension.is_none() 
        && mime_extension.is_none()
        && ct.is_none()
        && pt.is_none() {
        error_while_inspecting();
        return
    }
    
    let mut report = Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::new("Plaintext File (Not encrypted with Enkryptit!)", EnkryptitStyle::Orange))
        .field(ReportArgument::label("Name"), ReportArgument::value(format_option(name)))
        .field(ReportArgument::label("Directory"), ReportArgument::value(format_option(directory)))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(format_option_to_kb(size)));

    if permissions.is_some() {
        match FullPermission::new(permissions.unwrap()) {
            Ok(fp) => {
                report = report.field(ReportArgument::label("Permissions"), ReportArgument::value(fp.to_string()));    
            },
            Err(e) => {
                let err: EnkryptitError = e.into();
                err.to_output().display();
            }
        };
    }

    report.field(ReportArgument::label("Shown extension"), ReportArgument::value(format_option(extension)))
        .field(ReportArgument::label("Real extension"), ReportArgument::accent(format_option(mime_extension)))
        .field(ReportArgument::label("Recommanded compression type  (for encryption)"), ReportArgument::accent(format_option(ct)))
        .field(ReportArgument::label("Recommanded parallelism type (for encryption)"), ReportArgument::accent(format_option(pt)))
        .display();

}

fn display_encrypted_file(name: Option<String>, directory: Option<String>, size: Option<u64>, version: u8, ct: Option<CompressionType>, pt: Option<ParallelismType>, keytype: Option<KeyType>, nonce: Option<[u8; 24]>) {
    
    // If everything is `None`, then we return that an error occured.
    if name.is_none() 
        && directory.is_none() 
        && size.is_none() 
        && keytype.is_none()
        && nonce.is_none()
        && ct.is_none()
        && pt.is_none() {
        error_while_inspecting();
        return
    }
    
    Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("File encrypted with Enkryptit!"))
        .field(ReportArgument::label("Version"), ReportArgument::accent(version.to_string()))
        .field(ReportArgument::label("Name"), ReportArgument::value(format_option(name)))
        .field(ReportArgument::label("Directory"), ReportArgument::value(format_option(directory)))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(format_option_to_kb(size)))
        .field(ReportArgument::label("Compression type"), ReportArgument::accent(format_option(ct)))
        .field(ReportArgument::label("Recommended parallelism type (for decrypting)"), ReportArgument::accent(format_option(pt)))
        .field(ReportArgument::label("Key Type"), ReportArgument::accent(format_option(keytype)))
        .field(ReportArgument::label("Nonce"), ReportArgument::accent(format_nonce(nonce)))
        .display();

}

fn display_encrypted_archive(name: Option<String>, directory: Option<String>, size: Option<u64>, version: u8, entries_number: Option<u64>, keytype: Option<KeyType>) {
    
    // If everything is `None`, then we return that an error occured.
    if name.is_none() 
        && directory.is_none() 
        && size.is_none() 
        && keytype.is_none()
        && entries_number.is_none() {

        error_while_inspecting();
        return
    }
    
    Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("Folder (Archive) encrypted with Enkryptit!"))
        .field(ReportArgument::label("Version"), ReportArgument::accent(version.to_string()))
        .field(ReportArgument::label("Name"), ReportArgument::value(format_option(name)))
        .field(ReportArgument::label("Directory"), ReportArgument::value(format_option(directory)))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(format_option_to_kb(size)))
        .field(ReportArgument::label("Entries number"), ReportArgument::accent(format_option(entries_number)))
        .field(ReportArgument::label("Key Type"), ReportArgument::accent(format_option(keytype)))
        .display();

}

fn display_plaintext_folder(name: Option<String>, directory: Option<String>, size: Option<u64>, permissions: Option<u32>) {
    
    // If everything is `None`, then we return that an error occured.
    if name.is_none() 
        && directory.is_none() 
        && size.is_none() 
        && permissions.is_none() {
        error_while_inspecting();
        return
    }

    let mut report = Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("Plaintext Folder (Not encrypted with Enkryptit!)"))
        .field(ReportArgument::label("Name"), ReportArgument::value(format_option(name)))
        .field(ReportArgument::label("Directory"), ReportArgument::value(format_option(directory)))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(format_option_to_kb(size)));

    if permissions.is_some() {
        match FullPermission::new(permissions.unwrap()) {
            Ok(fp) => {
                report = report.field(ReportArgument::label("Permissions"), ReportArgument::value(fp.to_string()));    
            },
            Err(e) => {
                let err: EnkryptitError = e.into();
                err.to_output().display();
            }
        };
    }

    report.display();
}

fn to_kb(size: u64) -> u64 {
    size /  1024
}

fn error_while_inspecting() {
    EnkryptitOutput::error("A severe error occured while inspecting the file.", EnkryptitError::InspectionError)
        .with_help("Please ensure that the file exists, and that its name does not contain wrong unicode character.")
        .with_location("inspect/mod.rs")
        .display();
}

fn format_option<T: std::fmt::Display>(value: Option<T>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| NOT_FOUND.to_string())
}

fn format_option_to_kb(value: Option<u64>) -> String {
    match value {
        Some(value) => to_kb(value).to_string(),
        None => NOT_FOUND.to_string(),
    }
}

fn format_nonce(value: Option<[u8; 24]>) -> String {
    match value {
        Some(value) => {
            value.encode_hex::<String>()
        }
        None => NOT_FOUND.to_string()
    }
}