// Parsers for meta-pattern operators

mod or_parser;
mod sequence_parser;
mod not_parser;
mod and_parser;
mod primary_parser;
mod group_parser;
mod search_parser;

pub(crate) use or_parser::parse_or;
