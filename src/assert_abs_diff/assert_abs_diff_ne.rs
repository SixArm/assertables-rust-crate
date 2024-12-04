//! Assert an absolute difference is not equal to an expression.
//!
//! Deprecated. Please rename from `assert_abs_diff_ne` into `assert_abs_diff_ne_x`.

/// Assert an absolute difference is not equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_ne_as_result` into `assert_abs_diff_ne_x_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_abs_diff_ne_as_result` into `assert_abs_diff_ne_x_as_result`."
)]
#[macro_export]
macro_rules! assert_abs_diff_ne_as_result {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_ne_x_as_result!($($arg)*)
    }
}

/// Assert an absolute difference is not equal to an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_ne` into `assert_abs_diff_ne_x`.
///
#[deprecated(note = "Please rename from `assert_abs_diff_ne` into `assert_abs_diff_ne_x`.")]
#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_ne_x!($($arg)*)
    }
}

/// Assert an absolute difference is not equal to an expression.
///
/// Deprecated. Please rename from `debug_assert_abs_diff_ne` into `debug_assert_abs_diff_ne_x`.
///
#[deprecated(
    note = "Please rename from `debug_assert_abs_diff_ne` into `debug_assert_abs_diff_ne_x`."
)]
#[macro_export]
macro_rules! debug_assert_abs_diff_ne {
    ($($arg:tt)*) => {
        $crate::debug_assert_abs_diff_ne_x!($($arg)*)
    }
}
