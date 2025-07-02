use bc_envelope::prelude::*;

use crate::{
    DCBORMatcher, DCBORPattern, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching CBOR values with support for exact values and advanced
/// pattern matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CBORPattern {
    /// Matches any CBOR value.
    Any,
    /// Matches the specific CBOR value.
    Value(CBOR),
    /// Matches CBOR values using dcbor-pattern expressions.
    Pattern(DCBORPattern),
}

impl CBORPattern {
    /// Creates a new `CBORPattern` that matches any CBOR value.
    pub fn any() -> Self {
        CBORPattern::Any
    }

    /// Creates a new `CBORPattern` that matches a specific CBOR value.
    pub fn value(cbor: impl CBOREncodable) -> Self {
        CBORPattern::Value(cbor.to_cbor())
    }

    /// Creates a new `CBORPattern` that matches CBOR values using dcbor-pattern
    /// expressions.
    pub fn pattern(pattern: DCBORPattern) -> Self {
        CBORPattern::Pattern(pattern)
    }

    /// Creates a new `CBORPattern` from a dcbor-pattern Pattern.
    pub fn from_dcbor_pattern(dcbor_pattern: DCBORPattern) -> Self {
        CBORPattern::Pattern(dcbor_pattern)
    }

    /// Convert dcbor captures to envelope captures by converting dcbor paths
    /// to envelope paths.
    fn convert_dcbor_captures_to_envelope_captures(
        dcbor_captures: std::collections::HashMap<String, Vec<Vec<CBOR>>>,
        base_envelope: &Envelope,
    ) -> std::collections::HashMap<String, Vec<Path>> {
        let mut envelope_captures = std::collections::HashMap::new();

        for (capture_name, dcbor_capture_paths) in dcbor_captures {
            let envelope_capture_paths: Vec<Path> = dcbor_capture_paths
                .into_iter()
                .map(|dcbor_path| {
                    Self::convert_dcbor_path_to_envelope_path(
                        dcbor_path,
                        base_envelope,
                    )
                })
                .collect();
            envelope_captures.insert(capture_name, envelope_capture_paths);
        }

        envelope_captures
    }

    /// Convert a single dcbor path to an envelope path.
    fn convert_dcbor_path_to_envelope_path(
        dcbor_path: Vec<CBOR>,
        base_envelope: &Envelope,
    ) -> Vec<Envelope> {
        let mut envelope_path = vec![base_envelope.clone()];

        // Skip first element if it matches the base envelope's CBOR content
        let skip_first =
            if let Some(base_cbor) = base_envelope.subject().as_leaf() {
                dcbor_path
                    .first()
                    .map(|first| first == &base_cbor)
                    .unwrap_or(false)
            } else {
                false
            };

        let elements_to_add: Vec<_> = if skip_first {
            dcbor_path.into_iter().skip(1).collect()
        } else {
            dcbor_path
        };

        for cbor_element in elements_to_add {
            envelope_path.push(Envelope::new(cbor_element));
        }

        envelope_path
    }

    /// Collect capture names from a dcbor pattern
    fn collect_dcbor_capture_names(
        &self,
        dcbor_pattern: &DCBORPattern,
        names: &mut Vec<String>,
    ) {
        // For now, parse the pattern string to extract capture names
        // This is a simple approach until dcbor-pattern provides a better API
        let pattern_str = dcbor_pattern.to_string();

        // Simple regex-like parsing to find @name( patterns
        let mut chars = pattern_str.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '@' {
                let mut name = String::new();
                // Collect characters until we hit '('
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '(' {
                        break;
                    }
                    name.push(chars.next().unwrap());
                }
                if !name.is_empty() && !names.contains(&name) {
                    names.push(name);
                }
            }
        }
    }
}

impl std::hash::Hash for CBORPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            CBORPattern::Any => {
                0u8.hash(state);
            }
            CBORPattern::Value(cbor) => {
                1u8.hash(state);
                cbor.hash(state);
            }
            CBORPattern::Pattern(pattern) => {
                2u8.hash(state);
                // Hash the string representation since DCBORPattern doesn't
                // implement Hash
                pattern.to_string().hash(state);
            }
        }
    }
}

