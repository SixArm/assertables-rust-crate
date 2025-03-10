//! Assert two expressions are Some and their values are not equal.
//!
//! Deprecated. Please rename from `assert_option_some_ne` into `assert_some_ne` because more developers prefer the shorter name.

/// Assert two expressions are Some and their values are not equal.
///
/// Deprecated. Please rename from `assert_option_some_ne_as_result` into `assert_some_ne_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_some_ne_as_result` into `assert_some_ne_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_some_ne_as_result {
    ($($arg:tt)*) => {
        $crate::assert_some_ne_as_result!($($arg)*)
    }
}

/// Assert two expressions are Some and their values are not equal.
///
/// Deprecated. Please rename from `assert_option_some_ne` into `assert_some_ne` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_option_some_ne` into `assert_some_ne` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_option_some_ne {
    ($($arg:tt)*) => {
        $crate::assert_some_ne!($($arg)*)
    }
}

/// Assert two expressions are Some and their values are not equal.
///
/// Please rename from `debug_assert_option_some_ne` into `debug_assert_some_ne` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_option_some_ne` into `debug_assert_some_ne` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_option_some_ne {
    ($($arg:tt)*) => {
        $crate::debug_assert_some_ne!($($arg)*)
    }
}
