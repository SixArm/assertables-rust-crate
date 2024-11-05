//! Namespaces: examples of how to include Assertables macros in source.
//!
//! Some developers prefer to use a wildcard to use all the macros.
//!
//! Other developers prefer to be specific to use just some macros.

/// Use a wildcard when you want all the assertables macros.
#[test]
fn use_wildcard() {
    use assertables::*;
    assert_starts_with!("alfa", "al");
}

/// Use one macro.
#[test]
fn use_one_macro() {
    use assertables::assert_starts_with;
    assert_starts_with!("alfa", "al");
}

/// Use multiple macros.
#[test]
fn use_multiple_macros() {
    use assertables::{
        assert_starts_with,
        assert_contains,
        assert_ends_with
    };
    assert_starts_with!("alfa", "al");
    assert_contains!("alfa", "lf");
    assert_ends_with!("alfa", "fa");
}
