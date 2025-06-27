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

## Current Task: Integration of `dcbor-pattern`.

- `dcbor-pattern` is a crate that provides a pattern matcher for dCBOR values.
- The goal is to integrate `dcbor-pattern` into `bc-envelope-pattern` so that it takes over the pattern matching for envelope leaves, which are dCBOR values.

### Already complete:

- This crate now uses `dcbor-pattern`'s implementations of `Interval`, `Reluctance`, and `Quantifier`.
- This crate now has a new error type, `DcborPatternError`, and implements automatic conversion from `dcbor_pattern::Error`.
- **Phase 1.1 - TextPattern Integration**: Successfully integrated `TextPattern` as a wrapper around `dcbor_pattern::TextPattern`
  - Replaced enum-based implementation with struct wrapper
  - Implemented delegation to dcbor-pattern for CBOR matching using `as_leaf()` method
  - Maintained existing API surface for backwards compatibility
  - Added comprehensive test coverage for dcbor-pattern integration
  - All existing tests continue to pass
- **Phase 1.1 - BoolPattern Integration**: Successfully integrated `BoolPattern` as a wrapper around `dcbor_pattern::BoolPattern`
  - Replaced enum-based implementation with struct wrapper
  - Implemented delegation to dcbor-pattern for CBOR matching using `as_leaf()` method
  - Maintained existing API surface for backwards compatibility (any, value)
  - Added comprehensive test coverage for dcbor-pattern integration
  - All existing tests continue to pass

## Development Plan: Deep Integration of `dcbor-pattern`

This section outlines the comprehensive plan for integrating `dcbor-pattern` functionality into `bc-envelope-pattern`. The integration involves replacing the current leaf pattern implementations with delegated calls to `dcbor-pattern`, while maintaining the envelope-specific structure and meta-pattern functionality.

### Phase 1: Core Pattern Type Integration

#### 1.1 Replace Individual Leaf Pattern Types - IN PROGRESS
**Current State**: `bc-envelope-pattern` has its own implementations for leaf patterns like `NumberPattern`, `TextPattern`, `BoolPattern`, etc.

**Goal**: Replace these with wrapper types that delegate to `dcbor-pattern` equivalents.

**Implementation Strategy**:
- Modify `src/pattern/leaf/` modules to wrap `dcbor_pattern::Pattern` types
- Replace individual pattern implementations with delegation to `dcbor-pattern`
- Maintain existing API surface for backwards compatibility
- **Maintain separation of concerns**: Let dcbor-pattern handle CBOR value matching, bc-envelope-pattern handle envelope structure matching
- **No VM changes needed**: Integration happens at the `Matcher::paths_with_captures()` level, not at the VM level

**Progress**:
- ✅ **COMPLETED**: `src/pattern/leaf/text_pattern.rs` - Successfully converted to wrapper around `dcbor_pattern::TextPattern`
  - Changed from enum-based to struct-based wrapper implementation
  - Implemented proper delegation using `envelope.subject().as_leaf()`
  - Added comprehensive test coverage including dcbor-pattern integration tests
  - All existing functionality preserved and tests pass

- ✅ **COMPLETED**: `src/pattern/leaf/number_pattern.rs` - Successfully converted to wrapper around `dcbor_pattern::NumberPattern`
  - Changed from enum-based to struct-based wrapper implementation
  - Implemented proper delegation using `envelope.subject().as_leaf()`
  - Added comprehensive test coverage including dcbor-pattern integration tests
  - All existing functionality preserved and tests pass
  - Maintained all existing API methods (any, exact, range, greater_than, greater_than_or_equal, less_than, less_than_or_equal, nan)

- ✅ **COMPLETED**: `src/pattern/leaf/bool_pattern.rs` - Successfully converted to wrapper around `dcbor_pattern::BoolPattern`
  - Changed from enum-based to struct-based wrapper implementation
  - Implemented proper delegation using `envelope.subject().as_leaf()`
  - Added comprehensive test coverage including dcbor-pattern integration tests
  - All existing functionality preserved and tests pass
  - Maintained all existing API methods (any, value)

- ✅ **COMPLETED**: `src/pattern/leaf/byte_string_pattern.rs` - Successfully converted to wrapper around `dcbor_pattern::ByteStringPattern`
  - Changed from enum-based to struct-based wrapper implementation
  - Implemented proper delegation using `envelope.subject().as_leaf()`
  - Added comprehensive test coverage including dcbor-pattern integration tests
  - All existing functionality preserved and tests pass
  - Maintained all existing API methods (any, value, regex)

- ✅ **COMPLETED**: `src/pattern/leaf/date_pattern.rs` - Successfully converted to wrapper around `dcbor_pattern::DatePattern`
  - Changed from enum-based to struct-based wrapper implementation
  - Implemented proper delegation using `envelope.subject().as_leaf()`
  - Added comprehensive test coverage including dcbor-pattern integration tests
  - All existing functionality preserved and tests pass
  - Maintained all existing API methods (any, value, range, earliest, latest, iso8601, regex)

