use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{
        Matcher, Path, compile_as_atomic, leaf::LeafPattern,
        vm::Instr,
    },
};

/// Pattern for matching boolean values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BoolPattern {
    /// Matches any boolean value.
    Any,
    /// Matches the specific boolean value.
    Exact(bool),
}

impl BoolPattern {
    /// Creates a new `BoolPattern` that matches any boolean value.
    pub fn any() -> Self { BoolPattern::Any }

    /// Creates a new `BoolPattern` that matches the specific boolean value.
    pub fn exact(value: bool) -> Self { BoolPattern::Exact(value) }
}

impl Matcher for BoolPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let is_hit =
            envelope
                .extract_subject::<bool>()
                .ok()
                .is_some_and(|value| match self {
                    BoolPattern::Any => true,
                    BoolPattern::Exact(want) => value == *want,
                });

        if is_hit {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Bool(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for BoolPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolPattern::Any => write!(f, "BOOL"),
            BoolPattern::Exact(value) => write!(f, "BOOL({})", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_pattern_display() {
        assert_eq!(BoolPattern::any().to_string(), "BOOL");
        assert_eq!(BoolPattern::exact(true).to_string(), "BOOL(true)");
        assert_eq!(BoolPattern::exact(false).to_string(), "BOOL(false)");
    }
}
