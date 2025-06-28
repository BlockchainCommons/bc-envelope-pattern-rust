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

### Development Plan: dcbor-pattern Capture Integration

Based on investigation of how `dcbor-pattern` implements named captures and analysis of the current `bc-envelope-pattern` capture implementation gap, here is the comprehensive plan for Phase 3:

#### Current Status Summary

**✅ CONFIRMED Working Infrastructure:**
- `bc-envelope-pattern` has envelope-level capture support ONLY via the meta `CapturePattern`
- Pattern `@name(NUMBER(42))` works perfectly: matches and captures correctly
- `dcbor-pattern` has complete CBOR-level capture support via `CapturePattern`
- Both use identical `@name(pattern)` syntax and `paths_with_captures()` API
- Both correctly handle nested captures and complex capture scenarios

**❌ CONFIRMED Missing Integration:**
- **ALL base patterns** (leaf, structure) in `bc-envelope-pattern` return empty `HashMap::new()` for captures
- **Only** the meta `CapturePattern` actually collects captures by wrapping other patterns
- `CBORPattern::paths_with_captures()` ignores dcbor captures (marked with `_dcbor_captures`)
- **VERIFIED**: Pattern `CBOR(/@name(NUMBER(42))/)` matches but returns 0 captures
- **VERIFIED**: Pattern `@env(CBOR(/@dcbor(NUMBER(42))/))` captures `env` but ignores `dcbor`
- No conversion from dcbor capture paths to envelope capture paths
- No testing coverage for CBOR pattern captures
- TODO comments indicate incomplete implementation

**Key Discovery**: The capture system in `bc-envelope-pattern` is fundamentally different from `dcbor-pattern`:
- In `dcbor-pattern`: Individual patterns can have captures (e.g., `TextPattern` with regex captures)
- In `bc-envelope-pattern`: ONLY the meta `CapturePattern` handles captures by wrapping other patterns

#### Phase 3 Implementation Tasks

##### Task 3.1: Core Capture Path Conversion (HIGH PRIORITY)

**Objective**: Update `CBORPattern::paths_with_captures` to properly convert dcbor captures to envelope captures.

**Location**: `src/pattern/leaf/cbor_pattern.rs` lines 89 and 142

**Key Insight**: Unlike other leaf patterns that always return `HashMap::new()` for captures, `CBORPattern` needs to be the FIRST base pattern to actually implement capture support by delegating to dcbor-pattern's capture system.

**Changes Required**:

1. **Replace ignored `_dcbor_captures` with proper handling**:
   ```rust
   // Current code:
   let (dcbor_paths, _dcbor_captures) = dcbor_pattern.paths_with_captures(&subject_cbor);

   // New code:
   let (dcbor_paths, dcbor_captures) = dcbor_pattern.paths_with_captures(&subject_cbor);
   ```

2. **Implement dcbor-to-envelope capture path conversion**:
   ```rust
   // Convert dcbor captures to envelope captures
   let mut envelope_captures = std::collections::HashMap::new();
   for (capture_name, dcbor_capture_paths) in dcbor_captures {
       let envelope_capture_paths: Vec<Path> = dcbor_capture_paths
           .into_iter()
           .map(|dcbor_path| convert_dcbor_capture_path_to_envelope_path(dcbor_path, envelope))
           .collect();
       envelope_captures.insert(capture_name, envelope_capture_paths);
   }
   ```

3. **Create helper function for path conversion**:
   ```rust
   fn convert_dcbor_capture_path_to_envelope_path(dcbor_path: Vec<CBOR>, base_envelope: &Envelope) -> Vec<Envelope> {
       let mut envelope_path = vec![base_envelope.clone()];

       // Skip first element if it matches the base envelope's CBOR content
       let skip_first = if let Some(base_cbor) = base_envelope.subject().as_leaf() {
           dcbor_path.first().map(|first| first == &base_cbor).unwrap_or(false)
       } else {
           false
       };

       let elements_to_add = if skip_first {
           dcbor_path.into_iter().skip(1)
       } else {
           dcbor_path.into_iter().skip(0)
       };

       for cbor_element in elements_to_add {
           envelope_path.push(Envelope::new(cbor_element));
       }

       envelope_path
   }
   ```

4. **Apply to both KnownValue and CBOR leaf code paths** (lines 89 and 142)

**Note**: This makes `CBORPattern` unique among bc-envelope-pattern's base patterns as the first to actually return non-empty captures from `paths_with_captures()`.

##### Task 3.2: Comprehensive Test Coverage (HIGH PRIORITY)

**Objective**: Create comprehensive test suite for dcbor captures within CBOR patterns.

**Location**: New file `tests/test_cbor_captures.rs`

**Test Cases Required**:

1. **Simple dcbor captures**:
   ```rust
   // Pattern: CBOR(/@num(NUMBER(42))/)
   // Envelope: 42
   // Expected: captures["num"] = [path to 42]
   ```

