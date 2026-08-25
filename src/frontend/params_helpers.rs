use crate::parameters::params::{load_params, save_params};
use crate::types::CompressionType;
use crate::types::KeyParams;

/// Helper for showing the parameters.
/// It loads the current parameters before priting the parameters, or printing an error.
pub fn show_params() {
    match load_params() {
        Ok(params) => {
            println!("\n Actual parameters :");
            println!(" Key Type : {:?}", params.key_params);
            println!(" Compression : {:?}", params.compression);
            println!();
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}

/// Helper for updating the parameters.
pub fn update_params(compression: Option<String>, key_type: Option<String>) {
    match load_params() {
        Ok(mut params) => {
            if let Some(c) = compression {
                match c.to_lowercase().as_str() {
                    "zstd" | "1" => params.compression = CompressionType::Zstd,
                    "lz4" | "2" => params.compression = CompressionType::Lz4,
                    "xz" | "3" => params.compression = CompressionType::Xz,
                    "none" | "no" | "4" => params.compression = CompressionType::NoComp,
                    other => {
                        eprintln!("[ERROR] Unknown compression: {}", other);
                        std::process::exit(1);
                    }
                }
            }

            if let Some(kt) = key_type {
                match kt.to_lowercase().as_str() {
                    "password" | "pwd" | "1" => params.key_params = KeyParams::PassWord,
                    "os" | "2" => params.key_params = KeyParams::Os,
                    "file" | "3" => params.key_params = KeyParams::File,
                    other => {
                        eprintln!("[ERROR] Unknown key type: {}", other);
                        std::process::exit(1);
                    }
                }
            }

            match save_params(&params) {
                Ok(_) => println!("\n Params were changed ! \n"),
                Err(e) => {
                    eprintln!("[ERROR] Failed to save: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}