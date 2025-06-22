#![allow(dead_code)]

use bc_envelope::{
    base::envelope::EnvelopeCase, format::EnvelopeSummary, prelude::*,
};
use crate::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PathElementFormat {
    #[default]
    Summary,
    EnvelopeUR,
    DigestUR,
}

/// Options for formatting paths.
#[derive(Debug, Clone)]
pub struct FormatPathsOpts {
    /// Maximum length of each line's content (after indentation).
    /// If None, no truncation is applied.
    max_length: Option<usize>,

    /// Whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    indent: bool,

    /// Format for each path element.
    /// Default is `PathElementFormat::Summary`.
    element_format: PathElementFormat,
}

impl Default for FormatPathsOpts {
    /// Returns the default formatting options:
    /// - `max_length`: None (no truncation)
    /// - `indent`: true
    fn default() -> Self {
        Self {
            max_length: None,
            indent: true,
            element_format: PathElementFormat::default(),
        }
    }
}

impl FormatPathsOpts {
    /// Creates a new FormatPathsOpts with default values.
    pub fn new() -> Self { Self::default() }

    /// Sets the maximum length of each line's content (after indentation).
    /// If set, content longer than this will be truncated with an ellipsis.
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Sets whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    pub fn indent(mut self, indent: bool) -> Self {
        self.indent = indent;
        self
    }

    /// Sets the format for each path element.
    /// Default is `PathElementFormat::Summary`.
    pub fn element_format(mut self, format: PathElementFormat) -> Self {
        self.element_format = format;
        self
    }
}

impl AsRef<FormatPathsOpts> for FormatPathsOpts {
    fn as_ref(&self) -> &FormatPathsOpts { self }
}

pub fn envelope_summary(env: &Envelope) -> String {
    let id = env.short_id(DigestDisplayFormat::Short);
    let summary = match env.case() {
        EnvelopeCase::Node { .. } => {
            format!("NODE {}", env.format_flat())
        }
        EnvelopeCase::Leaf { cbor, .. } => {
            format!(
                "LEAF {}",
                cbor.envelope_summary(usize::MAX, &FormatContextOpt::default())
                    .unwrap_or_else(|_| "ERROR".to_string())
            )
        }
        EnvelopeCase::Wrapped { .. } => {
            format!("WRAPPED {}", env.format_flat())
        }
        EnvelopeCase::Assertion(_) => {
            format!("ASSERTION {}", env.format_flat())
        }
        EnvelopeCase::Elided(_) => "ELIDED".to_string(),
        EnvelopeCase::KnownValue { value, .. } => {
            let content = with_format_context!(|ctx: &FormatContext| {
                let known_value = KnownValuesStore::known_value_for_raw_value(
                    value.value(),
                    Some(ctx.known_values()),
                );
                format!("'{}'", known_value)
            });
            format!("KNOWN_VALUE {}", content)
        }
        EnvelopeCase::Encrypted(_) => "ENCRYPTED".to_string(),
        EnvelopeCase::Compressed(_) => "COMPRESSED".to_string(),
    };
    format!("{} {}", id, summary)
}

/// Truncates a string to the specified maximum length, appending an ellipsis if
/// truncated. If `max_length` is None, returns the original string.
fn truncate_with_ellipsis(s: &str, max_length: Option<usize>) -> String {
    match max_length {
        Some(max_len) if s.len() > max_len => {
            if max_len > 1 {
                format!("{}…", &s[0..(max_len - 1)])
            } else {
                "…".to_string()
            }
        }
        _ => s.to_string(),
    }
}

// Format each path element on its own line, each line successively indented by
// 4 spaces. Options can be provided to customize the formatting.
pub fn format_path_opt(
    path: &Path,
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();
    let mut lines = Vec::new();
    for (index, element) in path.iter().enumerate() {
        let indent = if opts.indent {
            " ".repeat(index * 4)
        } else {
            String::new()
        };

        let content = match opts.element_format {
            PathElementFormat::Summary => {
                let summary = envelope_summary(element);
                truncate_with_ellipsis(&summary, opts.max_length)
            }
            PathElementFormat::EnvelopeUR => element.ur_string(),
            PathElementFormat::DigestUR => element.digest().ur_string(),
        };

        lines.push(format!("{}{}", indent, content));
    }
    lines.join("\n")
}

// Format each path element on its own line, each line successively indented by
// 4 spaces.
pub fn format_path(path: &Path) -> String {
    format_path_opt(path, FormatPathsOpts::default())
}

// Format multiple paths with custom formatting options.
pub fn format_paths_opt(
    paths: &[Path],
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();
    paths
        .iter()
        .map(|path| format_path_opt(path, opts))
        .collect::<Vec<_>>()
        .join("\n")
}

// Format multiple paths with default options.
pub fn format_paths(paths: &[Path]) -> String {
    format_paths_opt(paths, FormatPathsOpts::default())
}
