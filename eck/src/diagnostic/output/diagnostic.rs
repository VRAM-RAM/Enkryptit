use std::fmt;
use crate::errors::EnkryptitError;
use crate::diagnostic::output::Snippet;
use crate::diagnostic::DOC_URL;
use miette::{
    Diagnostic, GraphicalReportHandler, GraphicalTheme, LabeledSpan, NamedSource, Severity,
    SourceCode, SourceSpan, ThemeStyles,
};

/// Diagnostic used to render [`EnkryptitOutput`] errors through miette.
///
/// Unlike `MietteDiagnostic`, it implements `source_code()` + `labels()`, so
/// miette can draw the full code frame (`╭─[…]`, `│`, `·`, `─┬─`, `╰──`)
/// around the offending token when a [`Snippet`] is provided.
#[derive(Debug)]
pub struct EnkryptitDiagnostic {
    message: String,
    code: String,
    help: Option<String>,
    url: String,
    severity: Severity,
    source: Option<NamedSource<String>>,
    labels: Vec<LabeledSpan>,
}

impl fmt::Display for EnkryptitDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EnkryptitDiagnostic {}

impl Diagnostic for EnkryptitDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(&self.code) as Box<dyn fmt::Display>)
    }

    fn severity(&self) -> Option<Severity> {
        Some(self.severity)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help
            .as_ref()
            .map(Box::new)
            .map(|c| c as Box<dyn fmt::Display>)
    }

    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(&self.url) as Box<dyn fmt::Display>)
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source.as_ref().map(|s| s as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(self.labels.iter().cloned()))
    }
}

/// Builds the graphical theme used for *all* diagnostics: unicode drawings,
/// RGB colors, per-severity glyphs.
pub fn theme() -> GraphicalTheme {
    let mut theme = GraphicalTheme::unicode();
    theme.styles = ThemeStyles::rgb();
    theme.styles.error = theme.styles.error.bold();
    theme.styles.warning = theme.styles.warning.bold();
    theme.characters.error = "✖".into();
    theme.characters.warning = "⚠".into();
    theme.characters.advice = "☞".into();
    theme
}

/// Renders an error report into a plain `String`.
///
/// Pure function so the layout can be unit-tested without touching stdout.
/// If no [`Snippet`] is provided, the report degrades gracefully to the
/// compact `× message` + `help:` form (no code frame).
pub fn render_error(
    msg: &str,
    error: &EnkryptitError,
    location: &Option<String>,
    help: &Option<String>,
    snippet: &Option<Snippet>,
) -> String {
    let (source, labels) = match snippet {
        Some(snippet) => {
            // Fall back to the location hint as the curved-label text.
            let label = snippet.label.clone().or_else(|| location.clone());
            let source = NamedSource::new(snippet.source_name.clone(), snippet.source.clone());
            let label_span = LabeledSpan::new_with_span(label, SourceSpan::from(snippet.span));
            (Some(source), vec![label_span])
        }
        None => (None, Vec::new()),
    };

    // When a snippet is drawn, the location already appears as the curved
    // label; only fold it into the help text otherwise.
    let help_msg = if snippet.is_some() {
        help.as_ref().map(|h| h.to_string())
    } else {
        match (location, help) {
            (Some(location), Some(help)) => Some(format!("{help} (at {location})")),
            (Some(location), None) => Some(format!("Error occurred at {location}")),
            (None, Some(help)) => Some(help.to_string()),
            (None, None) => None,
        }
    };

    let diagnostic = EnkryptitDiagnostic {
        message: msg.to_string(),
        code: error.code(),
        help: help_msg,
        url: DOC_URL.to_string(),
        severity: Severity::Error,
        source,
        labels,
    };

    let handler = GraphicalReportHandler::new_themed(theme())
        .with_urls(true)
        .with_links(true)
        .with_context_lines(1);

    let mut output = String::new();
    handler
        .render_report(&mut output, &diagnostic)
        .expect("rendering a miette report into String shouldn't fail");
    output
}