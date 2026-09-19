use crate::errors::EnkryptitError;
use crate::diagnostic::output::Snippet;

/// The kind of an [`EnkryptitOutput`]. Defines what type of information the `EnkryptitOutput` contains and so 
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
    Info,

    /// The kind of an empty [`EnkryptitOutput`]. May be removed in a later version, but used for now in [`InspectionReport`]
    Phantom,
}