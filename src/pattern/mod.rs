// Pattern module - provides pattern matching functionality for envelopes
mod matcher;
mod vm;

// Subdirectory modules
mod leaf;
mod meta;
mod structure;

// Integration modules
pub mod dcbor_integration;

// Re-export all types
use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{RangeBounds, RangeInclusive},
};

use bc_envelope::prelude::*;
use known_values::KnownValue;
pub use matcher::{Matcher, Path, compile_as_atomic};

use self::{
    leaf::{
        ArrayPattern, BoolPattern, ByteStringPattern, DatePattern,
        KnownValuePattern, LeafPattern, MapPattern, NullPattern, NumberPattern,
        TextPattern,
    },
    meta::{
        AndPattern, AnyPattern, CapturePattern, GroupPattern, MetaPattern,
        NotPattern, OrPattern, SearchPattern, TraversePattern,
    },
    structure::{
        AssertionsPattern, DigestPattern, LeafStructurePattern, NodePattern,
        ObjectPattern, ObscuredPattern, PredicatePattern, StructurePattern,
        SubjectPattern, WrappedPattern,
    },
};
use crate::{
    DCBORPattern, Quantifier, Reluctance,
    pattern::{leaf::CBORPattern, vm::Instr},
};

/// The main pattern type used for matching envelopes.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Pattern {
    /// Leaf patterns for matching CBOR values.
    Leaf(LeafPattern),

    /// Structure patterns for matching envelope elements.
    Structure(StructurePattern),

    /// Meta-patterns for combining and modifying other patterns.
    Meta(MetaPattern),
}

impl Matcher for Pattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let results = self.vm_run(haystack);
        let mut paths = Vec::new();
        let mut captures: HashMap<String, Vec<Path>> = HashMap::new();
        for (p, caps) in results {
            paths.push(p);
            for (name, mut vals) in caps {
                captures.entry(name).or_default().append(&mut vals);
            }
        }
        (paths, captures)
    }

    fn is_complex(&self) -> bool {
        match self {
            Pattern::Leaf(leaf) => leaf.is_complex(),
            Pattern::Structure(structure) => structure.is_complex(),
            Pattern::Meta(meta) => meta.is_complex(),
        }
    }
}

// region: Leaf Patterns
//
//

