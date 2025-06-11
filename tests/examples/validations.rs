//! Validations: examples of Assertables macros using results for validations.
//!
//! The Assertables macros can be useful for runtime validation, verification,
//! sanitization, reliability engineering, chaos engineering, and the like.
//!
//! This file has some very simple demonstrations of how to use Assertables in
//! validations.

use assertables::*;

/// Validate an email address format, such as "alice@example.com".
/// Note that real-world code would use more-complex email validation.
#[test]
fn validate_email_address() {
    fn validate_email_address(s: &str) -> Result<(), String> {
        assert_contains_as_result!(s, "@")?;
        assert_contains_as_result!(3..255, &s.len())?;
        Ok(())
    }

    // Success
    let result = validate_email_address("alice@example.com");
    assert_ok!(result);

    // Failure because the string is missing the @ sign
    let result = validate_email_address("alfa");
    assert_err!(result);

    // Failure because the string is too short
    let result = validate_email_address("@");
    assert_err!(result);
}

/// Validate point nearness, such as point (1.01, 2.01) being near point (1.02, 2.02).
/// Note that real-world code would use more-complex geometry validation.
#[test]
fn validate_point_nearness() {
    fn validate_point_nearness(x1: f32, y1: f32, x2: f32, y2: f32) -> Result<(), String> {
        assert_in_delta_as_result!(x1, x2, 0.1)?;
        assert_in_delta_as_result!(y1, y2, 0.1)?;
        Ok(())
    }

    // Success
    let result = validate_point_nearness(1.01, 2.01, 1.02, 2.02);
    assert_ok!(result);

    // Failure because the x coordinates are far
    let result = validate_point_nearness(1.01, 2.01, 9.02, 2.02);
    assert_err!(result);

    // Failure because the y coordinates are far
    let result = validate_point_nearness(1.01, 2.01, 1.02, 9.02);
    assert_err!(result);
}

/// Validate a string contains certain words such as "alfa" and "bravo".
/// Note that real-world code would use more-complex text validation.
#[test]
fn validate_words() {
    fn validate_words(s: &str) -> Result<(), String> {
        assert_contains_as_result!(s, "alfa")?;
        assert_contains_as_result!(s, "bravo")?;
        Ok(())
    }

    // Success
    let result = validate_words("this text contains alfa and bravo");
    assert_ok!(result);

    // Failure because the word "alfa" is missing
    let result = validate_words("this text contains bravo");
    assert_err!(result);

    // Failure because the word "bravo" is missing
    let result = validate_words("this text contains alfa");
    assert_err!(result);
}
