use std::{fs::File};
use crate::errors::EnkryptitError;
use crate::context::{LOW_BOUNDARY, MID_INFERIOR_BOUNDARY, MID_SUPERIOR_BOUNDARY, SUPERIOR_BOUNDARY};
use crate::types::ParallelismType;

pub fn infer_parallelism(path: &str) -> Result<ParallelismType, EnkryptitError> {
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let file = File::open(path)?;
    let len = file.metadata()?.len();

    println!("Cpus : {}", cpus);

    Ok(match len {
        0..LOW_BOUNDARY => ParallelismType::Single,
        LOW_BOUNDARY..MID_INFERIOR_BOUNDARY => {
            ParallelismType::MultiThread(4.min(cpus) as u8)
        }
        MID_INFERIOR_BOUNDARY..MID_SUPERIOR_BOUNDARY => {
            ParallelismType::MultiThread(6.min(cpus) as u8)
        }
        MID_SUPERIOR_BOUNDARY..SUPERIOR_BOUNDARY => {
            ParallelismType::MultiThread(8.min(cpus) as u8)
        }
        _ => {
            ParallelismType::MultiThread(cpus as u8)
        }
    })
} 