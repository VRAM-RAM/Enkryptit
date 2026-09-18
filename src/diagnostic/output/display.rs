use colored::Colorize;
use miette::{GraphicalReportHandler};
use crate::diagnostic::DOC_URL;
use crate::diagnostic::output::diagnostic::{render_error, theme};
use crate::diagnostic::output::Snippet;

use crate::{
    diagnostic::{
        output::kind::{EnkryptitOutputKind},
        EnkryptitOutput,
    },
    errors::EnkryptitError,
};

impl EnkryptitOutput {
    pub fn display(self) {
        match self.kind {
            EnkryptitOutputKind::Info => display_info(&self.msg),
            EnkryptitOutputKind::Phantom => (),
            EnkryptitOutputKind::Error {
                error,
                location,
                help,
                snippet,
            } => display_error(&self.msg, &error, &location, &help, &snippet),
            EnkryptitOutputKind::Success => display_success(&self.msg),
            EnkryptitOutputKind::Warning => display_warning(&self.msg),
        };
    }
}

pub fn display_error(
    msg: &str,
    error: &EnkryptitError,
    location: &Option<String>,
    help: &Option<String>,
    snippet: &Option<Snippet>,
) {
    tracing::error!("{}", error);

    println!("{}", render_error(msg, error, location, help, snippet));
}

fn display_warning(msg: &str) {
    let report = miette::MietteDiagnostic::new(msg)
        .with_url(DOC_URL)
        .with_severity(miette::Severity::Warning);

    let handler = GraphicalReportHandler::new_themed(theme());

    let mut output = String::new();

    handler
        .render_report(&mut output, &report)
        .expect("rendering a miette report into String shouldn't fail");

    tracing::warn!("{}", msg);

    println!("{}", output);
}

const SUCCESS_BOX_WIDTH: usize = 60;
const SUCCESS_BOX_PADDING: usize = 2;
const SUCCESS_BADGE_WIDTH: usize = 3;

fn display_success(msg: &str) {
    tracing::info!("Success : {}", msg);

    let inner_width = SUCCESS_BOX_WIDTH - 4; // both borders + both paddings
    let message_width = inner_width - SUCCESS_BADGE_WIDTH;

    let mut lines = wrap_text(msg, message_width);
    if lines.is_empty() {
        lines.push(String::new());
    }

    let horizontal = "─".repeat(SUCCESS_BOX_WIDTH - 2);

    println!();
    println!("{}", format!("╭{horizontal}╮").green());
    for (i, line) in lines.iter().enumerate() {
        let badge = if i == 0 {
            format!("{}  ", "✔".green().bold())
        } else {
            " ".repeat(SUCCESS_BADGE_WIDTH)
        };
        let text = pad_to(line, message_width);
        println!(
            "{}{}{}{}{}",
            "│".green(),
            " ".repeat(SUCCESS_BOX_PADDING),
            badge,
            text,
            "│".green(),
        );
    }
    println!("{}", format!("╰{horizontal}╯").green());
}

fn display_info(msg: &str) {
    tracing::info!("{}", msg);
    println!("\n{} {}", "[INFO]".cyan().bold(), msg);
}

/// Word-wrap `msg` into lines of at most `width` characters.
///
/// Paragraphs (`\n`) are kept apart and over-long words are hard-split so the
/// text never exceeds the requested width.
fn wrap_text(msg: &str, width: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut line = String::new();

    for paragraph in msg.split('\n') {
        for word in paragraph.split_whitespace() {
            if word.chars().count() > width {
                if !line.is_empty() {
                    wrapped.push(std::mem::take(&mut line));
                }
                let mut word = word.to_string();
                while word.chars().count() > width {
                    let chunk: String = word.chars().take(width).collect();
                    wrapped.push(chunk);
                    word = word.chars().skip(width).collect();
                }
                line = word;
                continue;
            }

            if line.is_empty() {
                line.push_str(word);
            } else if line.chars().count() + 1 + word.chars().count() <= width {
                line.push(' ');
                line.push_str(word);
            } else {
                wrapped.push(std::mem::take(&mut line));
                line.push_str(word);
            }
        }

        if !line.is_empty() {
            wrapped.push(std::mem::take(&mut line));
        }
    }

    wrapped
}

/// Pad `text` on the right with spaces so it is exactly `width` characters.
fn pad_to(text: &str, width: usize) -> String {
    let extra = width.saturating_sub(text.chars().count());
    format!("{text}{}", " ".repeat(extra))
}