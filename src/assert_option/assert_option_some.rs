//! Assert expression is Some.
//!
//! Deprecated. Please rename from `assert_option_some` into `assert_some` because more developers prefer the shorter name.

/// Assert expression is Some.
///
/// Deprecated. Please rename from `assert_option_some_as_result` into `assert_some_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_some_as_result` into `assert_some_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_some_as_result {
    ($($arg:tt)*) => {
        $crate::assert_some_as_result!($($arg)*)
    }
}

/// Assert expression is Some.
///
/// Deprecated. Please rename from `assert_option_some` into `assert_some` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_some` into `assert_some` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_some {
    ($($arg:tt)*) => {
        $crate::assert_some!($($arg)*)
    }
}

/// Assert expression is Some.
///
/// Deprecated. Please rename from `debug_assert_option_some` into `debug_assert_some` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_option_some` into `debug_assert_some` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_option_some {
    ($($arg:tt)*) => {
        $crate::debug_assert_some!($($arg)*)
    }
}
