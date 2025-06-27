mod error;
mod format;
mod parse;
mod pattern;

pub use error::{Error, Result};
pub use format::{
    FormatPathsOpts, PathElementFormat, format_path, format_path_opt,
    format_paths, format_paths_opt,
};
pub use pattern::{Matcher, Path, Pattern};

pub use dcbor_pattern::Interval;
pub use dcbor_pattern::Quantifier;
pub use dcbor_pattern::Reluctance;
