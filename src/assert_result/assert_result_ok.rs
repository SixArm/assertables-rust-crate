//! Assert expression is Ok.
//!
//! Deprecated. Please rename from `assert_result_ok` into `assert_ok` because more developers prefer the shorter name.

/// Assert expression is Ok.
///
/// Deprecated. Please rename from `assert_result_ok_as_result` into `assert_ok_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_result_ok_as_result` into `assert_ok_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok_as_result {
    ($($arg:tt)*) => {
        $crate::assert_ok_as_result!($($arg)*)
    }
}

/// Assert expression is Ok.
///
/// Deprecated. Please rename from `assert_result_ok` into `assert_ok` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_result_ok` into `assert_ok` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok {
    ($($arg:tt)*) => {
        $crate::assert_ok!($($arg)*)
    }
}

/// Assert expression is Ok.
///
/// Deprecated. Please rename from `debug_assert_result_ok` into `debug_assert_ok` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_result_ok` into `debug_assert_ok` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_result_ok {
    ($($arg:tt)*) => {
        $crate::debug_assert_ok!($($arg)*)
    }
}
