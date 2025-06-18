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
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        let (paths, mut caps) = self.pattern.paths_with_captures(envelope);
        if !paths.is_empty() {
            caps.entry(self.name.clone())
                .or_default()
                .extend(paths.clone());
        }
        (paths, caps)
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        let id = captures.len();
        captures.push(self.name.clone());
        code.push(Instr::CaptureStart(id));
        self.pattern.compile(code, lits, captures);
        code.push(Instr::CaptureEnd(id));
    }
}

impl std::fmt::Display for CapturePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}({})", self.name, self.pattern)
    }
}
