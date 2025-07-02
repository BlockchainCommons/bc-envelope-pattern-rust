//! # DCBor Pattern Integration
//!
//! This module provides integration between dcbor-pattern and
//! bc-envelope-pattern, allowing dcbor patterns to be used as envelope patterns
//! through conversion.

use crate::{
    DCBORPattern, Pattern, Result,
    pattern::{
        leaf::{
            ArrayPattern, BoolPattern, ByteStringPattern, CBORPattern,
            DatePattern, KnownValuePattern, LeafPattern, MapPattern,
            NullPattern, NumberPattern, TaggedPattern, TextPattern,
        },
        meta::{AnyPattern, MetaPattern},
    },
};

/// Convert a dcbor-pattern Pattern to a bc-envelope-pattern Pattern.
///
/// This function serves as the bridge between the two pattern systems,
/// allowing dcbor-pattern expressions to be used in envelope pattern contexts.
///
/// # Arguments
///
/// * `dcbor_pattern` - The dcbor-pattern Pattern to convert
///
/// # Returns
///
/// * `Result<Pattern>` - The converted envelope pattern, or an error if
///   conversion fails
///
/// # Examples
///
/// ```rust
/// use dcbor_pattern as dp;
/// use bc_envelope_pattern::dcbor_integration::convert_dcbor_pattern_to_envelope_pattern;
///
/// // Convert a simple boolean pattern
/// let dcbor_bool = dp::Pattern::bool(true);
/// let envelope_pattern = convert_dcbor_pattern_to_envelope_pattern(dcbor_bool).unwrap();
/// ```
pub fn convert_dcbor_pattern_to_envelope_pattern(
    dcbor_pattern: DCBORPattern,
) -> Result<Pattern> {
    match dcbor_pattern {
        DCBORPattern::Value(value_pattern) => {
            convert_value_pattern_to_envelope_pattern(value_pattern)
        }
        DCBORPattern::Structure(structure_pattern) => {
            convert_structure_pattern_to_envelope_pattern(structure_pattern)
        }
        DCBORPattern::Meta(meta_pattern) => {
            convert_meta_pattern_to_envelope_pattern(meta_pattern)
        }
    }
}

/// Convert a dcbor-pattern ValuePattern to an envelope leaf pattern.
fn convert_value_pattern_to_envelope_pattern(
    value_pattern: dcbor_pattern::ValuePattern,
) -> Result<Pattern> {
    let leaf_pattern = match value_pattern {
        dcbor_pattern::ValuePattern::Bool(bool_pattern) => {
            LeafPattern::Bool(BoolPattern::from_dcbor_pattern(bool_pattern))
        }
        dcbor_pattern::ValuePattern::Number(number_pattern) => {
            LeafPattern::Number(NumberPattern::from_dcbor_pattern(
                number_pattern,
            ))
        }
        dcbor_pattern::ValuePattern::Text(text_pattern) => {
            LeafPattern::Text(TextPattern::from_dcbor_pattern(text_pattern))
        }
        dcbor_pattern::ValuePattern::ByteString(bytestring_pattern) => {
            LeafPattern::ByteString(ByteStringPattern::from_dcbor_pattern(
                bytestring_pattern,
            ))
        }
        dcbor_pattern::ValuePattern::Date(date_pattern) => {
            LeafPattern::Date(DatePattern::from_dcbor_pattern(date_pattern))
        }
        dcbor_pattern::ValuePattern::KnownValue(known_value_pattern) => {
            LeafPattern::KnownValue(KnownValuePattern::from_dcbor_pattern(
                known_value_pattern,
            ))
        }
        dcbor_pattern::ValuePattern::Null(_) => LeafPattern::Null(NullPattern),
        dcbor_pattern::ValuePattern::Digest(digest_pattern) => {
            // Digest patterns don't have a direct envelope equivalent yet
            // For now, wrap as a generic CBOR pattern
            return Ok(Pattern::Leaf(LeafPattern::Cbor(
                CBORPattern::from_dcbor_pattern(DCBORPattern::Value(
                    dcbor_pattern::ValuePattern::Digest(digest_pattern),
                )),
            )));
        }
    };

    Ok(Pattern::Leaf(leaf_pattern))
}

