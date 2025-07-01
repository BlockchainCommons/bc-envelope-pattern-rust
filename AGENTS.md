# `bc-envelope-pattern` Crate Documentation

This file contains general information for contributors to the `bc-envelope-pattern` crate, which provides a pattern matcher and text syntax pattern parser for Gordian Envelope. A general description of this crate and its use are in `README.md`. Further documentation including the pattern expression syntax can be found in the `docs/` directory. Make sure to read those before starting on any tasks.

## General Guidance

This crate is now in preview release. You are likely to be asked for bug fixes, enhancements, and documentation improvements. Always make sure that `cargo test` and `cargo clippy` pass before you're done with your changes.

## Intro to Gordian Envelope

- The basic structure of Gordian Envelope is defined in the [Envelope I-D](https://datatracker.ietf.org/doc/draft-mcnally-envelope/).
- envelopes are structured as a tree.
- Leaves of the tree are CBOR values.
- Branches can happen at each NODE (a subject having one or more assertions).
- Each assertion has a predicate and an object.
- Every *part* of an Envelope is *itself* an Envelope, which can have assertions.

## Important Dependencies

- `bc-envelope`: One of the most important dependencies for this crate, `bc-envelope` provides the core functionality for working with Gordian Envelope. In particular, you will want to make sure you are familiar with the `envelope.rs` and `queries.rs` modules.
- `dcbor`: Envelope is built on deterministic CBOR (dCBOR), which is implemented in the `dcbor` crate.
- `dcbor-parse`: This crate provides a parser for dCBOR diagnostic notation, which is used to specify patterns in a human-readable format.
- `dcbor-pattern`: This crate provides a pattern matcher for dCBOR values, which is used to match patterns against the leaves of the Envelope tree.

When working on this crate:

1. **Test thoroughly**: Run `cargo test` and `cargo clippy` before submitting changes
2. **Follow established patterns**: New patterns should use the dcbor-pattern integration approach
3. **Document changes**: Update both code documentation and this AGENTS.md file as needed
4. **Validate dcbor-pattern syntax**: Use only valid dcbor-pattern expressions (e.g., `number`, not `uint`)

### Architecture Notes

The current architecture successfully separates concerns:
- **`bc-envelope-pattern`**: Handles envelope structure, meta-patterns, and high-level pattern logic
- **`dcbor-pattern`**: Handles CBOR value matching within envelope leaves
- **Integration layer**: Wrapper types that delegate CBOR matching while maintaining envelope APIs

This design provides the best of both worlds: the mature, well-tested CBOR pattern matching from `dcbor-pattern` with the envelope-specific functionality that makes this crate valuable.

### Tips

- **CBOR/Envelope Isomorphism**: `CBOR` objects can be converted to Envelope leaves using `.into()`. Envelope leaves can be converted to `CBOR` objects using `.as_leaf()`.
- **Easy Creation of CBOR Objects**: For tests, use `dcbor_parse::parse_dcbor_item()` to create `CBOR` objects from diagnostic notation strings. Prefer this over programmatically constructing `CBOR` objects.
- **`dcbor-pattern` Pattern Expression (patex) Syntax**: Documented here: [../dcbor-pattern/docs/DCBORPatternSyntax.md](../dcbor-pattern/docs/DCBORPatternSyntax.md)
- **`bc-envelope-pattern` Pattern Expression (patex) Syntax**: Documented here: [docs/EnvelopePatternSyntax.md](docs/EnvelopePatternSyntax.md)


## Development Plan: Enhanced Pattern Parsing with `parse_partial()`

### Feature Request: Unified Pattern Parsing with Fallback

**Status**: 📋 PLANNED

**Motivation**:
With the recent introduction of `dcbor_pattern::Pattern::parse_partial()`, we now have the capability to parse any dcbor-pattern expression without consuming the entire input stream. This opens up the possibility of creating a more unified parsing approach that can:

1. **Primary**: Attempt to parse using dcbor-pattern's comprehensive syntax
2. **Fallback**: Fall back to envelope-specific parsing when dcbor-pattern doesn't recognize the pattern
3. **Future-proof**: Automatically adopt new dcbor-pattern features without manual updates

**Current Architecture Benefits**:
- Many leaf patterns (BOOL, TEXT, NUMBER, etc.) already use dcbor-pattern as wrapper types
- Remaining patterns (MAP, etc.) still use custom parsing logic
- The integration layer successfully separates concerns

**Proposed Enhancement**:
Instead of maintaining separate parsers for each remaining pattern type, implement a unified approach in the primary parser that:

```rust
// Proposed approach in primary_parser.rs
pub(crate) fn parse_primary(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // First, try to parse using dcbor-pattern's parse_partial
    let remaining_input = lexer.remainder();

    if let Ok((dcbor_pattern, consumed)) = dcbor_pattern::Pattern::parse_partial(remaining_input) {
        // Success! We can use this dcbor pattern for leaf matching
        lexer.bump(consumed);
        return Ok(convert_dcbor_pattern_to_envelope_pattern(dcbor_pattern));
    }

    // Fallback to existing envelope-specific parsing
    match lexer.next() {
        Some(Ok(Token::Map)) => leaf::parse_map(lexer),
        Some(Ok(Token::Node)) => structure::parse_node(lexer),
        // ... other envelope-specific patterns
        _ => Err(Error::UnrecognizedToken(lexer.span()))
    }
}
```

**Benefits**:
- **Reduced Maintenance**: Less custom parsing code to maintain
- **Automatic Feature Adoption**: New dcbor-pattern syntax automatically available
- **Consistency**: Unified approach across all leaf patterns
- **Graceful Degradation**: Envelope-specific patterns still work via fallback
- **Future-Proof**: Ready for new dcbor-pattern capabilities

**Implementation Plan**:

1. **Phase 1: Infrastructure**
   - Create `convert_dcbor_pattern_to_envelope_pattern()` function
   - Add comprehensive tests for the conversion layer
   - Ensure error messages remain helpful for envelope contexts

2. **Phase 2: Primary Parser Integration**
   - Modify `parse_primary()` to attempt dcbor-pattern parsing first
   - Implement graceful fallback to envelope-specific parsing
   - Maintain backward compatibility for all existing patterns

3. **Phase 3: Pattern-Specific Integration**
   - Convert remaining patterns (like MAP) to use the new approach where beneficial
   - Keep envelope-specific patterns (NODE, ASSERTION, etc.) in fallback
   - Comprehensive testing of the hybrid approach

4. **Phase 4: Optimization and Cleanup**
   - Remove redundant parsing code where dcbor-pattern suffices
   - Optimize the conversion layer for performance
   - Update documentation and examples

**Success Criteria**:
- All existing tests continue to pass
- New dcbor-pattern syntax automatically works in envelope patterns
- Performance remains equivalent or improves
- Error messages remain clear and helpful
- Code complexity decreases overall
