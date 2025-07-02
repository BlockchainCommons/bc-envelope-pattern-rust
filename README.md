# Blockchain Commons Pattern Matcher for Gordian Envelope

### _by Wolf McNally and Blockchain Commons_

---

## Introduction

[Gordian Envelope](https://www.blockchaincommons.com/introduction/Envelope-Intro/) is a structured format for hierarchical binary data focused on privacy. The Rust implementation provides a feature-rich, production-ready reference implementation.

Envelopes are designed to facilitate "smart documents" with a number of unique features:

- **Hierarchical structure**: Easy representation of a variety of semantic structures, from simple key-value pairs to complex property graphs
- **Merkle-like digest tree**: Built-in integrity verification at any level of the structure
- **Deterministic representation**: Uses CBOR with deterministic encoding rules for consistent serialization
- **Privacy-focused**: The holder of a document can selectively encrypt or elide specific parts without invalidating the structure, signatures, or digest tree
- **Progressive trust**: Holders can reveal information incrementally to build trust with verifiers

This crate provides a pattern matcher and text syntax pattern parser for Gordian Envelope, allowing you to match specific structures within envelopes.

## Getting Started

```toml
[dependencies]
bc-envelope-pattern = "0.1.0"
```

## The Pattern Matcher

Unlike regular expressions which match sequential characters in strings, Gordian Envelope patterns match sequential elements of envelope trees. Therefore a given pattern can match multiple paths from the root of the envelope, and hence the matcher can return multiple matches.

## Overview of Usage

This crate provides two APIs for creating `Pattern` objects:

- **Programmatic**: The `Pattern` type has many functions for creating various kinds of patterns. For example, the `Pattern::any_text()` function creates a pattern that matches any text value, while the `Pattern::cbor()` function creates a pattern that matches a given CBOR value.
- **Pattern Expression Syntax**: Allows you to write patterns in a human-readable format that can be parsed into `Pattern` objects using `Pattern::parse()`, which takes a string containing the pattern and returns a `Pattern` object or an error if the pattern is invalid.

Once you have a `Pattern` object, you can use it to match against Gordian Envelope structures.

- `Pattern::match()` function takes an `Envelope` and returns a simple boolean indicating whether the pattern matches the envelope.
- `Pattern::paths()` returns a `Vec<Path>` containing the paths that match the pattern within the envelope. Each `Path` is a `Vec<Envelope>` representing a sequential traversal of envelopes that match the pattern, starting from the root of the envelope.
- `Pattern::paths_with_captures()` returns the same `Vec<Path>` but also another set of paths for each named capture in the pattern. This allows you to extract specific parts of the envelope that match named captures in the pattern.

The `format_paths()` and `format_paths_opt()` functions allow you to format paths in various ways, such as as URs, summaries, or just the last element of each path. This is useful for displaying the results of pattern matching in a human-readable format.

## Docs

The [docs/envelope_patex.md](docs/envelope_patex.md) file contains the specification for the pattern expression syntax used by this crate.

Further documentation is forthcoming and will be added to the `docs/` directory.

## Tools

The [`envelope` command-line tool](https://crates.io/crates/bc-envelope-cli) provides a powerful interface for working with Gordian Envelope patterns. You can use it to match patterns against envelopes, extract data, and format the output in various ways.