/// Convert a dcbor-pattern StructurePattern to an envelope pattern.
fn convert_structure_pattern_to_envelope_pattern(
    structure_pattern: dcbor_pattern::StructurePattern,
) -> Result<Pattern> {
    let leaf_pattern = match structure_pattern {
        dcbor_pattern::StructurePattern::Array(array_pattern) => {
            LeafPattern::Array(ArrayPattern::from_dcbor_array_pattern(
                array_pattern,
            ))
        }
        dcbor_pattern::StructurePattern::Map(map_pattern) => {
            LeafPattern::Map(MapPattern::from_dcbor_pattern(map_pattern))
        }
        dcbor_pattern::StructurePattern::Tagged(tagged_pattern) => {
            LeafPattern::Tag(TaggedPattern::from_dcbor_pattern(tagged_pattern))
        }
    };

    Ok(Pattern::Leaf(leaf_pattern))
}

/// Convert a dcbor-pattern MetaPattern to an envelope meta pattern.
fn convert_meta_pattern_to_envelope_pattern(
    meta_pattern: dcbor_pattern::MetaPattern,
) -> Result<Pattern> {
    let meta_pattern_clone = meta_pattern.clone();
    match meta_pattern {
        dcbor_pattern::MetaPattern::Any(_) => {
            // The dcbor "any" pattern corresponds to our "any" meta pattern
            Ok(Pattern::Meta(MetaPattern::Any(AnyPattern::new())))
        }
        dcbor_pattern::MetaPattern::And(and_pattern) => {
            // Convert AND pattern by recursively converting each sub-pattern
            let mut converted_patterns = Vec::new();
            for pattern in and_pattern.patterns() {
                converted_patterns.push(
                    convert_dcbor_pattern_to_envelope_pattern(pattern.clone())?,
                );
            }
            Ok(Pattern::and(converted_patterns))
        }
        dcbor_pattern::MetaPattern::Or(or_pattern) => {
            // Convert OR pattern by recursively converting each sub-pattern
            let mut converted_patterns = Vec::new();
            for pattern in or_pattern.patterns() {
                converted_patterns.push(
                    convert_dcbor_pattern_to_envelope_pattern(pattern.clone())?,
                );
            }
            Ok(Pattern::or(converted_patterns))
        }
        dcbor_pattern::MetaPattern::Not(not_pattern) => {
            // Convert NOT pattern by recursively converting the inner pattern
            let inner_pattern = convert_dcbor_pattern_to_envelope_pattern(
                not_pattern.pattern().clone(),
            )?;
            Ok(Pattern::not_matching(inner_pattern))
        }
        dcbor_pattern::MetaPattern::Capture(_capture_pattern) => {
            // Capture patterns don't have a direct envelope equivalent yet
            // For now, wrap as a generic CBOR pattern
            Ok(Pattern::Leaf(LeafPattern::Cbor(
                CBORPattern::from_dcbor_pattern(DCBORPattern::Meta(
                    meta_pattern_clone,
                )),
            )))
        }
        dcbor_pattern::MetaPattern::Repeat(_repeat_pattern) => {
            // Repeat patterns don't have a direct envelope equivalent
            // For now, wrap as a generic CBOR pattern
            Ok(Pattern::Leaf(LeafPattern::Cbor(
                CBORPattern::from_dcbor_pattern(DCBORPattern::Meta(
                    meta_pattern_clone,
                )),
            )))
        }
        dcbor_pattern::MetaPattern::Search(_search_pattern) => {
            // Search patterns don't have a direct envelope equivalent
            // For now, wrap as a generic CBOR pattern
            Ok(Pattern::Leaf(LeafPattern::Cbor(
                CBORPattern::from_dcbor_pattern(DCBORPattern::Meta(
                    meta_pattern_clone,
                )),
            )))
        }
        dcbor_pattern::MetaPattern::Sequence(_sequence_pattern) => {
            // Sequence patterns don't have a direct envelope equivalent
            // For now, wrap as a generic CBOR pattern
            Ok(Pattern::Leaf(LeafPattern::Cbor(
                CBORPattern::from_dcbor_pattern(DCBORPattern::Meta(
                    meta_pattern_clone,
                )),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use dcbor_pattern as dp;

    use super::*;

    #[test]
    fn test_convert_bool_pattern() {
        let dcbor_bool = dp::Pattern::bool(true);
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_bool).unwrap();

        match envelope_pattern {
            Pattern::Leaf(LeafPattern::Bool(_)) => {
                // Success - converted to boolean pattern
            }
            _ => panic!("Expected boolean leaf pattern"),
        }
    }

    #[test]
    fn test_convert_number_pattern() {
        let dcbor_number = dp::Pattern::number(42);
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_number).unwrap();

        match envelope_pattern {
            Pattern::Leaf(LeafPattern::Number(_)) => {
                // Success - converted to number pattern
            }
            _ => panic!("Expected number leaf pattern"),
        }
    }

    #[test]
    fn test_convert_text_pattern() {
        let dcbor_text = dp::Pattern::text("hello");
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_text).unwrap();

        match envelope_pattern {
            Pattern::Leaf(LeafPattern::Text(_)) => {
                // Success - converted to text pattern
            }
            _ => panic!("Expected text leaf pattern"),
        }
    }

    #[test]
    fn test_convert_any_pattern() {
        let dcbor_any = dp::Pattern::any();
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_any).unwrap();

        match envelope_pattern {
            Pattern::Meta(MetaPattern::Any(_)) => {
                // Success - converted to any meta pattern
            }
            _ => panic!("Expected any meta pattern"),
        }
    }

    #[test]
    fn test_convert_or_pattern() {
        let dcbor_or = dp::Pattern::or(vec![
            dp::Pattern::bool(true),
            dp::Pattern::number(42),
        ]);
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_or).unwrap();

        match envelope_pattern {
            Pattern::Meta(MetaPattern::Or(_)) => {
                // Success - converted to or meta pattern
            }
            _ => panic!("Expected or meta pattern"),
        }
    }

    #[test]
    fn test_convert_and_pattern() {
        let dcbor_and = dp::Pattern::and(vec![
            dp::Pattern::any_bool(),
            dp::Pattern::any_number(),
        ]);
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_and).unwrap();

        match envelope_pattern {
            Pattern::Meta(MetaPattern::And(_)) => {
                // Success - converted to and meta pattern
            }
            _ => panic!("Expected and meta pattern"),
        }
    }

    #[test]
    fn test_convert_not_pattern() {
        let dcbor_not = dp::Pattern::not_matching(dp::Pattern::bool(false));
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_not).unwrap();

        match envelope_pattern {
            Pattern::Meta(MetaPattern::Not(_)) => {
                // Success - converted to not meta pattern
            }
            _ => panic!("Expected not meta pattern"),
        }
    }

    #[test]
    fn test_convert_capture_pattern() {
        // Since capture patterns don't have direct envelope equivalents,
        // they should be wrapped as CBOR patterns
        let dcbor_capture =
            dp::Pattern::capture("test", dp::Pattern::bool(true));
        let envelope_pattern =
            convert_dcbor_pattern_to_envelope_pattern(dcbor_capture).unwrap();

        match envelope_pattern {
            Pattern::Leaf(LeafPattern::Cbor(_)) => {
                // Success - converted to CBOR pattern (fallback)
            }
            _ => panic!("Expected CBOR leaf pattern as fallback"),
        }
    }
}
