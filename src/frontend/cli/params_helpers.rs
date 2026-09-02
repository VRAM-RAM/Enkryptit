use crate::log_error;
use crate::parameters::params::{load_params, save_params};
use crate::types::CompressionType;
use crate::types::KeyParams;
use crate::types::ParallelismType;

/// Helper for showing the parameters.
/// It loads the current parameters before priting the parameters, or printing an error.
pub fn show_params() {
    match load_params() {
        Ok(params) => {
            println!("\n Actual parameters :");
            println!(" Key Type : {:?}", params.key_params);
            println!(" Compression : {:?}", params.compression);
            println!(" Parallelism : {:?}", params.parallelism);
            println!();
        }
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}

/// Helper for parsing the parallelism CLI value.
/// Accepts : `single`, `multi` (or `multithread`), `multi:<n>` (or `multi <n>`), `auto`, `automatic`
/// where `n` is a strictly positive number of threads (defaults to 4).
fn parse_parallelism(value: &str) -> Result<ParallelismType, String> {
    let lower = value.to_lowercase();

    if lower == "single" || lower == "no" || lower == "none" {
        return Ok(ParallelismType::Single);
    }

    if lower == "auto" || lower == "automatic" {
        return  Ok(ParallelismType::Auto);
    }
    
    // Support "multi:<n>" and "multi <n>" forms, and plain "multi".
    let (base, count_str) = if let Some(rest) = lower.strip_prefix("multi:") {
        ("multi", Some(rest))
    } else if let Some(rest) = lower.strip_prefix("multi ") {
        ("multi", Some(rest))
    } else if lower == "multi" || lower == "multithread" {
        ("multi", None)
    } else {
        return Err(format!(
            "Unknown parallelism: {value}. Expected `single`, `multi` or `multi:<threads>`"
        ));
    };

    if base != "multi" {
        return Err(format!("Unknown parallelism: {value}"));
    }

    let threads = match count_str {
        Some(s) => s
            .trim()
            .parse::<u8>()
            .map_err(|_| format!("Invalid thread count: `{s}`"))?,
        // Plain "multi" without an explicit count -> pick a sensible default
        None => 4,
    };

    if threads == 0 {
        return Err("The number of threads must be greater than 0".to_string());
    }

    Ok(ParallelismType::MultiThread(threads))
}

/// Helper for updating the parameters.
pub fn update_params(
    compression: Option<String>,
    key_type: Option<String>,
    parallelism: Option<String>,
) {
    match load_params() {
        Ok(mut params) => {
            if let Some(c) = compression {
                match c.to_lowercase().as_str() {
                    "zstd" | "1" => params.compression = CompressionType::Zstd,
                    "lz4" | "2" => params.compression = CompressionType::Lz4,
                    "xz" | "3" => params.compression = CompressionType::Xz,
                    "none" | "no" | "4" => params.compression = CompressionType::NoComp,
                    "auto" | "a" | "5" => params.compression = CompressionType::Auto,
                    other => {
                        log_error!(format!("Unknown compression: {}", other));
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
                        log_error!(format!("Unknown key type: {}", other));
                        std::process::exit(1);
                    }
                }
            }

            if let Some(p) = parallelism {
                match parse_parallelism(&p) {
                    Ok(par) => params.parallelism = par,
                    Err(msg) => {
                        log_error!(msg);
                        std::process::exit(1);
                    }
                }
            }

            match save_params(&params) {
                Ok(_) => println!("\n Params were changed ! \n"),
                Err(e) => {
                    log_error!(format!("Failed to save: {}", e));
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            log_error!(e);
            std::process::exit(1);
        }
    }
}
