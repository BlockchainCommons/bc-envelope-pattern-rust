//! Simple group wrapper.  For now we only emit SAVE instructions;
//! future work can acquire captures and named captures.

use bc_envelope::Envelope;

use crate::{
    Matcher, Path,
    pattern::{Pattern, vm::Instr},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CapturePattern {
    name: String,
    pattern: Box<Pattern>,
}

impl CapturePattern {
    /// Creates a new `CapturePattern` with the given name and pattern.
    pub fn new(name: impl AsRef<str>, pattern: Pattern) -> Self {
        CapturePattern {
            name: name.as_ref().to_string(),
            pattern: Box::new(pattern),
        }
    }

    /// Returns the name of the capture.
    pub fn name(&self) -> &str { &self.name }

    /// Returns the inner pattern.
    pub fn pattern(&self) -> &Pattern { &self.pattern }
}

impl Matcher for CapturePattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        self.paths_with_captures(envelope).0
    }

    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, std::collections::HashMap<String, Path>) {
        let (paths, mut caps) = self.pattern.paths_with_captures(envelope);
        if let Some(p) = paths.first() {
            caps.insert(self.name.clone(), p.clone());
        }
        (paths, caps)
    }

    fn compile(&self, code: &mut Vec<Instr>, lits: &mut Vec<Pattern>) {
        code.push(Instr::Save); // start
        self.pattern.compile(code, lits);
        code.push(Instr::Save); // end
    }
}

impl std::fmt::Display for CapturePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}({})", self.name, self.pattern)
    }
}
