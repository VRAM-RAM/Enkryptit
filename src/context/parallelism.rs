use crate::errors::EnkryptitError;
use crate::context::{LOW_BOUNDARY, MID_INFERIOR_BOUNDARY, MID_SUPERIOR_BOUNDARY, SUPERIOR_BOUNDARY};
use crate::types::ParallelismType;

pub fn infer_parallelism(size: u64) -> Result<ParallelismType, EnkryptitError> {
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    Ok(match size {
        0..LOW_BOUNDARY => ParallelismType::Single,
        LOW_BOUNDARY..MID_INFERIOR_BOUNDARY => {
            ParallelismType::MultiThread(8.min(cpus) as u8)
        }
        MID_INFERIOR_BOUNDARY..MID_SUPERIOR_BOUNDARY => {
            ParallelismType::MultiThread(12.min(cpus) as u8)
        }
        MID_SUPERIOR_BOUNDARY..SUPERIOR_BOUNDARY => {
            ParallelismType::MultiThread(16.min(cpus) as u8)
        }
        _ => {
            ParallelismType::MultiThread(cpus as u8)
        }
    })
} 