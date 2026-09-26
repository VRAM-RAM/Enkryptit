use crate::diagnostic::output::Snippet;
use crate::errors::EnkryptitError;

/// The kind of an [`EnkryptitOutput`](crate::diagnostic::EnkryptitOutput). Defines what type of information the `EnkryptitOutput` contains and so
/// has an impact on the way the `EnkryptitOutput` is displayed.
pub enum EnkryptitOutputKind {
    /// An error. Contains :
    /// - The error itself (as an [`EnkryptitError`])
    /// - The location of the error (**optional**)
    /// - The help message associated (**optional**)
    /// - A [`Snippet`] (**optionnal**)
    Error {
        error: EnkryptitError,
        location: Option<String>,
        help: Option<String>,
        snippet: Option<Snippet>,
    },

    Success,
    Warning,
    /// Informational output. Currently unused (the TUI call sites were taken
    /// out during the WIP rework); kept as public API for future messages.
    #[allow(dead_code)]
    Info,

    /// The kind of an empty [`EnkryptitOutput`](crate::diagnostic::EnkryptitOutput). May be removed in a later version, but used for now in [`InspectionReport`](crate::treatment::inspect::InspectionReport)
    Phantom,
}
