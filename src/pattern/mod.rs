// Pattern module - provides pattern matching functionality for envelopes
mod greediness;
mod matcher;
mod pattern_impl;
mod vm;

// Subdirectory modules
mod leaf;
mod meta;
mod structure;

// Re-export all types
pub use greediness::Greediness;
pub use matcher::{Matcher, Path, compile_as_atomic};
pub use pattern_impl::Pattern;
