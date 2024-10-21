//! Assert expression is None.
//!
//! Deprecated. Please rename from `assert_option_none` into `assert_none` because more developers prefer the shorter name.

/// Assert expression is None.
///
/// Deprecated. Please rename from `assert_option_none_as_result` into `assert_none_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_none_as_result` into `assert_none_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_none_as_result {
    ($($arg:tt)*) => {
        $crate::assert_none_as_result!($($arg)*)
    }
}

/// Assert expression is None.
///
/// Deprecated. Please rename from `assert_option_none` into `assert_none` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_none` into `assert_none` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_none {
    ($($arg:tt)*) => {
        $crate::assert_option_none!($($arg)*)
    }
}

/// Assert expression is None.
///
/// Deprecated. Please rename from `debug_assert_option_none` into `debug_assert_none` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_option_none` into `debug_assert_none` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_option_none {
    ($($arg:tt)*) => {
        $crate::debug_assert_none!($($arg)*)
    }
}
