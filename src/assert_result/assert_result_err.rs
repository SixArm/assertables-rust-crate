//! Assert expression is Err.
//!
//! Deprecated. Please rename from `assert_result_err` into `assert_err` because more developers prefer the shorter name.

/// Assert expression is Err.
///
/// Deprecated. Please rename from `assert_result_err_as_result` into `assert_err_as_result` because more developers prefer the shorter name.
/// 
#[deprecated(
    note = "Please rename from `assert_result_err_as_result` into `assert_err_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_err_as_result {
    ($($arg:tt)*) => {
        $crate::assert_err_as_result!($($arg)*)
    }
}

/// Assert expression is Err.
///
/// Deprecated. Please rename from `assert_result_err` into `assert_err` because more developers prefer the shorter name.
#[deprecated(
    note = "Please rename from `assert_result_err` into `assert_err` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_err {
    ($($arg:tt)*) => {
        $crate::assert_err!($($arg)*)
    }
}

/// Assert expression is Err.
///
/// Deprecated. Please rename from `debug_assert_result_err` into `debug_assert_err` because more developers prefer the shorter name.
/// 
#[deprecated(
    note = "Please rename from `debug_assert_result_err` into `debug_assert_err` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_result_err {
    ($($arg:tt)*) => {
        $crate::debug_assert_err!($($arg)*)
    }
}
