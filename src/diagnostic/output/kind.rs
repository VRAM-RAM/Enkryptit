use crate::errors::EnkryptitError;
use crate::diagnostic::output::Snippet;

pub enum EnkryptitOutputKind {
    Error {
        error: EnkryptitError,
        location: Option<String>,
        help: Option<String>,
        snippet: Option<Snippet>,
    },
    Success,
    Warning,
    Info,
    Phantom,
}