impl Matcher for CBORPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        let subject = envelope.subject();

        // Special case for KnownValue
        if let Some(known_value) = subject.as_known_value() {
            return match self {
                CBORPattern::Any => (
                    vec![vec![envelope.clone()]],
                    std::collections::HashMap::new(),
                ),
                CBORPattern::Value(expected_cbor) => {
                    // Create CBOR from the KnownValue for comparison
                    let known_value_cbor = known_value.to_cbor();
                    if &known_value_cbor == expected_cbor {
                        (
                            vec![vec![envelope.clone()]],
                            std::collections::HashMap::new(),
                        )
                    } else {
                        (vec![], std::collections::HashMap::new())
                    }
                }
                CBORPattern::Pattern(dcbor_pattern) => {
                    // Create CBOR from the KnownValue for pattern matching
                    let known_value_cbor = known_value.to_cbor();
                    let (dcbor_paths, dcbor_captures) =
                        dcbor_pattern.paths_with_captures(&known_value_cbor);

                    if !dcbor_paths.is_empty() {
                        // Convert dcbor paths to envelope paths by extending
                        // the base envelope path
                        let base_path = vec![envelope.clone()];
                        let envelope_paths: Vec<Path> = dcbor_paths
                            .into_iter()
                            .map(|dcbor_path| {
                                let mut extended_path = base_path.clone();
                                // Convert each CBOR in the dcbor path to an
                                // Envelope and append
                                // Skip the first element as it represents the
                                // root CBOR that we already have as the
                                // envelope
                                for cbor_element in
                                    dcbor_path.into_iter().skip(1)
                                {
                                    extended_path
                                        .push(Envelope::new(cbor_element));
                                }
                                extended_path
                            })
                            .collect();

                        // Convert dcbor captures to envelope captures
                        let envelope_captures =
                            Self::convert_dcbor_captures_to_envelope_captures(
                                dcbor_captures,
                                envelope,
                            );
                        (envelope_paths, envelope_captures)
                    } else {
                        (vec![], std::collections::HashMap::new())
                    }
                }
            };
        }

        // Standard case for CBOR leaf
        let subject_cbor = match subject.as_leaf() {
            Some(cbor) => cbor,
            None => return (vec![], std::collections::HashMap::new()),
        };

        match self {
            CBORPattern::Any => (
                vec![vec![envelope.clone()]],
                std::collections::HashMap::new(),
            ),
            CBORPattern::Value(expected_cbor) => {
                if subject_cbor == *expected_cbor {
                    (
                        vec![vec![envelope.clone()]],
                        std::collections::HashMap::new(),
                    )
                } else {
                    (vec![], std::collections::HashMap::new())
                }
            }
            CBORPattern::Pattern(dcbor_pattern) => {
                let (dcbor_paths, dcbor_captures) =
                    dcbor_pattern.paths_with_captures(&subject_cbor);

                if !dcbor_paths.is_empty() {
                    // Convert dcbor paths to envelope paths by extending the
                    // base envelope path
                    let base_path = vec![envelope.clone()];

                    let envelope_paths: Vec<Path> = dcbor_paths
                        .into_iter()
                        .map(|dcbor_path| {
                            let mut extended_path = base_path.clone();
                            // Convert each CBOR in the dcbor path to an
                            // Envelope and append
                            // Skip the first element only if it exactly matches
                            // our root CBOR
                            let skip_first = dcbor_path
                                .first()
                                .map(|first| first == &subject_cbor)
                                .unwrap_or(false);

                            let elements_to_add: Box<
                                dyn Iterator<Item = dcbor::CBOR>,
                            > = if skip_first {
                                Box::new(dcbor_path.into_iter().skip(1))
                            } else {
                                Box::new(dcbor_path.into_iter())
                            };

                            for cbor_element in elements_to_add {
                                extended_path.push(Envelope::new(cbor_element));
                            }
                            extended_path
                        })
                        .collect();

                    // Convert dcbor captures to envelope captures
                    let envelope_captures =
                        Self::convert_dcbor_captures_to_envelope_captures(
                            dcbor_captures,
                            envelope,
                        );
                    (envelope_paths, envelope_captures)
                } else {
                    (vec![], std::collections::HashMap::new())
                }
            }
        }
    }

    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        self.paths_with_captures(envelope).0
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        // Register any capture names from this CBOR pattern
        if let CBORPattern::Pattern(dcbor_pattern) = self {
            let mut capture_names = Vec::new();
            self.collect_dcbor_capture_names(dcbor_pattern, &mut capture_names);
            for name in capture_names {
                if !captures.contains(&name) {
                    captures.push(name);
                }
            }
        }

        // Use standard atomic compilation - the VM will handle captures
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Cbor(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for CBORPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CBORPattern::Any => write!(f, "CBOR"),
            CBORPattern::Value(cbor) => {
                write!(f, "CBOR({})", cbor.diagnostic_flat())
            }
            CBORPattern::Pattern(pattern) => {
                write!(f, "CBOR(/{}/)", pattern)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbor_pattern_any() {
        let envelope = Envelope::new("test");
        let pattern = CBORPattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);
    }

    #[test]
    fn test_cbor_pattern_exact() {
        let value = "test_value";
        let envelope = Envelope::new(value);
        let cbor = envelope.subject().as_leaf().unwrap().clone();
        let pattern = CBORPattern::value(cbor);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with different value
        let different_envelope = Envelope::new("different");
        let paths = pattern.paths(&different_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_cbor_pattern_dcbor_pattern() {
        // Test with a simple dcbor pattern that matches any number
        let dcbor_pattern = DCBORPattern::any_number();
        let cbor_pattern = CBORPattern::pattern(dcbor_pattern);

        // Test with a number envelope
        let number_envelope = Envelope::new(42);
        let paths = cbor_pattern.paths(&number_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![number_envelope.clone()]);

        // Test with a non-number envelope
        let text_envelope = Envelope::new("hello");
        let paths = cbor_pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_cbor_pattern_dcbor_pattern_with_captures() {
        // Test with a dcbor pattern that has captures
        let dcbor_pattern = DCBORPattern::any_number();
        let cbor_pattern = CBORPattern::pattern(dcbor_pattern);

        let number_envelope = Envelope::new(42);
        let (paths, captures) =
            cbor_pattern.paths_with_captures(&number_envelope);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![number_envelope.clone()]);
        // For a simple pattern without explicit captures, the captures map
        // should be empty
        assert!(captures.is_empty());
    }
}
