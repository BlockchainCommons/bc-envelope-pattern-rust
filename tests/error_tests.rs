use bc_envelope_pattern::{Error, Pattern};

#[test]
fn test_unrecognized_token_error() {
    let result = Pattern::parse("invalid@pattern");
    match result {
        Err(Error::UnrecognizedToken(span)) => {
            assert_eq!(span.start, 0);
            assert_eq!(span.end, 1);
        }
        _ => panic!("Expected UnrecognizedToken error, got: {:?}", result),
    }
}

#[test]
fn test_unrecognized_token_at_specific_position() {
    let result = Pattern::parse("\"hello\"@");
    match result {
        Err(Error::UnrecognizedToken(span)) => {
            assert_eq!(span.start, 7);
            assert_eq!(span.end, 8);
        }
        _ => panic!("Expected UnrecognizedToken error, got: {:?}", result),
    }
}

#[test]
fn test_extra_data_error() {
    let result = Pattern::parse("\"hello\" \"world\"");
    match result {
        Err(Error::ExtraData(span)) => {
            assert_eq!(span.start, 8);
        }
        _ => panic!("Expected ExtraData error, got: {:?}", result),
    }
}

#[test]
fn test_unexpected_end_of_input() {
    let result = Pattern::parse("\"hello\" &");
    match result {
        Err(Error::UnexpectedEndOfInput) => {
            // Expected
        }
        _ => panic!("Expected UnexpectedEndOfInput error, got: {:?}", result),
    }
}

#[test]
fn test_valid_pattern_still_works() {
    let result = Pattern::parse("\"hello\"");
    assert!(
        result.is_ok(),
        "Valid pattern should parse successfully: {:?}",
        result
    );
}
