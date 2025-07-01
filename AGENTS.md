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


## Current Status: update patterns.

- Once you have completed the `NEXT TASK`, stop and let me run tests.

### COMPLETE:

- Update the `ANY` pattern to use the `dcbor-pattern` crate's `*` syntax for matching any envelope.
- Update the `BOOL` pattern to use the `dcbor-pattern` crate's `bool`, `true`, and `false` syntax for matching boolean values.
- Update the `NULL` pattern to use the `dcbor-pattern` crate's `null` syntax for matching null values.
- Update the `BSTR` pattern to use the `dcbor-pattern` crate's `bstr` syntax for matching byte strings.
- Update the `TEXT` pattern to use the `dcbor-pattern` crate's `text` syntax for matching text strings.
- Update the `NUMBER` pattern to use the `dcbor-pattern` crate's `number` syntax for matching numbers.
- Update the `DATE` pattern to use the `dcbor-pattern` crate's `date` syntax for matching dates.
- Update the `KNOWN` pattern to use the `dcbor-pattern` crate's `known` syntax for matching known values.
- Update the `TAG` pattern to use the `dcbor-pattern` crate's `tagged` syntax for matching arrays.
- Update the `ARRAY` pattern to use the `dcbor-pattern` crate's new syntax for matching arrays.

### NEXT TASK:

- Write the development plan for updating the `MAP` pattern to use the `dcbor-pattern` crate's new syntax for matching maps.

#### Old Syntax:

- Map
    - `MAP`
        - Matches any map.
    - `MAP ( n )`
        - Matches a map with exactly `n` entries.
    - `MAP ( { n , m } )`
        - Matches a map with between `n` and `m` entries, inclusive.

#### New Syntax:

- Map
    - `{*}`
        - Matches any map.
    - `{{n}}`
        - Matches a map with exactly `n` entries.
    - `{{n,m}}`
        - Matches a map with between `n` and `m` entries, inclusive.
    - `{{n,}}`
        - Matches a map with at least `n` entries.
    - `{pattern: pattern, pattern: pattern, ...}`
        - Matches if the specified patterns match the map's keys and values (order isn't important).

    - Examples:
        - Old: `MAP` becomes `{*}`
        - Old: `MAP( { 1 } )` becomes `{ {1} }`
        - Old: `MAP( {1, 2} )` becomes `{ {1, 2} }`
        - Old: `MAP( { 1 , } )` becomes `{ {1 , } }`
        - Old: `MAP(key: value, ...)` becomes `{key: value, ...}`

#### Important Notes:

    - Remember: The map pattern is just a proxy for the functionality in `dcbor-pattern`, and all you are doing is adapting the existing syntax. You do not need to implement any additional logic for the `map` pattern, just pass from `{` through `}` to `dcbor-pattern`. There will be no envelope patterns inside the `map` pattern, so you do not need to worry about the envelope structure. You are just matching a CBOR value against a pattern, like every other leaf/value pattern. YOU ARE JUST A PROXY.
    - Tip: We've already converted the `array` (`[*]`) pattern to use the new syntax, so you can use that as a reference for how to convert the `map` pattern.
    - Tip: Intervals `{n, m}` are delimited by braces and so are maps. The key to differentiating them from the new map syntax is that intervals cannot appear by themselves. Finding a `{` token where a pattern is expected means you are looking at a map pattern. You need to "naively" parse the map pattern including balanced delimiters and ignoring everything else until you find the closing `}`. Then you need to have `dcbor-pattern` parse the map pattern, which will return a `dcbor-pattern::Pattern`.
    - This is de novo development, so DO NOT take any action to ensure backward-compatibility.
    - REPEAT: REMOVE THE OLD SYNTAX AND REPLACE IT WITH THE NEW SYNTAX.
    - Only put debug examples in `examples/`. Put tests you want to be kept for regression in `tests/`. DO NOT use the root directory or other directories for temporary debug examples.

### Development Plan

Based on the thorough analysis of the existing codebase and the architecture used for the `ARRAY` pattern conversion, here is the comprehensive development plan for updating the `MAP` pattern to use the `dcbor-pattern` crate's new syntax.

- At every step, `cargo test` must be run to ensure that the changes do not break existing functionality.

## Phase 1: Add Support for New Map Syntax Tokens

**Goal**: Extend the tokenizer and parser to recognize the new `{` syntax for maps while maintaining the existing `MAP` syntax temporarily.

### 1.1 Update Token Recognition
- **File**: `src/parse/token.rs`
- **Action**: Add a new token `BraceOpen` for `{` character
- **Details**: The current `{` token is parsed as `Range` (for intervals like `{1,2}`), but we need to distinguish between map patterns (`{*}`, `{key: value}`) and interval patterns (`{1,2}`) contextually
- **Note**: This will require careful handling since `{` is used for both ranges and maps

### 1.2 Update Primary Parser
- **File**: `src/parse/meta/primary_parser.rs`
- **Action**: Add a case for `Token::BraceOpen` that calls a new map parser
- **Details**: Similar to how `Token::BracketOpen` calls `leaf::parse_array(lexer)`

## Phase 2: Implement New Map Parser

**Goal**: Create a new map parsing function that can handle the new dcbor-pattern syntax.