impl Pattern {
    /// Creates a new `Pattern` that matches any CBOR value.
    pub fn any_cbor() -> Self {
        Pattern::Leaf(LeafPattern::Cbor(CBORPattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific CBOR value.
    pub fn cbor(cbor: impl CBOREncodable) -> Self {
        Pattern::Leaf(LeafPattern::Cbor(CBORPattern::value(cbor)))
    }

    /// Creates a new `Pattern` that matches CBOR values using dcbor-pattern
    /// expressions.
    pub fn cbor_pattern(pattern: DCBORPattern) -> Self {
        Pattern::Leaf(LeafPattern::Cbor(CBORPattern::pattern(pattern)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any boolean value.
    pub fn any_bool() -> Self {
        Pattern::Leaf(LeafPattern::Bool(BoolPattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific boolean value.
    pub fn bool(b: bool) -> Self {
        Pattern::Leaf(LeafPattern::Bool(BoolPattern::value(b)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any text value.
    pub fn any_text() -> Self {
        Pattern::Leaf(LeafPattern::Text(TextPattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific text value.
    pub fn text<T: Into<String>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Text(TextPattern::value(value)))
    }

    /// Creates a new `Pattern` that matches text values that match the given
    /// regular expression.
    pub fn text_regex(regex: regex::Regex) -> Self {
        Pattern::Leaf(LeafPattern::Text(TextPattern::regex(regex)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any Date (CBOR tag 1) value.
    pub fn any_date() -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific Date (CBOR tag 1) value.
    pub fn date(date: Date) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::value(date)))
    }

    /// Creates a new `Pattern` that matches Date (CBOR tag 1) values within a
    /// specified range (inclusive).
    pub fn date_range(range: RangeInclusive<Date>) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::range(range)))
    }

    /// Creates a new `Pattern` that matches Date (CBOR tag 1) values that are
    /// on or after the specified date.
    pub fn date_earliest(date: Date) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::earliest(date)))
    }

    /// Creates a new `Pattern` that matches Date (CBOR tag 1) values that are
    /// on or before the specified date.
    pub fn date_latest(date: Date) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::latest(date)))
    }

    /// Creates a new `Pattern` that matches Date (CBOR tag 1) values by their
    /// ISO-8601 string representation.
    pub fn date_iso8601(iso_string: impl Into<String>) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::string(iso_string)))
    }

    /// Creates a new `Pattern` that matches Date (CBOR tag 1) values whose
    /// ISO-8601 string representation matches the given regular expression.
    pub fn date_regex(regex: regex::Regex) -> Self {
        Pattern::Leaf(LeafPattern::Date(DatePattern::regex(regex)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any number value.
    pub fn any_number() -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific number value.
    pub fn number<T: Into<f64>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::exact(value)))
    }

    /// Creates a new `Pattern` that matches number values within a specified
    /// range (inclusive).
    pub fn number_range<A: Into<f64> + Copy>(range: RangeInclusive<A>) -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::range(range)))
    }

    /// Creates a new `Pattern` that matches number values that are greater than
    /// the specified value.
    pub fn number_greater_than<T: Into<f64>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::greater_than(value)))
    }

    /// Creates a new `Pattern` that matches number values that are greater than
    /// or equal to the specified value.
    pub fn number_greater_than_or_equal<T: Into<f64>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Number(
            NumberPattern::greater_than_or_equal(value),
        ))
    }

    /// Creates a new `Pattern` that matches number values that are less than
    /// the specified value.
    pub fn number_less_than<T: Into<f64>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::less_than(value)))
    }

    /// Creates a new `Pattern` that matches number values that are less than or
    /// equal to the specified value.
    pub fn number_less_than_or_equal<T: Into<f64>>(value: T) -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::less_than_or_equal(
            value,
        )))
    }

    /// Creates a new `Pattern` that matches number values that are NaN (Not a
    /// Number).
    pub fn number_nan() -> Self {
        Pattern::Leaf(LeafPattern::Number(NumberPattern::nan()))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any byte string value.
    pub fn any_byte_string() -> Self {
        Pattern::Leaf(LeafPattern::ByteString(ByteStringPattern::any()))
    }

    /// Creates a new `Pattern` that matches a specific byte string value.
    pub fn byte_string(value: impl AsRef<[u8]>) -> Self {
        Pattern::Leaf(LeafPattern::ByteString(ByteStringPattern::value(value)))
    }

    /// Creates a new `Pattern` that matches byte string values that match the
    /// given binary regular expression.
    pub fn byte_string_binary_regex(regex: regex::bytes::Regex) -> Self {
        Pattern::Leaf(LeafPattern::ByteString(ByteStringPattern::regex(regex)))
    }
}

impl Pattern {
    pub fn any_known_value() -> Self {
        Pattern::Leaf(LeafPattern::KnownValue(KnownValuePattern::any()))
    }

    pub fn known_value(value: KnownValue) -> Self {
        Pattern::Leaf(LeafPattern::KnownValue(KnownValuePattern::value(value)))
    }

    pub fn known_value_named<T: Into<String>>(name: T) -> Self {
        Pattern::Leaf(LeafPattern::KnownValue(KnownValuePattern::named(name)))
    }

    pub fn known_value_regex(regex: regex::Regex) -> Self {
        Pattern::Leaf(LeafPattern::KnownValue(KnownValuePattern::regex(regex)))
    }

    pub fn unit() -> Self { Self::known_value(known_values::UNIT) }
}

