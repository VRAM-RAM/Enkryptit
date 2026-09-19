use tracing_subscriber::EnvFilter;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use crate::{directory::project_dir_path, errors::EnkryptitError};

#[allow(unused)]
/// A structure that stores the [`WorkerGuard`] for logging, and that initializes the `tracing` logging.
pub struct EnkryptitLogger(WorkerGuard);

impl EnkryptitLogger {
    /// Initializes the `tracing` logger. 
    pub fn init() -> Result<Self, EnkryptitError> {
        let file = rolling::daily(
            project_dir_path()?,
            "log.txt",
        );

        let (writer, _guard) = tracing_appender::non_blocking(file);
        
        tracing_subscriber::fmt()
            .with_writer(writer)
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

        Ok(Self(_guard))
    }
}