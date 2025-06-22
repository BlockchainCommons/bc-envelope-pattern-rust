#![allow(dead_code)]

use bc_envelope::{
    base::envelope::EnvelopeCase, format::EnvelopeSummary, prelude::*,
};

use crate::Path;

/// A builder that provides formatting options for each path element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathElementFormat {
    /// Summary format, with optional maximum length for truncation.
    Summary(Option<usize>),
    EnvelopeUR,
    DigestUR,
}

impl Default for PathElementFormat {
    fn default() -> Self { PathElementFormat::Summary(None) }
}

/// Options for formatting paths.
#[derive(Debug, Clone)]
pub struct FormatPathsOpts {
    /// Whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    indent: bool,

    /// Format for each path element.
    /// Default is `PathElementFormat::Summary(None)`.
    element_format: PathElementFormat,

    /// If true, only the last element of each path will be formatted.
    /// This is useful for displaying only the final destination of a path.
    /// If false, all elements will be formatted.
    last_element_only: bool,
}

impl Default for FormatPathsOpts {
    /// Returns the default formatting options:
    /// - `indent`: true
    /// - `element_format`: PathElementFormat::Summary(None)
    /// - `last_element_only`: false
    fn default() -> Self {
        Self {
            indent: true,
            element_format: PathElementFormat::default(),
            last_element_only: false,
        }
    }
}

impl FormatPathsOpts {
    /// Creates a new FormatPathsOpts with default values.
    pub fn new() -> Self { Self::default() }

    /// Sets whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    pub fn indent(mut self, indent: bool) -> Self {
        self.indent = indent;
        self
    }

    /// Sets the format for each path element.
    /// Default is `PathElementFormat::Summary(None)`.
    pub fn element_format(mut self, format: PathElementFormat) -> Self {
        self.element_format = format;
        self
    }

    /// Sets whether to format only the last element of each path.
    /// If true, only the last element will be formatted.
    /// If false, all elements will be formatted.
    pub fn last_element_only(mut self, last_element_only: bool) -> Self {
        self.last_element_only = last_element_only;
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

/// Format each path element on its own line, each line successively indented by
/// 4 spaces. Options can be provided to customize the formatting.
pub fn format_path_opt(
    path: &Path,
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();

    if opts.last_element_only {
        // Only format the last element, no indentation.
        if let Some(element) = path.iter().last() {
            match opts.element_format {
                PathElementFormat::Summary(max_length) => {
                    let summary = envelope_summary(element);
                    truncate_with_ellipsis(&summary, max_length)
                }
                PathElementFormat::EnvelopeUR => element.ur_string(),
                PathElementFormat::DigestUR => element.digest().ur_string(),
            }
        } else {
            String::new()
        }
    } else {
        match opts.element_format {
            PathElementFormat::Summary(max_length) => {
            // Multi-line output with indentation for summaries.
            let mut lines = Vec::new();
            for (index, element) in path.iter().enumerate() {
                let indent = if opts.indent {
                " ".repeat(index * 4)
                } else {
                String::new()
                };

                let summary = envelope_summary(element);
                let content = truncate_with_ellipsis(&summary, max_length);

                lines.push(format!("{}{}", indent, content));
            }
            lines.join("\n")
            }
            PathElementFormat::EnvelopeUR => {
            // Single-line, space-separated envelope URs.
            path.iter()
                .map(|element| element.ur_string())
                .collect::<Vec<_>>()
                .join(" ")
            }
            PathElementFormat::DigestUR => {
            // Single-line, space-separated digest URs.
            path.iter()
                .map(|element| element.digest().ur_string())
                .collect::<Vec<_>>()
                .join(" ")
            }
        }
    }
}

/// Format each path element on its own line, each line successively indented by
/// 4 spaces.
pub fn format_path(path: &Path) -> String {
    format_path_opt(path, FormatPathsOpts::default())
}

/// Format multiple paths with custom formatting options.
pub fn format_paths_opt(
    paths: &[Path],
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();
    match opts.element_format {
        PathElementFormat::EnvelopeUR | PathElementFormat::DigestUR => {
            // Join all formatted paths with a space for UR formats.
            paths
                .iter()
                .map(|path| format_path_opt(path, opts))
                .collect::<Vec<_>>()
                .join(" ")
        }
        PathElementFormat::Summary(_) => {
            // Join all formatted paths with a newline for summary format.
            paths
                .iter()
                .map(|path| format_path_opt(path, opts))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

/// Format multiple paths with default options.
pub fn format_paths(paths: &[Path]) -> String {
    format_paths_opt(paths, FormatPathsOpts::default())
}
