#![allow(clippy::uninlined_format_args)]

mod error;
mod interval;
mod parse;
mod pattern;
mod quantifier;
mod reluctance;

pub use error::{Error, Result};
pub use interval::Interval;
pub use parse::parse_pattern;
pub use pattern::{Matcher, Path, Pattern};
pub use quantifier::Quantifier;
pub use reluctance::Reluctance;
