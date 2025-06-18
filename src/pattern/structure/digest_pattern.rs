use bc_components::{Digest, DigestProvider};
use bc_envelope::prelude::*;

use crate::{
    Pattern,
    pattern::{
        Matcher, Path, compile_as_atomic,
        structure::StructurePattern, vm::Instr,
    },
};

/// Pattern for matching envelopes by their digest.
#[derive(Debug, Clone)]
pub enum DigestPattern {
    /// Matches the exact digest.
    Digest(Digest),
    /// Matches the prefix of a digest (case insensitive).
    Prefix(Vec<u8>),
    /// Matches the binary regular expression for a digest.
    BinaryRegex(regex::bytes::Regex),
}

impl PartialEq for DigestPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DigestPattern::Digest(a), DigestPattern::Digest(b)) => a == b,
            (DigestPattern::Prefix(a), DigestPattern::Prefix(b)) => {
                a.eq_ignore_ascii_case(b)
            }
            (DigestPattern::BinaryRegex(a), DigestPattern::BinaryRegex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for DigestPattern {}

impl std::hash::Hash for DigestPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DigestPattern::Digest(a) => {
                0u8.hash(state);
                a.hash(state);
            }
            DigestPattern::Prefix(prefix) => {
                1u8.hash(state);
                prefix.hash(state);
            }
            DigestPattern::BinaryRegex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl DigestPattern {
    /// Creates a new `DigestPattern` that matches the exact digest.
    pub fn digest(digest: Digest) -> Self { DigestPattern::Digest(digest) }

    /// Creates a new `DigestPattern` that matches the prefix of a digest.
    pub fn prefix(prefix: impl AsRef<[u8]>) -> Self {
        DigestPattern::Prefix(prefix.as_ref().to_vec())
    }

    /// Creates a new `DigestPattern` that matches the binary regex for a
    /// digest.
    pub fn binary_regex(regex: regex::bytes::Regex) -> Self {
        DigestPattern::BinaryRegex(regex)
    }
}

impl Matcher for DigestPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let digest = envelope.digest();
        let is_hit = match self {
            DigestPattern::Digest(pattern_digest) => *pattern_digest == *digest,
            DigestPattern::Prefix(prefix) => {
                digest.data().starts_with(prefix)
            }
            DigestPattern::BinaryRegex(regex) => regex.is_match(digest.data()),
        };

        if is_hit {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Structure(StructurePattern::Digest(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for DigestPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DigestPattern::Digest(digest) => write!(f, "DIGEST({})", digest),
            DigestPattern::Prefix(prefix) => write!(f, "DIGEST({})", hex::encode(prefix)),
            DigestPattern::BinaryRegex(regex) => write!(f, "DIGEST(/{}/)", regex),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest_pattern_display() {
        let data: &[u8] = b"test";
        let digest = data.digest().into_owned();
        let pattern = DigestPattern::digest(digest.clone());
        assert_eq!(format!("{}", pattern), format!("DIGEST({})", digest));
        let prefix = vec![0x74, 0x65, 0x73]; // "tes"
        let pattern = DigestPattern::prefix(prefix.clone());
        assert_eq!(format!("{}", pattern), format!("DIGEST({})", hex::encode(&prefix)));
        let regex = regex::bytes::Regex::new(r"^te.*").unwrap();
        let pattern = DigestPattern::binary_regex(regex.clone());
        assert_eq!(format!("{}", pattern), format!("DIGEST(/{}/)", regex));
    }
}
