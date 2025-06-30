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

### COMPLETE:

- Update the `ANY` pattern to use the `dcbor-pattern` crate's `*` syntax for matching any envelope.
- Update the `BOOL` pattern to use the `dcbor-pattern` crate's `bool`, `true`, and `false` syntax for matching boolean values.
- Update the `NULL` pattern to use the `dcbor-pattern` crate's `null` syntax for matching null values.
- Update the `BSTR` pattern to use the `dcbor-pattern` crate's `bstr` syntax for matching byte strings.
- Update the `TEXT` pattern to use the `dcbor-pattern` crate's `text` syntax for matching text strings.
- Update the `NUMBER` pattern to use the `dcbor-pattern` crate's `number` syntax for matching numbers.
- Update the `DATE` pattern to use the `dcbor-pattern` crate's `date` syntax for matching dates.
- Update the `KNOWN` pattern to use the `dcbor-pattern` crate's `known` syntax for matching known values.

### NEXT TASK:

- Update the `TAG` pattern to use the `dcbor-pattern` crate's `tagged` syntax for matching arrays.

#### Old Syntax:
    - `TAG`
        - Matches any CBOR tagged value.
    - `TAG ( value )`
        - Matches the specified CBOR tagged value. This is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `TAG ( name )`
        - Matches the CBOR tagged value with the specified name. It is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `TAG ( /regex/ )`
        - Matches a CBOR tagged value with a name that matches the specified regex.

#### New Syntax:
    - `tagged`
        - Matches any CBOR tagged value.
    - `tagged ( value, pattern )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `tagged ( name, pattern )`
        - Matches the CBOR tagged value with the specified name and content that matches the given pattern. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `tagged ( /regex/, pattern )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

#### Important Notes:
    - Remember: once you've parsed `tagged` or `tagged(value, pattern)`, you are just a proxy for the functionality in `dcbor-pattern`. You do not need to implement any additional logic for the `tagged` pattern. There will be no envelope patterns inside the `tagged` pattern, so you do not need to worry about the envelope structure. You are just matching a CBOR value against a pattern, like every other leaf/value pattern. YOU ARE JUST A PROXY.
    - `dcbor-pattern` can do everything including looking up tag names.
    - You *do* need to call `bc_envelope::register_tags()` at the start of *every* test that may need tag name resolution.
    - This is de novo development, so DO NOT take any action to ensure backward-compatibility.
    - REPEAT: REMOVE THE OLD SYNTAX AND REPLACE IT WITH THE NEW SYNTAX.
    - Only put debug examples in `examples/`. Put tests you want to be kept for regression in `tests/`. DO NOT use the root directory or other directories for temporary debug examples.
