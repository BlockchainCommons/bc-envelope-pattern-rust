mod error;
mod format;
mod parse;
mod pattern;

pub use dcbor_pattern::{
    Interval, Matcher as DCBORMatcher, Pattern as DCBORPattern, Quantifier,
    Reluctance,
};
pub use error::{Error, Result};
pub use format::{
    FormatPathsOpts, PathElementFormat, format_path, format_path_opt,
    format_paths, format_paths_opt, format_paths_with_captures,
    format_paths_with_captures_opt,
};
pub use pattern::{Matcher, Path, Pattern, dcbor_integration};
