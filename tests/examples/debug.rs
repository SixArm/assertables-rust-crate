//! Debug assertions: examples of how to include Assertables debug_assertmacros in source.
//!
//! Some developers prefer to use a wildcard to use all the macros.
//!
//! Other developers prefer to be specific to use just some macros.

/// Use a wildcard when you want all the assertables macros.
#[test]
fn use_wildcard() {
    use assertables::*;
    debug_assert_starts_with!("alfa", "al");
}

/// Use one macro.
#[test]
fn use_one_macro() {
    use assertables::debug_assert_starts_with;
    debug_assert_starts_with!("alfa", "al");
}

/// Use multiple macros.
#[test]
fn use_multiple_macros() {
    use assertables::{debug_assert_contains, debug_assert_ends_with, debug_assert_starts_with};
    debug_assert_starts_with!("alfa", "al");
    debug_assert_contains!("alfa", "lf");
    debug_assert_ends_with!("alfa", "fa");
}
