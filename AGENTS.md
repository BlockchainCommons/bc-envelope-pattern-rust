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

## Current Status: `dcbor-pattern` Integration

Basic integration of `dcbor-pattern` into `bc-envelope-pattern` has been successfully completed. This crate now delegates all CBOR leaf pattern matching to the mature `dcbor-pattern` crate while maintaining full backwards compatibility and envelope-specific functionality.

### Integration Summary:

Core integration work is complete and tested:
- **Full `CBOR(/pattern/)` syntax support** using dcbor-pattern expressions (e.g., `CBOR(/NUMBER/)`, `CBOR(/TEXT/)`, `CBOR(/ARRAY/)`)
- **Robust pattern validation** that correctly rejects invalid syntax (e.g., `uint`) and accepts valid patterns
- **Complete backwards compatibility** - all existing APIs work unchanged
- **Comprehensive test coverage** including integration tests and error handling validation
- **Seamless error handling** with proper conversion from dcbor-pattern errors

### Phase 2: Enhanced Path and Capture Integration - COMPLETED ✅

**✅ Phase 2 integration is now complete!** bc-envelope-pattern now provides extended path details that show internal CBOR structure when dcbor-pattern matches within CBOR leaves.

#### Completed Features:

- **✅ Extended path details**: When dcbor-pattern matches within CBOR leaves, paths are extended to show internal CBOR structure elements as individual Envelope path components
- **✅ Proper path conversion**: Uses `.into()` to convert `CBOR` objects to Envelope leaves and handles path extension correctly
- **✅ Multiple path results**: The VM now properly spawns threads for each path returned by atomic patterns, enabling multiple path results for CBOR patterns
- **✅ Comprehensive test coverage**: All tests now use `format_paths()` and `assert_actual_expected!` with `indoc!` for multiline expected strings, following the established rubric
- **✅ Edge case handling**: Covers arrays, nested structures, single values, text searches, no matches, order preservation, complex nesting, and map key/value traversal
- **✅ Code cleanup**: Removed debug output and ensured all test files follow the established rubric with proper formatting and assertions
- **✅ All tests passing**: 267 tests pass (including 73 unit tests, 194 integration tests) with 3 ignored tests for future work

#### Test Files Updated and Verified:
- `tests/test_cbor_path_extension.rs` - Complete CBOR path extension test coverage
- `tests/test_cbor_paths_formatted.rs` - Path formatting verification tests
- `tests/test_dcbor_paths.rs` - Basic dcbor-pattern integration tests
- `tests/test_extended_paths.rs` - Extended path functionality tests
- `tests/common/mod.rs` - Fixed `assert_actual_expected!` macro

#### Technical Implementation:

- **CBORPattern path extension**: `CBORPattern::paths_with_captures` properly converts dcbor-pattern paths (Vec<CBOR>) into Envelope paths, handling root skipping and correct path extension
- **VM thread spawning**: Updated `MatchPredicate` instruction to spawn threads for each path returned by `atomic_paths`, replacing single-path compilation
- **Test framework compliance**: All relevant tests use the established rubric with `bc_envelope_pattern::format_paths()`, `assert_actual_expected!`, and `indoc!` for output validation
- **Macro improvements**: Fixed the `assert_actual_expected!` macro to properly handle format arguments

The integration now provides seamless access to both envelope-level structure and internal CBOR pattern matches, maintaining full backwards compatibility while significantly enhancing pattern matching capabilities.

- **CBORPattern path extension**: `CBORPattern::paths_with_captures` properly converts dcbor-pattern paths (Vec<CBOR>) into Envelope paths, handling root skipping and correct path extension
- **VM thread spawning**: Updated `MatchPredicate` instruction to spawn threads for each path returned by `atomic_paths`, replacing single-path compilation
- **Test framework compliance**: All relevant tests use the established rubric with `bc_envelope_pattern::format_paths()`, `assert_actual_expected!`, and `indoc!` for output validation

The integration now provides seamless access to both envelope-level structure and internal CBOR pattern matches, maintaining full backwards compatibility while significantly enhancing pattern matching capabilities.

## Phase 3: Complete capture implementation - IN PROGRESS

<Development Plan goes here>

## Phase 4: dcbor-pattern Capture Integration - PLANNED

### Investigation Complete: Implementation Required ⚠️

**Status**: dcbor-pattern captures are NOT currently integrated with bc-envelope-pattern captures. Investigation reveals the infrastructure exists but integration is incomplete.

