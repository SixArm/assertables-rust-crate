//! Validation examples
//!
//! The Assertables macros can be useful for runtime validation, verification,
//! sanitization, reliability engineering, chaos engineering, and the like.
//!
//! This file has some very simple demonstrations of how to use Assertables in
//! validations.

use assertables::*;

#[test]
fn validate_email_address() {

	fn validate_email_address(s: &str) -> Result<(), String> {
		assert_contains_as_result!(s, "@")?;
		assert_contains_as_result!(3..255, &s.len())
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

#[test]
fn validate_point_nearness() {

	fn validate_point_nearness(x1: f32, y1: f32, x2: f32, y2: f32) -> Result<(), String> {
		assert_in_delta_as_result!(x1, x2, 0.1)?;
		assert_in_delta_as_result!(y1, y2, 0.1)
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

#[test]
fn validate_positive_percentage() {

	fn validate_positive_percentage(x: f32) -> Result<(), String> {
		assert_infix_as_result!(x > 0.0)?;
		assert_infix_as_result!(x <= 100.0)
	}

	// Success
	let result = validate_positive_percentage(1.0);
	assert_ok!(result);
	
	// Failure because the number is too low
	let result = validate_positive_percentage(0.0);
	assert_err!(result);

	// Failure because the number is too high
	let result = validate_positive_percentage(101.0);
	assert_err!(result);

}
