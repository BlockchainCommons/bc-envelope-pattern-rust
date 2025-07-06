use bc_envelope::prelude::*;

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

pub fn format_paths_with_captures(
    paths: &[Path],
    captures: &std::collections::HashMap<String, Vec<Path>>,
) -> String {
    format_paths_with_captures_opt(paths, captures, FormatPathsOpts::default())
}

/// Format multiple paths with captures in a structured way.
/// Captures come first, sorted lexicographically by name, with their name
/// prefixed by '@'. Regular paths follow after all captures.
pub fn format_paths_with_captures_opt(
    paths: &[Path],
    captures: &std::collections::HashMap<String, Vec<Path>>,
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();
    let mut result = Vec::new();

    // First, format all captures, sorted lexicographically by name
    let mut capture_names: Vec<&String> = captures.keys().collect();
    capture_names.sort();

    for capture_name in capture_names {
        if let Some(capture_paths) = captures.get(capture_name) {
            result.push(format!("@{}", capture_name));
            for path in capture_paths {
                let formatted_path = format_path_opt(path, opts);
                // Add indentation to each line of the formatted path
                for line in formatted_path.split('\n') {
                    if !line.is_empty() {
                        result.push(format!("    {}", line));
                    }
                }
            }
        }
    }

    // Then, format all regular paths
    match opts.element_format {
        PathElementFormat::EnvelopeUR | PathElementFormat::DigestUR => {
            // For UR formats, join paths with spaces on same line
            if !paths.is_empty() {
                let formatted_paths = paths
                    .iter()
                    .map(|path| format_path_opt(path, opts))
                    .collect::<Vec<_>>()
                    .join(" ");
                if !formatted_paths.is_empty() {
                    result.push(formatted_paths);
                }
            }
        }
        PathElementFormat::Summary(_) => {
            // For summary format, format each path separately
            for path in paths {
                let formatted_path = format_path_opt(path, opts);
                for line in formatted_path.split('\n') {
                    if !line.is_empty() {
                        result.push(line.to_string());
                    }
                }
            }
        }
    }

    result.join("\n")
}

/// Format multiple paths with custom formatting options.
pub fn format_paths_opt(
    paths: &[Path],
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    // Call format_paths_with_captures with empty captures
    format_paths_with_captures_opt(
        paths,
        &std::collections::HashMap::new(),
        opts,
    )
}

