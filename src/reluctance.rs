/// Reluctance for quantifiers.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum Reluctance {
    /// Grabs as many repetitions as possible, then backtracks if the rest of
    /// the pattern cannot match.
    #[default]
    Greedy,
    /// Starts with as few repetitions as possible, adding more only if the rest
    /// of the pattern cannot match.
    Lazy,
    /// Grabs as many repetitions as possible and never backtracks; if the rest
    /// of the pattern cannot match, the whole match fails.
    Possessive,
}

impl Reluctance {
    pub fn suffix(&self) -> &'static str {
        match self {
            Reluctance::Greedy => "",
            Reluctance::Lazy => "?",
            Reluctance::Possessive => "+",
        }
    }
}
