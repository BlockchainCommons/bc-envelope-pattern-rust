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

- Update the `MAP` pattern to use the `dcbor-pattern` crate's new syntax for matching maps.

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
        - Old: `MAP({1})` becomes `{{1}}`
        - Old: `MAP({1, 2})` becomes `{{1, 2}}`
        - Old: `MAP({1,})` becomes `{{1,}}`
        - Old: `MAP(key: value, ...)` becomes `{key: value, ...}`

#### Important Notes:

    - Remember: The map pattern is just a proxy for the functionality in `dcbor-pattern`, and all you are doing is adapting the existing syntax. You do not need to implement any additional logic for the `map` pattern, just pass from `{` through `}` to `dcbor-pattern`. There will be no envelope patterns inside the `map` pattern, so you do not need to worry about the envelope structure. You are just matching a CBOR value against a pattern, like every other leaf/value pattern. YOU ARE JUST A PROXY.
    - This is de novo development, so DO NOT take any action to ensure backward-compatibility.
    - REPEAT: REMOVE THE OLD SYNTAX AND REPLACE IT WITH THE NEW SYNTAX.
    - Only put debug examples in `examples/`. Put tests you want to be kept for regression in `tests/`. DO NOT use the root directory or other directories for temporary debug examples.
    - Tip: We've already converted the `array` `[*]` pattern to use the new syntax, so you can use that as a reference for how to convert the `map` pattern.