impl Pattern {
    pub fn any_array() -> Self {
        Pattern::Leaf(LeafPattern::Array(ArrayPattern::any()))
    }

    pub fn array_with_range(interval: impl RangeBounds<usize>) -> Self {
        Pattern::Leaf(LeafPattern::Array(ArrayPattern::interval(interval)))
    }

    pub fn array_with_count(count: usize) -> Self {
        Pattern::Leaf(LeafPattern::Array(ArrayPattern::count(count)))
    }

    /// Creates an array pattern from a dcbor-pattern.
    /// This is used internally by the parser to delegate to dcbor-pattern.
    pub fn array_from_dcbor_pattern(pattern: DCBORPattern) -> Self {
        Pattern::Leaf(LeafPattern::Array(ArrayPattern::from_dcbor_pattern(
            pattern,
        )))
    }
}

impl Pattern {
    pub fn any_map() -> Self {
        Pattern::Leaf(LeafPattern::Map(MapPattern::any()))
    }

    pub fn map_with_range(interval: impl RangeBounds<usize>) -> Self {
        Pattern::Leaf(LeafPattern::Map(MapPattern::interval(interval)))
    }

    pub fn map_with_count(count: usize) -> Self {
        Pattern::Leaf(LeafPattern::Map(MapPattern::interval(count..=count)))
    }
}

impl Pattern {
    pub fn null() -> Self { Pattern::Leaf(LeafPattern::Null(NullPattern)) }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any tagged value.
    /// This is a proxy to dcbor-pattern's tagged functionality.
    pub fn any_tag() -> Self {
        Pattern::Leaf(crate::pattern::leaf::LeafPattern::Tag(
            crate::pattern::leaf::TaggedPattern::any(),
        ))
    }

    /// Creates a new `Pattern` that matches a specific tagged value with any
    /// content. This is a proxy to dcbor-pattern's tagged functionality.
    pub fn tagged(tag: impl Into<Tag>, pattern: DCBORPattern) -> Self {
        Pattern::Leaf(crate::pattern::leaf::LeafPattern::Tag(
            crate::pattern::leaf::TaggedPattern::with_tag(tag, pattern),
        ))
    }

    /// Creates a new `Pattern` that matches a tagged value with specific tag
    /// name and any content. This is a proxy to dcbor-pattern's tagged
    /// functionality.
    pub fn tagged_name(name: impl Into<String>, pattern: DCBORPattern) -> Self {
        Pattern::Leaf(crate::pattern::leaf::LeafPattern::Tag(
            crate::pattern::leaf::TaggedPattern::with_name(
                name.into(),
                pattern,
            ),
        ))
    }

    /// Creates a new `Pattern` that matches a tagged value with tag name
    /// matching regex and any content. This is a proxy to dcbor-pattern's
    /// tagged functionality.
    pub fn tagged_regex(regex: regex::Regex, pattern: DCBORPattern) -> Self {
        Pattern::Leaf(crate::pattern::leaf::LeafPattern::Tag(
            crate::pattern::leaf::TaggedPattern::with_regex(regex, pattern),
        ))
    }

    /// Creates a new `Pattern` that matches a tagged value from a
    /// dcbor_pattern::TaggedPattern. This is an internal helper for the
    /// parser.
    pub(crate) fn tagged_from_dcbor_pattern(
        tagged_pattern: dcbor_pattern::TaggedPattern,
    ) -> Self {
        Pattern::Leaf(crate::pattern::leaf::LeafPattern::Tag(
            crate::pattern::leaf::TaggedPattern::from_dcbor_pattern(
                tagged_pattern,
            ),
        ))
    }
}

//
//
// endregion

// region: Structure Patterns
//
//

impl Pattern {
    pub fn leaf() -> Self {
        Pattern::Structure(StructurePattern::Leaf(LeafStructurePattern::new()))
    }

    pub fn any_assertion() -> Self {
        Pattern::Structure(StructurePattern::Assertions(
            AssertionsPattern::any(),
        ))
    }