/// Format multiple paths with default options.
pub fn format_paths(paths: &[Path]) -> String {
    format_paths_opt(paths, FormatPathsOpts::default())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bc_envelope::prelude::*;
    use indoc::indoc;

    use super::*;

    fn create_test_path() -> Path {
        vec![
            Envelope::new(42),
            Envelope::new("test"),
            Envelope::new(vec![1, 2, 3]),
        ]
    }

    #[test]
    fn test_format_path_default() {
        let path = create_test_path();
        let actual = format_path(&path);

        #[rustfmt::skip]
        let expected = indoc! {r#"
            7f83f7bd LEAF 42
                6fe3180f LEAF "test"
                    4abc3113 LEAF [1, 2, 3]
        "#}.trim();

        assert_eq!(actual, expected, "format_path with default options");
    }

    #[test]
    fn test_format_path_last_element_only() {
        let path = create_test_path();
        let opts = FormatPathsOpts::new().last_element_only(true);
        let actual = format_path_opt(&path, opts);

        #[rustfmt::skip]
        let expected = indoc! {r#"
            4abc3113 LEAF [1, 2, 3]
        "#}.trim();

        assert_eq!(actual, expected, "format_path with last_element_only");
    }

    #[test]
    fn test_format_paths_multiple() {
        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let paths = vec![path1, path2];

        let actual = format_paths(&paths);

        #[rustfmt::skip]
        let expected = indoc! {r#"
            4bf5122f LEAF 1
            dbc1b4c9 LEAF 2
        "#}.trim();

        assert_eq!(actual, expected, "format_paths with multiple paths");
    }

    #[test]
    fn test_format_paths_with_captures() {
        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let paths = vec![path1.clone(), path2.clone()];

        let mut captures = HashMap::new();
        captures.insert("capture1".to_string(), vec![path1]);
        captures.insert("capture2".to_string(), vec![path2]);

        let actual = format_paths_with_captures_opt(
            &paths,
            &captures,
            FormatPathsOpts::default(),
        );

        #[rustfmt::skip]
        let expected = indoc! {r#"
            @capture1
                4bf5122f LEAF 1
            @capture2
                dbc1b4c9 LEAF 2
            4bf5122f LEAF 1
            dbc1b4c9 LEAF 2
        "#}.trim();

        assert_eq!(
            actual, expected,
            "format_paths_with_captures with sorted captures"
        );
    }

    #[test]
    fn test_format_paths_with_empty_captures() {
        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let paths = vec![path1, path2];

        let captures = HashMap::new();
        let formatted = format_paths_with_captures_opt(
            &paths,
            &captures,
            FormatPathsOpts::default(),
        );

        // Should be same as format_paths when no captures
        let expected = format_paths(&paths);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_capture_names_sorted() {
        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let path3 = vec![Envelope::new(3)];
        let paths = vec![];

        let mut captures = HashMap::new();
        captures.insert("zebra".to_string(), vec![path1]);
        captures.insert("alpha".to_string(), vec![path2]);
        captures.insert("beta".to_string(), vec![path3]);

        let actual = format_paths_with_captures_opt(
            &paths,
            &captures,
            FormatPathsOpts::default(),
        );

        #[rustfmt::skip]
        let expected = indoc! {r#"
            @alpha
                dbc1b4c9 LEAF 2
            @beta
                084fed08 LEAF 3
            @zebra
                4bf5122f LEAF 1
        "#}.trim();

        assert_eq!(
            actual, expected,
            "capture names should be sorted lexicographically"
        );
    }

    #[test]
    fn test_format_paths_with_captures_envelope_ur() {
        bc_components::register_tags();

        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let paths = vec![path1.clone(), path2.clone()];

        let mut captures = HashMap::new();
        captures.insert("capture1".to_string(), vec![path1]);

        let opts = FormatPathsOpts::new()
            .element_format(PathElementFormat::EnvelopeUR);

        let actual = format_paths_with_captures_opt(&paths, &captures, opts);

        // For this test, we need to check the structure but URs are long and
        // variable So we'll verify the structure exists
        assert!(actual.contains("@capture1"));
        assert!(actual.contains("ur:envelope"));

        // Count the number of ur:envelope occurrences (should be 3: 1 capture +
        // 2 regular paths)
        let ur_count = actual.matches("ur:envelope").count();
        assert_eq!(ur_count, 3, "Should have 3 envelope URs total");
    }

    #[test]
    fn test_format_paths_with_captures_digest_ur() {
        bc_components::register_tags();

        let path1 = vec![Envelope::new(1)];
        let path2 = vec![Envelope::new(2)];
        let paths = vec![path1.clone(), path2.clone()];

        let mut captures = HashMap::new();
        captures.insert("capture1".to_string(), vec![path1]);

        let opts =
            FormatPathsOpts::new().element_format(PathElementFormat::DigestUR);

        let actual = format_paths_with_captures_opt(&paths, &captures, opts);

        // For this test, we need to check the structure but digest URs are also
        // variable
        assert!(actual.contains("@capture1"));
        assert!(actual.contains("ur:digest"));

        // Count the number of ur:digest occurrences (should be 3: 1 capture + 2
        // regular paths)
        let ur_count = actual.matches("ur:digest").count();
        assert_eq!(ur_count, 3, "Should have 3 digest URs total");
    }

    #[test]
    fn test_format_paths_with_captures_no_indent() {
        let path1 = vec![Envelope::new(1)];
        let paths = vec![path1.clone()];

        let mut captures = HashMap::new();
        captures.insert("capture1".to_string(), vec![path1]);

        let opts = FormatPathsOpts::new().indent(false);

        let actual = format_paths_with_captures_opt(&paths, &captures, opts);

        #[rustfmt::skip]
        let expected = indoc! {r#"
            @capture1
                4bf5122f LEAF 1
            4bf5122f LEAF 1
        "#}.trim();

        assert_eq!(
            actual, expected,
            "captures should still have fixed indentation even with indent=false"
        );
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", None), "hello");
        assert_eq!(truncate_with_ellipsis("hello", Some(10)), "hello");
        assert_eq!(truncate_with_ellipsis("hello world", Some(5)), "hell…");
        assert_eq!(truncate_with_ellipsis("hello", Some(1)), "…");
    }
}
