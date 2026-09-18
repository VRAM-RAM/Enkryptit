//! Regression tests for the miette source-frame error rendering.
//!
//! `render_error` is the pure, testable core of `display_error`. These tests
//! pin down the arrow/line/curve frame drawn when a `Snippet` is attached, and
//! the graceful compact fallback when there is none.

use eck::diagnostic::output::{diagnostic::render_error, snippet::Snippet};
use eck::errors::EnkryptitError;

fn io_error() -> EnkryptitError {
    EnkryptitError::IoError(std::io::Error::from_raw_os_error(2))
}

/// Strip ANSI escape sequences (colors + OSC-8 hyperlinks) so assertions can
/// match on the visible characters regardless of styling.
fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                // CSI: ESC [ ... m
                Some('[') => {
                    while let Some(&n) = chars.peek() {
                        chars.next();
                        if n == 'm' {
                            break;
                        }
                    }
                }
                // OSC: ESC ] ... ESC \
                Some(']') => {
                    while let Some(&n) = chars.peek() {
                        chars.next();
                        if n == '\x1b' {
                            chars.next(); // backslash
                            break;
                        }
                    }
                }
                _ => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn render_error_draws_source_frame_with_snippet() {
    let path = "/tmp/missing.encky";
    let rendered = render_error(
        "io error: No such file or directory (os error 2)",
        &io_error(),
        &Some("I/O".to_string()),
        &Some("Check that the path exists.".to_string()),
        &Some(Snippet::cli_invocation("eck inspect", path)),
    );
    let plain = strip_ansi(&rendered);

    assert!(plain.contains("╭─[eck inspect"), "frame header:\n{plain}");
    assert!(
        plain.contains(&format!("eck inspect {path}")),
        "source line:\n{plain}"
    );
    assert!(plain.contains("╰──"), "curved pointer:\n{plain}");
    assert!(plain.contains("I/O"), "location as pointer label:\n{plain}");
    assert!(plain.contains("help: Check that the path exists."), "help footer:\n{plain}");
}

#[test]
fn render_error_without_snippet_stays_compact() {
    let rendered = render_error(
        "io error: gone",
        &io_error(),
        &Some("I/O".to_string()),
        &Some("Check the path.".to_string()),
        &None,
    );
    let plain = strip_ansi(&rendered);

    assert!(plain.contains("✖ io error: gone"), "message:\n{plain}");
    assert!(plain.contains("help: Check the path. (at I/O)"), "help fold:\n{plain}");
    assert!(!plain.contains("╭─["), "no frame without snippet:\n{plain}");
    assert!(!plain.contains("╰──"), "no pointer without snippet:\n{plain}");
}

#[test]
fn render_error_without_help_or_location_is_still_viable() {
    let rendered = render_error("boom", &io_error(), &None, &None, &None);
    assert!(strip_ansi(&rendered).contains("✖ boom"));
}