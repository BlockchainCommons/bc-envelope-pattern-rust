use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching text values.
#[derive(Debug, Clone)]
pub enum TextPattern {
    /// Matches any text.
    Any,
    /// Matches the specific text.
    Value(String),
    /// Matches the regex for a text.
    Regex(regex::Regex),
}

impl PartialEq for TextPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TextPattern::Any, TextPattern::Any) => true,
            (TextPattern::Value(a), TextPattern::Value(b)) => a == b,
            (TextPattern::Regex(a), TextPattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for TextPattern {}

impl std::hash::Hash for TextPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TextPattern::Any => {
                0u8.hash(state);
            }
            TextPattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            TextPattern::Regex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl TextPattern {
    /// Creates a new `TextPattern` that matches any text.
    pub fn any() -> Self { TextPattern::Any }

    /// Creates a new `TextPattern` that matches the specific text.
    pub fn value<T: Into<String>>(value: T) -> Self {
        TextPattern::Value(value.into())
    }

    /// Creates a new `TextPattern` that matches the regex for a text.
    pub fn regex(regex: regex::Regex) -> Self { TextPattern::Regex(regex) }
}

impl Matcher for TextPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let is_hit =
            envelope
                .extract_subject::<String>()
                .ok()
                .is_some_and(|value| match self {
                    TextPattern::Any => true,
                    TextPattern::Value(want) => value == *want,
                    TextPattern::Regex(regex) => regex.is_match(&value),
                });

        if is_hit {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Text(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for TextPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextPattern::Any => write!(f, "TEXT"),
            TextPattern::Value(value) => write!(f, r#"TEXT("{}")"#, value),
            TextPattern::Regex(regex) => write!(f, r#"TEXT(/{}/)"#, regex),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_pattern_display() {
        assert_eq!(TextPattern::any().to_string(), "TEXT");
        assert_eq!(TextPattern::value("Hello").to_string(), r#"TEXT("Hello")"#);
        assert_eq!(
            TextPattern::regex(regex::Regex::new(r"^\d+$").unwrap())
                .to_string(),
            r#"TEXT(/^\d+$/)"#
        );
    }
}
