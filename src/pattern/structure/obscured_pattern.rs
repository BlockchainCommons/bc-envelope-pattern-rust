use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::{
    Pattern,
    pattern::{
        Matcher, Path, compile_as_atomic, structure::StructurePattern,
        vm::Instr,
    },
};

/// Pattern for matching obscured elements.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ObscuredPattern {
    /// Matches any obscured element.
    Any,
    /// Matches any elided element.
    Elided,
    /// Matches any encrypted element.
    Encrypted,
    /// Matches any compressed element.
    Compressed,
}

impl ObscuredPattern {
    /// Creates a new `ObscuredPattern` that matches any obscured element.
    pub fn any() -> Self { ObscuredPattern::Any }

    /// Creates a new `ObscuredPattern` that matches any elided element.
    pub fn elided() -> Self { ObscuredPattern::Elided }

    /// Creates a new `ObscuredPattern` that matches any encrypted element.
    pub fn encrypted() -> Self { ObscuredPattern::Encrypted }

    /// Creates a new `ObscuredPattern` that matches any compressed element.
    pub fn compressed() -> Self { ObscuredPattern::Compressed }
}

impl Matcher for ObscuredPattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            let is_hit = match self {
                ObscuredPattern::Any => haystack.is_obscured(),
                ObscuredPattern::Elided => haystack.is_elided(),
                ObscuredPattern::Encrypted => haystack.is_encrypted(),
                ObscuredPattern::Compressed => haystack.is_compressed(),
            };

            if is_hit {
                vec![vec![haystack.clone()]]
            } else {
                vec![]
            }
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Structure(StructurePattern::Obscured(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for ObscuredPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObscuredPattern::Any => write!(f, "obscured"),
            ObscuredPattern::Elided => write!(f, "elided"),
            ObscuredPattern::Encrypted => write!(f, "encrypted"),
            ObscuredPattern::Compressed => write!(f, "compressed"),
        }
    }
}
