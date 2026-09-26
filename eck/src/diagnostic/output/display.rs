use crate::diagnostic::DOC_URL;
use crate::diagnostic::output::Snippet;
use crate::diagnostic::output::diagnostic::{render_error, theme};
use crate::{
    diagnostic::{EnkryptitOutput, output::kind::EnkryptitOutputKind},
    errors::EnkryptitError,
};
use colored::Colorize;
use miette::GraphicalReportHandler;
use miette::MietteDiagnostic;
use terminal_size::{Width as TermWidth, terminal_size};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

impl EnkryptitOutput {
    /// Matches the [`EnkryptitOutputKind`] that contains the output, and calls the corresponding `display` function.
    /// \
    /// - `Info` => `display_info()`
    /// - `Phantom` => None
    /// - `Error` => `display_error()`
    /// - `Success` => `display_success()`
    /// - `Warning` => `display_warning()`
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

/// Displays an error, calling [`render_error()`] and logging it using [`tracing::error!`].
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

/// Renders a warning into a plain `String`, using [`MietteDiagnostic`] and a
/// graphical handler with **Enkryptit!**'s theme defined in [`theme()`].
///
/// Pure function, so the layout can be unit-tested without touching stdout or
/// the log stream; `display_warning()` is the thin side-effecting wrapper.
pub fn render_warning(msg: &str) -> String {
    let report = MietteDiagnostic::new(msg)
        .with_url(DOC_URL)
        .with_severity(miette::Severity::Warning);

    let mut output = String::new();

    GraphicalReportHandler::new_themed(theme())
        .render_report(&mut output, &report)
        .expect("rendering a miette report into String shouldn't fail");

    output
}

/// Displays a warning, calling [`render_warning()`], logging it using [`tracing::warn!`]
/// and printing it to stdout.
fn display_warning(msg: &str) {
    tracing::warn!("{}", msg);

    println!("{}", render_warning(msg));
}

/// The minimal width of the success box.
const SUCCESS_BOX_MIN_WIDTH: usize = 60;

#[doc(hidden)]
pub const SUCCESS_BOX_PADDING: usize = 2;
#[doc(hidden)]
pub const SUCCESS_BADGE_WIDTH: usize = 3;

/// Fallback terminal width when the size cannot be resolved (e.g. no tty).
const DEFAULT_TERMINAL_WIDTH: usize = 80;

/// Displays a success. Its current flow is :
/// - Logs the **success** calling [`tracing::info`]
/// - Prepares the lines of text calling [`wrap_text()`] and using the constants
/// - Creates the horizontal bar of `-` (for both top and bottom)
/// - Prints the top (`╭`+ horizontal bar + `╮`) in green
/// - Prints and format the lines, adding the badge and the sides `|`
/// - Pirints the bottom (`╰` + horizontal bar + `╯`)
///
/// The box width adapts to the message (never below
/// [`SUCCESS_BOX_MIN_WIDTH`]) and is capped at the terminal width, so paths are
/// usually printed unwrapped on a single aligned line. All lengths are measured
/// in **display columns** (combining marks take 0 columns, wide CJK 2), keeping
/// the borders aligned.
fn display_success(msg: &str) {
    tracing::info!("Success : {}", msg);

    let box_width = choose_box_width(msg);
    let message_width = success_message_width(box_width);

    let mut lines = wrap_text(msg, message_width);
    if lines.is_empty() {
        lines.push(String::new());
    }

    let horizontal = "─".repeat(box_width - 2);

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

/// Number of display columns available for the message text inside a box of
/// `box_width`. The layout is fixed: border + left padding + badge + text +
/// border (the right side has no reserved padding, `pad_to` fills the gap).
#[doc(hidden)]
pub fn success_message_width(box_width: usize) -> usize {
    box_width - SUCCESS_BOX_PADDING - SUCCESS_BADGE_WIDTH - 2
}

/// Pick a box width that fits the (unwrapped) message, never below the minimum
/// and never wider than the terminal.
fn choose_box_width(msg: &str) -> usize {
    let min = SUCCESS_BOX_MIN_WIDTH;
    let content = msg.width() + SUCCESS_BOX_PADDING * 2 + SUCCESS_BADGE_WIDTH + 2;
    let term = terminal_width().saturating_sub(1);

    content.max(min).min(term.max(min))
}

/// Best-effort number of terminal columns, falling back to a default.
fn terminal_width() -> usize {
    terminal_size()
        .map(|(TermWidth(width), _)| usize::from(width))
        .unwrap_or(DEFAULT_TERMINAL_WIDTH)
}

/// Displays and logs an information calling [`tracing::info!`] and using a simple [`println!`] with cyan color.
fn display_info(msg: &str) {
    tracing::info!("{}", msg);
    println!("\n{} {}", "[INFO]".cyan().bold(), msg);
}

/// Word-wrap `msg` into lines of at most `width` **display columns**.
///
/// Paragraphs (`\n`) are kept apart and over-long words are hard-split on
/// grapheme boundaries, so combining marks are never separated from the base
/// character they decorate. The terminal's display width is used throughout
/// (0 for combining marks, 2 for wide CJK), keeping wrapped boxes aligned.
#[doc(hidden)]
pub fn wrap_text(msg: &str, width: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut line = String::new();

    for paragraph in msg.split('\n') {
        for word in paragraph.split_whitespace() {
            if word.width() > width {
                if !line.is_empty() {
                    wrapped.push(std::mem::take(&mut line));
                }
                wrapped.extend(split_word(word, width));
                continue;
            }

            if line.is_empty() {
                line.push_str(word);
            } else if line.width() + 1 + word.width() <= width {
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

/// Split an over-long word into chunks of at most `width` display columns,
/// never cutting inside a grapheme cluster.
fn split_word(word: &str, width: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk = String::new();

    for grapheme in word.graphemes(true) {
        let grapheme_width = grapheme.width().max(1);
        if !chunk.is_empty() && chunk.width() + grapheme_width > width {
            chunks.push(std::mem::take(&mut chunk));
        }
        chunk.push_str(grapheme);
    }

    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    chunks
}

/// Pad `text` on the right with spaces so it is exactly `width` display columns.
#[doc(hidden)]
pub fn pad_to(text: &str, width: usize) -> String {
    let extra = width.saturating_sub(text.width());
    format!("{text}{}", " ".repeat(extra))
}
