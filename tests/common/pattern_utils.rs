#![allow(dead_code)]

use bc_envelope::prelude::*;
use bc_envelope_pattern::Path;

/// Options for formatting paths.
#[derive(Debug, Clone, Default)]
pub struct FormatPathOpts {
    /// Maximum length of each line's content (after indentation).
    /// If None, no truncation is applied.
    max_length: Option<usize>,
}

impl FormatPathOpts {
    /// Creates a new FormatPathOpts with default values.
    pub fn new() -> Self { Self::default() }

    /// Sets the maximum length of each line's content (after indentation).
    /// If set, content longer than this will be truncated with an ellipsis.
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
}

impl AsRef<FormatPathOpts> for FormatPathOpts {
    fn as_ref(&self) -> &FormatPathOpts { self }
}

// Format each path element on its own line, each line successively indented by
// 4 spaces. Options can be provided to customize the formatting.
pub fn format_path_opt(path: &Path, opts: impl AsRef<FormatPathOpts>) -> String {
    let opts = opts.as_ref();
    let mut lines = Vec::new();
    for (i, element) in path.iter().enumerate() {
        let indent = " ".repeat(i * 4);
        let content = format!(
            "{} {}",
            element.short_id(DigestDisplayFormat::Short),
            element.format_flat()
        );

        let content = if let Some(max_len) = opts.max_length {
            if content.len() > max_len {
                // Ensure we have room for the ellipsis
                if max_len > 1 {
                    format!("{}…", &content[0..(max_len - 1)])
                } else {
                    "…".to_string()
                }
            } else {
                content
            }
        } else {
            content
        };

        lines.push(format!("{}{}", indent, content));
    }
    lines.join("\n")
}

// Format each path element on its own line, each line successively indented by
// 4 spaces.
pub fn format_path(path: &Path) -> String {
    format_path_opt(path, FormatPathOpts::default())
}

// Format multiple paths with custom formatting options.
pub fn format_paths_opt(paths: &[Path], opts: impl AsRef<FormatPathOpts>) -> String {
    let opts = opts.as_ref();
    paths
        .iter()
        .map(|path| format_path_opt(path, opts))
        .collect::<Vec<_>>()
        .join("\n")
}

// Format multiple paths with default options.
pub fn format_paths(paths: &[Path]) -> String {
    format_paths_opt(paths, FormatPathOpts::default())
}
