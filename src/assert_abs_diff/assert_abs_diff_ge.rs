//! Assert an absolute difference is greater than or equal to an expression.
//!
//! Deprecated. Please rename from `assert_abs_diff_ge` into `assert_abs_diff_ge_x`.

/// Assert an absolute difference is greater than or equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_ge_as_result` into `assert_abs_diff_ge_x_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_abs_diff_ge_as_result` into `assert_abs_diff_ge_x_as_result`."
)]
#[macro_export]
macro_rules! assert_abs_diff_ge_as_result {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_ge_x_as_result!($($arg)*)
    }
}

/// Assert an absolute difference is greater than or equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_ge` into `assert_abs_diff_ge_x`.
///
#[deprecated(note = "Please rename from `assert_abs_diff_ge` into `assert_abs_diff_ge_x`.")]
#[macro_export]
macro_rules! assert_abs_diff_ge {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_ge_x!($($arg)*)
    }
}

/// Assert an absolute difference is greater than or equal to an expression.
///
/// Deprecated. Please rename from `debug_assert_abs_diff_ge` into `debug_assert_abs_diff_ge_x`.
///
#[deprecated(
    note = "Please rename from `debug_assert_abs_diff_ge` into `debug_assert_abs_diff_ge_x`."
)]
#[macro_export]
macro_rules! debug_assert_abs_diff_ge {
    ($($arg:tt)*) => {
        $crate::debug_assert_abs_diff_ge_x!($($arg)*)
    }
}