**Files to Modify**:
- ✅ `src/pattern/leaf/text_pattern.rs` - wrap `dcbor_pattern::TextPattern` - **COMPLETED**
- ✅ `src/pattern/leaf/number_pattern.rs` - wrap `dcbor_pattern::NumberPattern` - **COMPLETED**
- ✅ `src/pattern/leaf/bool_pattern.rs` - wrap `dcbor_pattern::BoolPattern` - **COMPLETED**
- ✅ `src/pattern/leaf/byte_string_pattern.rs` - wrap `dcbor_pattern::ByteStringPattern` - **COMPLETED**
- ✅ `src/pattern/leaf/date_pattern.rs` - wrap `dcbor_pattern::DatePattern` - **COMPLETED**
- ⏳ `src/pattern/leaf/null_pattern.rs` - wrap `dcbor_pattern::NullPattern` - **NEXT**
- ⏳ `src/pattern/leaf/null_pattern.rs` - wrap `dcbor_pattern::NullPattern`
- ⏳ `src/pattern/leaf/known_value_pattern.rs` - wrap `dcbor_pattern::KnownValuePattern`

- Checkpoint: Have we successfully replaced all individual leaf pattern types with wrappers around `dcbor-pattern`? If so, we can proceed to the next phase.

#### 1.2 Leverage Existing as_leaf() Method
**Goal**: Use the existing `as_leaf()` method from `bc-envelope` for envelope-to-CBOR conversion.

**Current State**: The `as_leaf()` method in `bc-envelope/src/base/queries.rs` already provides clean, single-step conversion from envelope to CBOR: `pub fn as_leaf(&self) -> Option<CBOR>`. This is already being used in `cbor_pattern.rs`.

**Implementation**:
- Use existing `envelope.subject().as_leaf()` pattern throughout leaf patterns
- Handle special cases like `KnownValue` envelopes (already handled in current code)
- No additional conversion layer needed - the existing API is sufficient

**Files to Modify**:
- Update existing leaf pattern implementations to consistently use `as_leaf()`

#### 1.3 Update CBORPattern for Generic CBOR Matching
**Goal**: Replace the custom `CBORPattern` with a wrapper around `dcbor-pattern`'s generic CBOR pattern matching.

**Implementation**:
- Remove custom CBOR matching logic
- Delegate to `dcbor_pattern::Pattern` for CBOR value patterns
- Handle envelope-specific cases (KnownValue, etc.)

### Phase 2: Path and Capture Integration via CBOR-to-Envelope Conversion

#### 2.1 Unified Path Representation Strategy
**Current State**:
- `bc-envelope-pattern::Path` is `Vec<Envelope>`
- `dcbor-pattern::Path` is `Vec<CBOR>`
- Need to bridge these two path types for composite matching

**Goal**: Use CBOR-to-Envelope conversion to create unified envelope paths that include both envelope context and CBOR match details.

**Implementation Strategy**:
- **Leverage `.into()` conversion**: CBOR implements `EnvelopeEncodable`, so `cbor.into()` creates `Envelope::Leaf` containing the CBOR value
- **Extend envelope paths**: When dcbor-pattern matches within a leaf, convert `dcbor_pattern::Path` elements to envelopes and extend the envelope path
- **Composite paths**: Result shows envelope context (with hash prefixes) followed by nested CBOR elements as envelope leaves
- **Example**: Envelope path `[envelope_root]` + dcbor path `[cbor1, cbor2]` becomes `[envelope_root, envelope_leaf1, envelope_leaf2]`

**Benefits**:
- Single path type (`Vec<Envelope>`) for all results
- Preserves envelope formatting with hash prefixes
- Shows detailed CBOR structure within matched leaves
- Maintains existing API compatibility

#### 2.2 Unified Named Capture System
**Current State**:
- `bc-envelope-pattern` captures: `HashMap<String, Vec<Vec<Envelope>>>`
- `dcbor-pattern` captures: `HashMap<String, Vec<Vec<CBOR>>>`

**Goal**: Create unified capture system that handles both envelope-level and CBOR-level captures.

**Implementation Strategy**:
- **Convert CBOR captures to envelope captures**: Use `.into()` to convert each CBOR in dcbor-pattern captures to envelope leaves
- **Check for name conflicts**: Ensure no duplicate capture names between envelope and CBOR patterns
- **Merge capture maps**: Combine envelope and CBOR captures into single `HashMap<String, Vec<Vec<Envelope>>>`
- **Simple approach**: Return error if capture names conflict, otherwise merge seamlessly

**Benefits**:
- Single capture type for all results
- No namespace pollution needed
- Consistent formatting across all captures
- Straightforward conflict detection

#### 2.3 Implementation Details

**Files to Modify**:
- `src/pattern/leaf/*.rs` - update leaf pattern implementations to:
  1. Call dcbor-pattern matchers on `envelope.subject().as_leaf()`
  2. Convert resulting `dcbor_pattern::Path`s to envelope paths via `.into()`
  3. Extend base envelope path with converted CBOR elements
  4. Merge capture maps with conflict detection

