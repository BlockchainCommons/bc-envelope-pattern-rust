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

### Phase 2: Enhanced Path and Capture Integration

Currently, bc-envelope-pattern matches return envelope-level paths.

- **Extend path details**: When dcbor-pattern matches within CBOR leaves, extend paths to show internal CBOR structure. Use `.into()` to convert `CBOR` objects to Envelope leaves and `.as_leaf()` to convert Envelope leaves to `CBOR` objects.
- **Enhanced captures**: Merge dcbor-pattern captures with envelope captures for more detailed extraction
- **Composite formatting**: Show both envelope context and internal CBOR match details
- **Better error spans**: Ensure that `dcbor-pattern` parsing errors are reported with precise location information at the `bc-envelope-pattern` level.

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