    pub fn assertion_with_predicate(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Assertions(
            AssertionsPattern::with_predicate(pattern),
        ))
    }

    pub fn assertion_with_object(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Assertions(
            AssertionsPattern::with_object(pattern),
        ))
    }
}

impl Pattern {
    pub fn any_subject() -> Self {
        Pattern::Structure(StructurePattern::Subject(SubjectPattern::any()))
    }

    pub fn subject(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Subject(SubjectPattern::pattern(
            pattern,
        )))
    }
}

impl Pattern {
    pub fn any_predicate() -> Self {
        Pattern::Structure(StructurePattern::Predicate(PredicatePattern::any()))
    }

    pub fn predicate(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Predicate(
            PredicatePattern::pattern(pattern),
        ))
    }

    pub fn any_object() -> Self {
        Pattern::Structure(StructurePattern::Object(ObjectPattern::any()))
    }

    pub fn object(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Object(ObjectPattern::pattern(
            pattern,
        )))
    }
}

impl Pattern {
    pub fn digest(digest: bc_components::Digest) -> Self {
        Pattern::Structure(StructurePattern::Digest(DigestPattern::digest(
            digest,
        )))
    }

    pub fn digest_prefix(prefix: impl AsRef<[u8]>) -> Self {
        Pattern::Structure(StructurePattern::Digest(DigestPattern::prefix(
            prefix,
        )))
    }

    pub fn digest_binary_regex(regex: regex::bytes::Regex) -> Self {
        Pattern::Structure(StructurePattern::Digest(
            DigestPattern::binary_regex(regex),
        ))
    }

    pub fn any_node() -> Self {
        Pattern::Structure(StructurePattern::Node(NodePattern::any()))
    }

    pub fn node_with_assertions_range(range: impl RangeBounds<usize>) -> Self {
        Pattern::Structure(StructurePattern::Node(NodePattern::interval(range)))
    }

    pub fn node_with_assertions_count(count: usize) -> Self {
        Pattern::Structure(StructurePattern::Node(NodePattern::interval(
            count..=count,
        )))
    }

    pub fn obscured() -> Self {
        Pattern::Structure(StructurePattern::Obscured(ObscuredPattern::any()))
    }

    pub fn elided() -> Self {
        Pattern::Structure(
            StructurePattern::Obscured(ObscuredPattern::elided()),
        )
    }

    pub fn encrypted() -> Self {
        Pattern::Structure(StructurePattern::Obscured(
            ObscuredPattern::encrypted(),
        ))
    }

    pub fn compressed() -> Self {
        Pattern::Structure(StructurePattern::Obscured(
            ObscuredPattern::compressed(),
        ))
    }
}

//
//
// endregion

// region: Meta Patterns
//
//

impl Pattern {
    /// Creates a new `Pattern` that matches any element.
    pub fn any() -> Self { Pattern::Meta(MetaPattern::Any(AnyPattern::new())) }
}

