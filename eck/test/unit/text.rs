//! Tests for the display-width-aware helpers that render the success box
//! (`wrap_text`, `pad_to`, `success_message_width`).
//!
//! Terminals measure text in *display columns*: combining marks such as the
//! decomposed accents macOS puts in filenames ("é" = e + U+0301) take 0 columns
//! and wide CJK takes 2. These tests pin the invariants that keep the box
//! borders aligned no matter the message contents.

use eck::diagnostic::output::display::{
    SUCCESS_BADGE_WIDTH, SUCCESS_BOX_PADDING, pad_to, success_message_width, wrap_text,
};
use unicode_width::UnicodeWidthStr;

/// Realistic macOS path with *decomposed* accents ("é" = e + U+0301,
/// "à" = a + U+0300), the exact regression that desynced the box borders.
const ACCENTED: &str = "File was encrypted at /Users/axel/captures/Capture d'e\u{301}cran 2026-09-08 a\u{300} 07.07.22.png.encky";

#[test]
fn wrap_keeps_every_line_within_width() {
    let lines = wrap_text(ACCENTED, 53);
    assert!(!lines.is_empty());
    for line in &lines {
        assert!(
            line.width() <= 53,
            "line got display width {}: {line:?}",
            line.width()
        );
    }
    assert!(
        lines.iter().any(|l| l.contains('\u{301}')),
        "test must exercise the combining-mark path"
    );
}

/// The whole success frame — top/bottom bars and every content line — must be
/// exactly `box_width` display columns, so the right border lines up with the
/// corners. `success_message_width` is shared with `display_success`, so a
/// regression in the layout math (like padding the left side twice) shows up
/// here as a line that is too short.
#[test]
fn success_frame_share_one_width() {
    const BOX_WIDTH: usize = 100;
    let message_width = success_message_width(BOX_WIDTH);
    let horizontal = "─".repeat(BOX_WIDTH - 2);

    let top = format!("╭{horizontal}╮");
    let bottom = format!("╰{horizontal}╯");
    assert_eq!(top.width(), BOX_WIDTH);
    assert_eq!(bottom.width(), BOX_WIDTH);

    for line in wrap_text(ACCENTED, message_width) {
        let rendered = format!(
            "{}{}{}{}{}",
            "│",
            " ".repeat(SUCCESS_BOX_PADDING),
            " ".repeat(SUCCESS_BADGE_WIDTH),
            pad_to(&line, message_width),
            "│",
        );
        assert_eq!(rendered.width(), BOX_WIDTH, "misaligned line: {line:?}");
    }
}

#[test]
fn hard_split_never_tears_combining_marks() {
    let word = "e\u{301}".repeat(8);
    let lines = wrap_text(&word, 3);

    assert_eq!(lines.len(), 3); // 3 + 3 + 2 graphemes of width 1
    for chunk in &lines {
        assert!(
            !chunk.starts_with('\u{301}'),
            "leading combining mark: {chunk:?}"
        );
        assert_eq!(
            chunk.chars().count() % 2,
            0,
            "base+mark torn apart: {chunk:?}"
        );
        assert!(chunk.width() <= 3);
    }

    let joined: String = lines.concat();
    assert_eq!(joined, word);
    assert_eq!(joined.width(), word.width());
}

#[test]
fn wrap_respects_paragraphs_and_word_boundaries() {
    assert_eq!(
        wrap_text("one two three\nfour", 7),
        ["one two", "three", "four"]
    );
}

#[test]
fn wrap_flushes_current_line_before_hard_splitting() {
    assert_eq!(wrap_text("pre abcdefghij", 5), ["pre", "abcde", "fghij"]);
}

#[test]
fn wrap_hard_splits_overlong_words() {
    let lines = wrap_text("abcdefghijklmnop", 5);
    assert_eq!(lines, ["abcde", "fghij", "klmno", "p"]);
    for line in &lines {
        assert!(line.width() <= 5);
    }
}

#[test]
fn wrap_handles_empty_or_blank_input() {
    assert!(wrap_text("", 10).is_empty());
    assert!(wrap_text("   ", 10).is_empty());
    assert!(wrap_text("\n\n", 10).is_empty());
}

#[test]
fn pad_to_uses_display_columns() {
    let accented = pad_to("e\u{301}", 5);
    assert_eq!(accented.chars().count(), 6, "2 chars + 4 spaces");
    assert_eq!(accented.width(), 5);

    let wide = pad_to("日", 4);
    assert_eq!(wide.chars().count(), 3, "1 wide char + 2 spaces");
    assert_eq!(wide.width(), 4);

    assert_eq!(pad_to("abc", 5), "abc  ");
}

#[test]
fn message_width_geometry_is_consistent() {
    // border + left padding + badge + text + border == total box width
    let text = success_message_width(60);
    assert_eq!(1 + SUCCESS_BOX_PADDING + SUCCESS_BADGE_WIDTH + text + 1, 60);
}
