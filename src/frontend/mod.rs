pub mod cli;
pub mod treat_output;
pub mod tui;
use crate::{errors::EnkryptitError, treatment::inspect::InspectionReport};

/// Enum for Outputs, used in Tui & Cli
pub enum Output {
    Error { error: EnkryptitError },
    Success { message: String },
    InspectionReport(InspectionReport),
    CorruptedFile,
}