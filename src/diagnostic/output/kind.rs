use crate::errors::EnkryptitError;

pub enum EnkryptitOutputKind {
    Error { error: EnkryptitError, location: Option<String>, help: Option<String> },
    Success,
    Warning,
    Info,
    Phantom,
}