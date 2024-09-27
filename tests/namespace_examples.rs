//! Namespace examples.
//! 
//! This file has some simple demonstrations of how to use Assertables in
//! validations by using a namespace, explicitly using the macro names.

#[test]
fn test_wildcard_namespace() {
    use assertables::*;
    assert_starts_with!("alfa", "al");
}

#[test]
fn test_one_macro_namespace() {
    use assertables::assert_starts_with;
    assert_starts_with!("alfa", "al");
}

#[test]
fn test_two_macro_namespace() {
    use assertables::{assert_starts_with, assert_starts_with_as_result};
    assert_starts_with!("alfa", "al");
    assert!(assert_starts_with_as_result!("alfa", "al").is_ok());
}