2. **Search pattern captures**:
   ```rust
   // Pattern: CBOR(/@values(SEARCH(NUMBER))/)
   // Envelope: [1, 2, 3]
   // Expected: captures["values"] = [paths to 1, 2, 3]
   ```

3. **Nested structure captures**:
   ```rust
   // Pattern: CBOR(/@user(SEARCH(@score(NUMBER(>90))))/)
   // Envelope: {"users": [{"name": "Alice", "score": 95}, {"name": "Bob", "score": 85}]}
   // Expected: captures["user"] includes paths, captures["score"] = [path to 95]
   ```

4. **Multiple captures in single pattern**:
   ```rust
   // Pattern: CBOR(/@name(TEXT) & @age(NUMBER)/)
   // Test capture isolation and proper path assignment
   ```

5. **Complex array/map traversal captures**:
   ```rust
   // Pattern: CBOR(/ARRAY(@item(TEXT) > @item(NUMBER))/)
   // Test sequence capture behavior
   ```

6. **Capture name conflict detection**:
   ```rust
   // Pattern: @env_capture(CBOR(/@same_name(NUMBER)/))
   // Verify no conflicts between envelope and dcbor capture names
   ```

##### Task 3.3: Integration Testing (MEDIUM PRIORITY)

**Objective**: Ensure dcbor captures work seamlessly with existing envelope capture patterns.

**Test Scenarios**:

1. **Mixed envelope + dcbor captures**:
   ```rust
   // Pattern: @wrapper(WRAPPED -> @content(CBOR(/@number(NUMBER)/)))
   // Verify both envelope and dcbor captures work together
   ```

2. **Capture composition**:
   ```rust
   // Pattern: SEARCH(@found(CBOR(/@inner(TEXT)/)))
   // Test nested meta-pattern + dcbor captures
   ```

3. **VM integration**:
   ```rust
   // Verify capture instructions work correctly with CBOR patterns
   // Test CaptureStart/CaptureEnd instruction integration
   ```

##### Task 3.4: Error Handling and Edge Cases (MEDIUM PRIORITY)

**Objective**: Robust error handling for capture edge cases.

**Implementation Required**:

1. **Capture name validation**:
   - Ensure dcbor capture names don't conflict with envelope capture names
   - Provide clear error messages for conflicts

2. **Empty capture handling**:
   - Proper behavior when dcbor patterns match but captures are empty
   - Consistent HashMap behavior for missing captures

3. **Path conversion error handling**:
   - Handle malformed dcbor paths gracefully
   - Validate CBOR-to-Envelope conversion assumptions

##### Task 3.5: Documentation and Examples (LOW PRIORITY)

**Objective**: Document the new integrated capture functionality.

**Documentation Updates**:

1. **Update `AGENTS.md`**: Mark Phase 3 as complete, document new capabilities
2. **Update `docs/PatternSyntax.md`**: Add examples of CBOR pattern captures
3. **Add code examples**: Demonstrate combined envelope + dcbor captures
4. **API documentation**: Document new capture behavior in method docs

#### Implementation Order

1. **Task 3.1** (Core implementation) - Essential foundation
2. **Task 3.2** (Basic testing) - Validate core functionality
3. **Task 3.4** (Error handling) - Ensure robustness
4. **Task 3.3** (Integration testing) - Verify complete system works
5. **Task 3.5** (Documentation) - Complete the feature

#### Success Criteria

- ✅ All existing tests continue to pass (no regressions)
- ✅ `CBOR(/@name(pattern)/)` syntax works correctly
- ✅ dcbor captures integrate seamlessly with envelope captures
- ✅ No capture name conflicts between levels
- ✅ Complex nested capture scenarios work correctly
- ✅ All test patterns from requirements are implemented and pass
- ✅ `cargo test` and `cargo clippy` pass without issues
- ✅ Performance impact is minimal (captures are opt-in)

#### Technical Notes

- **Architectural Difference**: bc-envelope-pattern uses a wrapper-based capture system (only `CapturePattern` captures) while dcbor-pattern allows individual patterns to have captures. CBORPattern bridges this gap by being the first base pattern to delegate capture functionality.
- **Path conversion complexity**: The key challenge is converting dcbor paths (Vec<CBOR>) to envelope paths (Vec<Envelope>) while maintaining correct tree traversal semantics
- **Capture merging**: When both envelope-level `CapturePattern` and CBOR patterns have captures, ensure proper HashMap merging without key conflicts
- **Memory efficiency**: Capture conversion should be lazy/on-demand to avoid unnecessary allocations
- **Thread safety**: Ensure capture collections work correctly in VM multi-threading scenarios

This integration will provide a unified capture system where `@name(pattern)` syntax works seamlessly across both envelope structure patterns (via `CapturePattern` wrapper) and internal CBOR content patterns (via `CBORPattern` delegation), significantly enhancing pattern expressiveness while maintaining full backwards compatibility.

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