#### Current Implementation Gap

The `CBORPattern::paths_with_captures` method currently:
- Correctly calls `dcbor_pattern.paths_with_captures(&cbor)`
- Uses the returned paths but **ignores the dcbor captures** (using `_dcbor_captures`)
- Returns an empty HashMap for envelope captures: `std::collections::HashMap::new()`
- Has TODO comments: `// TODO: Convert dcbor captures in future phase`

#### dcbor-pattern Capture Infrastructure (Available)

dcbor-pattern provides full capture support:
- `CapturePattern` with `@name(pattern)` syntax
- `paths_with_captures()` returns `(Vec<Path>, HashMap<String, Vec<Path>>)`
- Captures work with search patterns, nested patterns, and complex compositions
- Well-tested implementation with proper capture collection

#### Required Integration Work

**Core Implementation**: Update `CBORPattern::paths_with_captures` to:

1. **Convert dcbor capture paths to envelope paths**:
   ```rust
   let (dcbor_paths, dcbor_captures) = dcbor_pattern.paths_with_captures(&subject_cbor);

   let mut envelope_captures = HashMap::new();
   for (capture_name, dcbor_capture_paths) in dcbor_captures {
       let envelope_capture_paths: Vec<Path> = dcbor_capture_paths
           .into_iter()
           .map(|dcbor_path| convert_dcbor_path_to_envelope_path(dcbor_path, base_envelope))
           .collect();
       envelope_captures.insert(capture_name, envelope_capture_paths);
   }
   ```

2. **Ensure unique capture names**: Verify dcbor capture names don't conflict with envelope capture names in the same pattern

3. **Handle both KnownValue and CBOR leaf cases**: Apply the same capture conversion logic to both code paths

#### Benefits of Integration

- **Unified capture system**: `@name(pattern)` syntax works seamlessly in CBOR patterns
- **Enhanced expressiveness**: Complex capture patterns like `CBOR(/@user(SEARCH(@score(NUMBER(>90))))/)` become possible
- **Consistent API**: All capture patterns return envelope paths, maintaining API consistency
- **No breaking changes**: Existing code continues to work; captures are purely additive

#### Testing Requirements

- Test dcbor captures with simple patterns: `CBOR(/@num(NUMBER(42))/)`
- Test dcbor captures with search patterns: `CBOR(/@values(SEARCH(NUMBER))/)`
- Test dcbor captures with nested structures and multiple captures
- Test capture name uniqueness and proper conflict detection
- Test integration with envelope-level captures in composite patterns

### Development Guidelines for Contributors

When working on this crate:

1. **Maintain backwards compatibility**: All existing APIs must continue to work
2. **Test thoroughly**: Run `cargo test` and `cargo clippy` before submitting changes
3. **Follow established patterns**: New patterns should use the dcbor-pattern integration approach
4. **Document changes**: Update both code documentation and this AGENTS.md file as needed
5. **Validate dcbor-pattern syntax**: Use only valid dcbor-pattern expressions (e.g., `NUMBER`, not `uint`)

### Architecture Notes

The current architecture successfully separates concerns:
- **`bc-envelope-pattern`**: Handles envelope structure, meta-patterns, and high-level pattern logic
- **`dcbor-pattern`**: Handles CBOR value matching within envelope leaves
- **Integration layer**: Wrapper types that delegate CBOR matching while maintaining envelope APIs

This design provides the best of both worlds: the mature, well-tested CBOR pattern matching from `dcbor-pattern` with the envelope-specific functionality that makes this crate valuable.

### Tips

- **CBOR/Envelope Isomorphism**: `CBOR` objects can be converted to Envelope leaves using `.into()`. Envelope leaves can be converted to `CBOR` objects using `.as_leaf()`.
- **Easy Creation of CBOR Objects**: For tests, use `dcbor_parse::parse_dcbor_item()` to create `CBOR` objects from diagnostic notation strings. Prefer this over programmatically constructing `CBOR` objects.
- **`dcbor-pattern` Pattern Expression (patex) Syntax**: Documented here: [../dcbor-pattern/docs/PatternSyntax.md](../dcbor-pattern/docs/PatternSyntax.md)
- **`bc-envelope-pattern` Pattern Expression (patex) Syntax**: Documented here: [docs/PatternSyntax.md](docs/PatternSyntax.md)