impl Pattern {
    /// Creates a new `Pattern` that only matches if all specified patterns
    /// match.
    pub fn and(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::And(AndPattern::new(patterns)))
    }

    /// Creates a new `Pattern` that matches if at least one of the specified
    /// patterns matches.
    pub fn or(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::Or(OrPattern::new(patterns)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches a traversal order of patterns.
    pub fn traverse(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::Traverse(TraversePattern::new(patterns)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that searches for a specific pattern within the
    /// envelope. Useful for finding patterns that may not be at the root
    /// of the envelope.
    pub fn search(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Search(SearchPattern::new(pattern)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that negates another pattern; matches if the
    /// specified pattern does not match.
    pub fn not_matching(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Not(NotPattern::new(pattern)))
    }
}

impl Pattern {
    /// Compile self to byte-code (recursive).
    pub(crate) fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        use Pattern::*;
        match self {
            Leaf(leaf_pattern) => leaf_pattern.compile(code, lits, captures),
            Structure(struct_pattern) => {
                struct_pattern.compile(code, lits, captures)
            }
            Meta(meta_pattern) => meta_pattern.compile(code, lits, captures),
        }
    }
}

impl Pattern {
    /// Creates a new `Pattern` that will match a pattern repeated a number of
    /// times according to the specified range and greediness.
    ///
    /// In regex terms:
    ///
    /// | Range         | Quantifier   |
    /// | :------------ | :----------- |
    /// | `..`          | `*`          |
    /// | `1..`         | `+`          |
    /// | `0..=1`       | `?`          |
    /// | `min..=max`   | `{min,max}`  |
    /// | `min..`       | `{min,}`     |
    /// | `..=max`      | `{0,max}`    |
    /// | `n..=n`       | `{n}`        |
    pub fn repeat(
        pattern: Pattern,
        interval: impl RangeBounds<usize>,
        reluctance: Reluctance,
    ) -> Self {
        Pattern::Meta(MetaPattern::Group(GroupPattern::repeat(
            pattern,
            Quantifier::new(interval, reluctance),
        )))
    }

    pub fn group(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Group(GroupPattern::new(pattern)))
    }
}

impl Pattern {
    /// Creates a new `Pattern` that will capture a pattern match with a name.
    pub fn capture(name: impl AsRef<str>, pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Capture(CapturePattern::new(name, pattern)))
    }
}

//
//
// endregion

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Leaf(leaf) => write!(f, "{}", leaf),
            Pattern::Structure(structure) => write!(f, "{}", structure),
            Pattern::Meta(meta) => write!(f, "{}", meta),
        }
    }
}

impl Pattern {
    /// Internal helper that runs the pattern through the VM and returns the
    /// matching paths.
    fn vm_run(
        &self,
        env: &Envelope,
    ) -> Vec<(Path, HashMap<String, Vec<Path>>)> {
        thread_local! {
            static PROG: RefCell<HashMap<u64, vm::Program>> = RefCell::new(HashMap::new());
        }

        // cheap structural hash
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        let key = h.finish();

        let prog = PROG
            .with(|cell| cell.borrow().get(&key).cloned())
            .unwrap_or_else(|| {
                let mut p = vm::Program {
                    code: Vec::new(),
                    literals: Vec::new(),
                    capture_names: Vec::new(),
                };
                self.compile(
                    &mut p.code,
                    &mut p.literals,
                    &mut p.capture_names,
                );
                p.code.push(Instr::Accept);
                PROG.with(|cell| {
                    cell.borrow_mut().insert(key, p.clone());
                });
                p
            });

        vm::run(&prog, env)
    }

    #[allow(dead_code)]
    fn vm_paths(&self, env: &Envelope) -> Vec<Path> {
        self.vm_run(env).into_iter().map(|(p, _)| p).collect()
    }

    pub(crate) fn collect_capture_names(&self, out: &mut Vec<String>) {
        if let Pattern::Meta(meta) = self {
            meta.collect_capture_names(out)
        }
    }
}

impl Pattern {
    /// Creates a new `Pattern` that matches any wrapped envelope without
    /// descending. Renamed from `wrapped()` to break tests so they can be
    /// fixed.
    pub fn wrapped() -> Self {
        Pattern::Structure(StructurePattern::Wrapped(WrappedPattern::new()))
    }

    /// Creates a new `Pattern` that matches a wrapped envelope and also matches
    /// on its unwrapped content.
    pub fn unwrap_matching(pattern: Pattern) -> Self {
        Pattern::Structure(StructurePattern::Wrapped(
            WrappedPattern::unwrap_matching(pattern),
        ))
    }

    /// Creates a new `Pattern` that matches any wrapped envelope and descends
    /// into it.
    pub fn unwrap() -> Self {
        Pattern::Structure(StructurePattern::Wrapped(WrappedPattern::unwrap()))
    }
}
