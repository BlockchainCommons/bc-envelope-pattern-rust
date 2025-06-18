use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that negates another pattern; matches when the inner pattern does
/// not match.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NotPattern(Box<Pattern>);

impl NotPattern {
    /// Creates a new `NotPattern` with the given pattern.
    pub fn new(pattern: Pattern) -> Self { NotPattern(Box::new(pattern)) }

    pub fn pattern(&self) -> &Pattern { &self.0 }
}

impl Matcher for NotPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        // If the inner pattern doesn't match, then we return the current
        // envelope as a match
        if !self.pattern().matches(envelope) {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    /// Compile into byte-code (NOT = negation of the inner pattern).
    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        // NOT = check that pattern doesn't match
        let idx = literals.len();
        literals.push(self.pattern().clone());
        code.push(Instr::NotMatch { pat_idx: idx });
    }
}

impl std::fmt::Display for NotPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!{}", self.pattern())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Pattern;

    #[test]
    fn test_not_pattern() {
        // Create a simple envelope
        let envelope = Envelope::new("test");

        // Pattern that matches text "test"
        let text_pattern = Pattern::text("test");
        assert!(text_pattern.matches(&envelope));

        // Not pattern that negates the text pattern
        let not_pattern = NotPattern::new(text_pattern);
        assert!(!not_pattern.matches(&envelope));

        // Using the Pattern::not_matching constructor
        assert!(
            !Pattern::not_matching(Pattern::text("test")).matches(&envelope)
        );

        // Not pattern with a pattern that doesn't match
        let not_matching_pattern = Pattern::text("different");
        assert!(!not_matching_pattern.matches(&envelope));

        let double_not = NotPattern::new(not_matching_pattern);
        assert!(double_not.matches(&envelope));

        // Using the Pattern::not_matching constructor
        assert!(
            Pattern::not_matching(Pattern::text("different"))
                .matches(&envelope)
        );
    }

    #[test]
    fn test_not_pattern_paths() {
        // Create a simple envelope
        let envelope = Envelope::new("test");

        // Pattern that doesn't match
        let not_matching_pattern = Pattern::text("different");
        let not_pattern = NotPattern::new(not_matching_pattern);

        let paths = not_pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 1);
        assert_eq!(paths[0][0].extract_subject::<String>().unwrap(), "test");

        // Pattern that does match
        let matching_pattern = Pattern::text("test");
        let not_pattern = NotPattern::new(matching_pattern);

        let paths = not_pattern.paths(&envelope);
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_not_pattern_display() {
        let pattern = NotPattern::new(Pattern::text("test"));
        assert_eq!(pattern.to_string(), r#"!TEXT("test")"#);

        let pattern = NotPattern::new(Pattern::any());
        assert_eq!(pattern.to_string(), "!ANY");
    }
}