**Conversion Pattern**:
```rust
// In leaf pattern implementations:
if let Some(cbor) = envelope.subject().as_leaf() {
    let (dcbor_paths, dcbor_captures) = dcbor_pattern.paths_with_captures(&cbor);

    // Convert CBOR paths to envelope extensions
    let envelope_paths: Vec<Path> = dcbor_paths.into_iter().map(|dcbor_path| {
        let mut envelope_path = vec![envelope.clone()];  // Base envelope context
        envelope_path.extend(dcbor_path.into_iter().map(|cbor| cbor.into())); // Convert CBOR to Envelope
        envelope_path
    }).collect();

    // Convert CBOR captures to envelope captures
    let envelope_captures: HashMap<String, Vec<Path>> = dcbor_captures.into_iter().map(|(name, dcbor_capture_paths)| {
        let envelope_capture_paths = dcbor_capture_paths.into_iter().map(|dcbor_path| {
            dcbor_path.into_iter().map(|cbor| cbor.into()).collect()
        }).collect();
        (name, envelope_capture_paths)
    }).collect();

    (envelope_paths, envelope_captures)
}
```

This approach elegantly solves both the composite path and unified capture challenges by leveraging the existing CBOR-to-Envelope conversion infrastructure.
- Maintain envelope-specific formatting for envelope paths
- Support mixed capture display showing both envelope and CBOR captures appropriately

**Files to Modify**:
- `src/format.rs` - integrate dcbor-pattern formatting capabilities
- Update public API to expose enhanced formatting options

### Phase 3: Parser Integration

#### 3.1 Extend Pattern Expression Syntax
**Goal**: Support dcbor-pattern syntax within envelope patterns.

**Current State**: `docs/PatternSyntax.md` defines envelope-specific pattern syntax.

**Implementation Strategy**:
- Extend parser to recognize dcbor-pattern expressions for leaf patterns
- Allow seamless mixing of envelope structure patterns with dcbor leaf patterns
- Update documentation to reflect new capabilities

**Files to Modify**:
- `src/parse/` modules - extend parser to handle dcbor-pattern syntax
- `docs/PatternSyntax.md` - document new syntax capabilities

#### 3.2 Integrate dcbor-pattern's Parser
**Goal**: Use dcbor-pattern's parser for leaf pattern parsing.

**Implementation**:
- Delegate leaf pattern parsing to `dcbor_pattern::Pattern::parse()`
- Handle parser errors and convert them to envelope pattern errors
- Ensure proper error reporting with spans

### Phase 4: Testing and Validation

#### 4.1 Test Suite Migration
**Goal**: Ensure all existing functionality continues to work while gaining new capabilities.

**Implementation Strategy**:
- Run existing test suite to ensure no regressions
- Add tests for new dcbor-pattern integration features
- Test performance compared to current implementation

**Files to Modify**:
- All test files in `tests/` - update as needed for new implementations
- Add integration tests for dcbor-pattern functionality

#### 4.2 Performance Validation
**Goal**: Ensure integration doesn't negatively impact performance.

**Implementation**:
- Benchmark existing functionality before and after integration
- Optimize hot paths if needed
- Consider caching strategies for envelope-to-CBOR conversion

### Phase 5: Documentation and Examples

#### 5.1 Update Documentation
**Goal**: Document new capabilities and updated APIs.

**Files to Update**:
- `README.md` - mention dcbor-pattern integration
- `docs/PatternSyntax.md` - document new syntax capabilities
- Add examples showing dcbor-pattern features in envelope context

#### 5.2 Create Integration Examples
**Goal**: Show how envelope patterns and dcbor patterns work together.

**Implementation**:
- Create examples showing complex leaf pattern matching
- Demonstrate capture functionality with both envelope and CBOR patterns
- Show performance benefits of integrated approach

### Implementation Notes

#### Backwards Compatibility
- Maintain existing API surface where possible
- Use deprecation warnings for APIs that will be removed
- Provide migration guide for users of current API

#### Error Handling
- Ensure proper error conversion from dcbor-pattern errors
- Maintain meaningful error messages with proper spans
- Handle edge cases in envelope-to-CBOR conversion

#### Performance Considerations
- Leverage the efficient `as_leaf()` method for envelope-to-CBOR conversion
- No additional conversion overhead needed beyond existing API
- Profile hot paths and optimize as needed

#### Code Quality
- Follow existing code style and patterns
- Use 4-space indentation for consistency with dcbor-pattern
- Ensure all new code passes clippy and rustfmt checks

### Success Criteria

1. **Functional**: All existing tests pass with new implementation
2. **Enhanced**: New dcbor-pattern features available in envelope context
3. **Performance**: No significant performance regression
4. **Maintainable**: Code is clean, well-documented, and follows project conventions
5. **Compatible**: Existing APIs continue to work with minimal changes

This integration will significantly enhance the pattern matching capabilities of `bc-envelope-pattern` by leveraging the mature, well-tested functionality of `dcbor-pattern` while maintaining the envelope-specific features that make this crate unique.
