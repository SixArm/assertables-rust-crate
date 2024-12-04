//! Assert an absolute difference is less than an expression.
//!
//! Deprecated. Please rename from `assert_abs_diff_lt` into `assert_abs_diff_lt_x`.

/// Assert an absolute difference is less than an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_lt_as_result` into `assert_abs_diff_lt_x_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_abs_diff_lt_as_result` into `assert_abs_diff_lt_x_as_result`."
)]
#[macro_export]
macro_rules! assert_abs_diff_lt_as_result {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_lt_x_as_result!($($arg)*)
    }
}

/// Assert an absolute difference is less than an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_lt` into `assert_abs_diff_lt_x`.
///
#[deprecated(note = "Please rename from `assert_abs_diff_lt` into `assert_abs_diff_lt_x`.")]
#[macro_export]
macro_rules! assert_abs_diff_lt {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_lt_x!($($arg)*)
    }
}

/// Assert an absolute difference is less than an expression.
///
/// Deprecated. Please rename from `debug_assert_abs_diff_lt` into `debug_assert_abs_diff_lt_x`.
///
#[deprecated(
    note = "Please rename from `debug_assert_abs_diff_lt` into `debug_assert_abs_diff_lt_x`."
)]
#[macro_export]
macro_rules! debug_assert_abs_diff_lt {
    ($($arg:tt)*) => {
        $crate::debug_assert_abs_diff_lt_x!($($arg)*)
    }
}
