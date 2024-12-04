//! Assert an absolute difference is equal to an expression.
//!
//! Deprecated. Please rename from `assert_abs_diff_eq_as_resul` into `assert_abs_diff_eq_x`.

/// Assert an absolute difference is equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_eq_as_result` into `assert_abs_diff_eq_x_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_abs_diff_eq_as_result` into `assert_abs_diff_eq_x_as_result`."
)]
#[macro_export]
macro_rules! assert_abs_diff_eq_as_result {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_eq_x_as_result!($($arg)*)
    }
}

/// Assert an absolute difference is equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_eq` into `assert_abs_diff_eq_x`.
///
#[deprecated(note = "Please rename from `assert_abs_diff_eq` into `assert_abs_diff_eq_x`.")]
#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_eq_x!($($arg)*)
    }
}

/// Assert an absolute difference is equal to an expression.
///
/// Deprecated. Please rename from `debug_assert_abs_diff_eq` into `debug_assert_abs_diff_eq_x`.
///
#[deprecated(
    note = "Please rename from `debug_assert_abs_diff_eq` into `debug_assert_abs_diff_eq_x`."
)]
#[macro_export]
macro_rules! debug_assert_abs_diff_eq {
    ($($arg:tt)*) => {
        $crate::debug_assert_abs_diff_eq_x!($($arg)*)
    }
}
