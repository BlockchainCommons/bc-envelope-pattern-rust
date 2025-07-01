// Pattern module - provides pattern matching functionality for envelopes
mod matcher;
mod pattern_impl;
mod vm;

// Subdirectory modules
mod leaf;
mod meta;
mod structure;

// Integration modules
pub mod dcbor_integration;

// Re-export all types
pub use matcher::{Matcher, Path, compile_as_atomic};
pub use pattern_impl::Pattern;
