use std::{cell::RefCell, collections::HashMap};

use bc_components::DigestProvider;
use bc_envelope::{EdgeType, Envelope};

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SearchPattern(Box<Pattern>);

impl SearchPattern {
    pub fn new(pattern: Pattern) -> Self { SearchPattern(Box::new(pattern)) }

    pub fn pattern(&self) -> &Pattern { &self.0 }
}

impl Matcher for SearchPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            let result_paths = RefCell::new(Vec::new());

            // State consists of the path from root to current node
            let visitor = |current_envelope: &Envelope,
                        _level: usize,
                        _incoming_edge: EdgeType,
                        path_to_current: Vec<Envelope>|
            -> (Vec<Envelope>, bool) {
                // Create the path to this node
                let mut new_path = path_to_current.clone();
                new_path.push(current_envelope.clone());

                // Test the pattern against this node
                let pattern_paths = self.0.paths(current_envelope);

                // If the pattern matches, emit the full paths
                for pattern_path in pattern_paths {
                    let mut full_path = new_path.clone();
                    // If the pattern path has elements beyond just the current
                    // envelope, extend with those additional
                    // elements. If the pattern path starts with the
                    // current envelope, skip it to avoid duplication.
                    if pattern_path.len() > 1 {
                        full_path.extend(pattern_path.into_iter().skip(1));
                    } else if pattern_path.len() == 1
                        && pattern_path[0].digest() != current_envelope.digest()
                    {
                        // Pattern found a different element, add it to the path
                        full_path.extend(pattern_path);
                    }
                    result_paths.borrow_mut().push(full_path);
                }

                // Continue walking with the new path
                (new_path, false)
            };

            // Start walking from the root with an empty path
            envelope.walk(false, Vec::new(), &visitor);

            let mut seen = std::collections::HashSet::new();
            let mut unique = Vec::new();
            for path in result_paths.into_inner() {
                let digest_path: Vec<_> =
                    path.iter().map(|e| e.digest().into_owned()).collect();
                if seen.insert(digest_path) {
                    unique.push(path);
                }
            }

            unique
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        let idx = lits.len();
        lits.push((*self.0).clone());

        let mut inner_names = Vec::new();
        self.0.collect_capture_names(&mut inner_names);
        let mut map = Vec::new();
        for name in inner_names {
            let pos = if let Some(i) = captures.iter().position(|n| n == &name)
            {
                i
            } else {
                let i = captures.len();
                captures.push(name.clone());
                i
            };
            map.push((name, pos));
        }

        code.push(Instr::Search { pat_idx: idx, capture_map: map });
    }
}

impl std::fmt::Display for SearchPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "search({})", self.pattern())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_pattern_display() {
        let pattern = SearchPattern::new(Pattern::text("test"));
        assert_eq!(pattern.to_string(), r#"search("test")"#);
    }
}
