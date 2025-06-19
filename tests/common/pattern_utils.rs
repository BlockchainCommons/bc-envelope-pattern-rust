#![allow(dead_code)]

use bc_envelope::prelude::*;
use bc_envelope_pattern::Path;

// Format each path element on its own line, each line successively indented by
// 4 spaces. Optionally truncate lines to the specified max_length with an
// ellipsis. The max_length is the maximum length *after* the indent.
pub fn format_path_opt(path: &Path, max_length: Option<usize>) -> String {
    let mut lines = Vec::new();
    for (i, element) in path.iter().enumerate() {
        let indent = " ".repeat(i * 4);
        let content = format!(
            "{} {}",
            element.short_id(DigestDisplayFormat::Short),
            element.format_flat()
        );

        let content = if let Some(max_len) = max_length {
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
pub fn format_path(path: &Path) -> String { format_path_opt(path, None) }

// Format multiple paths with optional line length limit.
pub fn format_paths_opt(paths: &[Path], max_length: Option<usize>) -> String {
    paths
        .iter()
        .map(|path| format_path_opt(path, max_length))
        .collect::<Vec<_>>()
        .join("\n")
}

// Format multiple paths.
pub fn format_paths(paths: &[Path]) -> String { format_paths_opt(paths, None) }
