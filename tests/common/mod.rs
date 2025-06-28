pub mod check_encoding;
pub mod test_data;
pub mod test_seed;

/// A macro to assert that two values are equal, printing them if they are not,
/// including newlines and indentation they may contain. This macro is useful
/// for debugging tests where you want to see the actual and expected values
/// when they do not match.
///
/// Use this rubric for all tests expecting formatted output:
///
/// #[rustfmt::skip]
/// let expected = indoc! {r#"
///     Some multiline
///         string with
///             indentation
/// "#}.trim();
/// assert_actual_expected!(generate_actual(), expected, "Description of the
/// test");
///
/// Run it once with dummy data to let the test fail and see the expected
/// output format. If the actual output is correct, then copy the actual
/// output in the terminal to the `expected` variable, then run the test again
/// to ensure it passes.
#[macro_export]
macro_rules! assert_actual_expected {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val);
                }
            }
        }
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val, $($arg)+);
                }
            }
        }
    };
}