### 2.1 Create Map Parser Utility Function
- **File**: `src/parse/utils.rs`
- **Action**: Add `parse_map_inner(src: &str) -> Result<(Pattern, usize)>` function
- **Pattern**: Follow the same approach as `parse_array_inner()` but for map syntax
- **Logic**:
  1. Check for `*` → return `Pattern::any_map()`
  2. Check for `{n}`, `{n,m}`, `{n,}` patterns → return appropriate range patterns
  3. For everything else → delegate to `dcbor-pattern` by wrapping content in `{...}` and parsing

### 2.2 Update Map Parser
- **File**: `src/parse/leaf/map_parser.rs`
- **Action**: Create a new function `parse_map_new_syntax()` that uses the new brace-based approach
- **Logic**: Parse from `{` through `}` using balanced delimiter counting, then pass to `dcbor-pattern`

### 2.3 Handle Ambiguous `{` Token
- **Challenge**: `{` can be either a range (`{1,2}`) or a map pattern (`{*}`, `{key: value}`)
- **Solution**: Add context-aware parsing in the primary parser:
  - When expecting a pattern and encountering `{`, look ahead to determine if it's a range or map
  - Ranges will have the pattern `{digit[,digit]}`
  - Maps will have `{*}` or `{pattern: pattern}` or `{{digit[,digit]}}`

## Phase 3: Integration and Pattern Creation

**Goal**: Integrate the new parsing with the existing pattern creation infrastructure.

### 3.1 Extend Pattern Creation Methods
- **File**: `src/pattern/pattern_impl.rs`
- **Action**: Add new methods for creating dcbor-pattern-based map patterns:
  - `map_from_dcbor_pattern(dcbor_pattern: dcbor_pattern::Pattern) -> Self`
- **Pattern**: Follow the same approach as `array_from_dcbor_pattern()`

### 3.2 Update Pattern Display
- **File**: `src/pattern/pattern_impl.rs` or relevant display implementation
- **Action**: Update the `Display` trait to show new syntax instead of old syntax
- **Note**: This will be done in the final phase when removing old syntax

## Phase 4: Testing and Validation

**Goal**: Ensure the new functionality works correctly before removing old syntax.

### 4.1 Add New Syntax Tests
- **File**: `tests/parse_tests_leaf.rs`
- **Action**: Add comprehensive tests for new map syntax:
  - `{*}` → matches any map
  - `{{n}}` → matches map with exactly n entries
  - `{{n,m}}` → matches map with n to m entries
  - `{{n,}}` → matches map with at least n entries
  - `{key: value}` → matches specific key-value patterns
- **Approach**: Add new test functions alongside existing `parse_map_patterns()` test

### 4.2 Add Integration Tests
- **File**: New test file or existing integration tests
- **Action**: Test that new syntax works with actual envelopes and CBOR maps
- **Coverage**: Ensure edge cases work (empty maps, complex nested patterns, etc.)

## Phase 5: Remove Old Syntax (Final Step)

**Goal**: Remove the deprecated `MAP` syntax and update all references to use new syntax.

### 5.1 Remove Old Parser Logic
- **File**: `src/parse/leaf/map_parser.rs`
- **Action**: Replace the old `parse_map()` function with the new syntax-based implementation
- **Note**: This completely removes backward compatibility as specified

### 5.2 Remove MAP Token
- **File**: `src/parse/token.rs`
- **Action**: Remove the `Map` token variant and its recognition
- **File**: `src/parse/meta/primary_parser.rs`
- **Action**: Remove the `Token::Map => leaf::parse_map(lexer)` case

### 5.3 Update Pattern Display
- **File**: Pattern display implementation
- **Action**: Update all map pattern display to use new syntax in `to_string()` methods

### 5.4 Update Tests
- **File**: `tests/parse_tests_leaf.rs`
- **Action**: Update the `parse_map_patterns()` test to use new syntax
- **Details**:
  - `"MAP"` → `"{*}"`
  - `"MAP(3)"` → `"{{3}}"`
  - `"MAP({2,4})"` → `"{{2,4}}"`
  - `"MAP({2,})"` → `"{{2,}}"`

### 5.5 Update Documentation
- **File**: `docs/EnvelopePatternSyntax.md`
- **Action**: Remove old MAP syntax and replace with examples of new syntax

## Key Implementation Notes

### Proxy Architecture
- **Critical**: The map pattern is just a proxy for `dcbor-pattern` functionality
- **Delegation**: Pass content from `{` through `}` directly to `dcbor-pattern::Pattern::parse()`
- **No Envelope Logic**: No need to handle envelope patterns inside map patterns - just CBOR matching

### Token Disambiguation Strategy
- **Context-Aware Parsing**: Use lookahead to determine if `{` starts a map pattern or interval
- **Decision Logic**:
  - If `{` followed by `*` → map pattern
  - If `{` followed by `{` → map pattern (e.g., `{{3}}`)
  - If `{` followed by pattern containing `:` → map pattern
  - If `{` followed by `digit,digit}` → interval (existing range logic)

### Testing Strategy
- **Parallel Implementation**: Implement new syntax alongside old syntax first
- **Comprehensive Testing**: Test all syntax variations before removing old patterns
- **Regression Testing**: Ensure existing functionality continues to work during transition

### Error Handling
- **Improved Messages**: Provide helpful error messages for malformed map patterns
- **Graceful Degradation**: Handle edge cases like unmatched braces, invalid patterns

This development plan ensures a systematic approach to replacing the MAP syntax while maintaining code quality and test coverage throughout the process.
