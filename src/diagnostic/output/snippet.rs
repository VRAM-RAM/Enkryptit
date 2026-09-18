
/// Context used to draw a miette source frame around an error.
///
/// Points at the offending token inside a synthetic *source* line so that
/// miette can render the `╭─[…] │ · ─┬─ ╰──` snippet, e.g.:
///
/// ```text
/// io::misc_io_error (link)
///
///  ✖ io error: No such file or directory (os error 2)
///   ╭─[eck:1:5]
/// 1 │ eck this_directory_is_absurd
///   ·     ────────────┬───────────
///   ·                 ╰── I/O
///   ╰────
///  help: Check that the path exists and that you have the required permissions.
/// ```
pub struct Snippet {
    /// Names the *source* shown in the frame header (e.g. `eck inspect`).
    pub source_name: String,
    /// The surrounding "code" line containing the offending token.
    pub source: String,
    /// `(byte_offset, byte_len)` of the offending token inside `source`.
    pub span: (usize, usize),
    /// Optional label rendered on the curved `╰──` pointer.
    pub label: Option<String>,
}

impl Snippet {
    /// Convenience builder: highlight the `path` token inside a synthetic
    /// command line like `eck inspect <path>`.
    pub fn cli_invocation(program: &str, path: &str) -> Self {
        let source = format!("{program} {path}");
        Self {
            source_name: program.to_string(),
            span: (program.len() + 1, path.len()),
            source,
            label: None,
        }
    }
}