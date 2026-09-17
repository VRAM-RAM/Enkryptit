use std::fs::OpenOptions;
use tracing_subscriber::EnvFilter;

use crate::{directory::project_dir_path, errors::EnkryptitError};

pub struct EnkryptitLogger;

impl EnkryptitLogger {
    pub fn init() -> Result<(), EnkryptitError> {
        let mut path = project_dir_path()?;
        path.push("log.txt");

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        tracing_subscriber::fmt()
            .with_writer(file)
            .with_ansi(false)
            .with_target(false)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info"))
            )
            .init();

        Ok(())
    }
}