use crate::{diagnostic::{output::kind::{EnkryptitOutputKind}}, errors::EnkryptitError};
use crate::diagnostic::output::snippet::Snippet;

pub mod snippet;
pub mod kind;
pub mod diagnostic;
pub mod display;


pub struct EnkryptitOutput {
    kind: EnkryptitOutputKind,
    msg: String,
}

impl EnkryptitOutput {
    pub fn new(kind: EnkryptitOutputKind, msg: impl Into<String>) -> Self {
        Self { kind, msg: msg.into() }
    }

    pub fn phantom() -> Self {
        Self { kind: EnkryptitOutputKind::Phantom, msg: String::new() }
    }

    pub fn success(msg: impl Into<String>) -> Self {
        Self { kind: EnkryptitOutputKind::Success, msg: msg.into() }
    }

    pub fn info(msg: impl Into<String>) -> Self {
        Self { kind: EnkryptitOutputKind::Info, msg: msg.into() }
    }

    pub fn warning(msg: impl Into<String>) -> Self {
        Self { kind: EnkryptitOutputKind::Warning, msg: msg.into() }
    }

    pub fn error(msg: impl Into<String>, error: EnkryptitError) -> Self {
        Self { kind: EnkryptitOutputKind::Error { error, location: None, help: None, snippet: None }, msg: msg.into() }
    }

    /// Attach a location hint (e.g. the operation/step that failed).
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        if let EnkryptitOutputKind::Error { location: slot, .. } = &mut self.kind {
            *slot = Some(location.into());
        }
        self
    }

    /// Attach a remediation hint for the user.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        if let EnkryptitOutputKind::Error { help: slot, .. } = &mut self.kind {
            *slot = Some(help.into());
        }
        self
    }

    /// Attach a source snippet used to draw a miette code frame pointing at
    /// the offending token. Only meaningful for error outputs.
    pub fn with_snippet(mut self, snippet: Snippet) -> Self {
        if let EnkryptitOutputKind::Error { snippet: slot, .. } = &mut self.kind {
            *slot = Some(snippet);
        }
        self
    }
}