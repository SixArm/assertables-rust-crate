//! Assert an absolute difference is greater than an expression.
//!
//! Deprecated. Please rename from `assert_abs_diff_gt` into `assert_abs_diff_gt_x`.

/// Assert an absolute difference is greater than an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_gt_as_result` into `assert_abs_diff_gt_x_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_abs_diff_gt_as_result` into `assert_abs_diff_gt_x_as_result`."
)]
#[macro_export]
macro_rules! assert_abs_diff_gt_as_result {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_gt_x_as_result!($($arg)*)
    }
}

/// Assert an absolute difference is greater than an expression.
///
/// Deprecated. Please rename from `assert_abs_diff_gt` into `assert_abs_diff_gt_x`.
///
#[deprecated(note = "Please rename from `assert_abs_diff_gt` into `assert_abs_diff_gt_x`.")]
#[macro_export]
macro_rules! assert_abs_diff_gt {
    ($($arg:tt)*) => {
        $crate::assert_abs_diff_gt_x!($($arg)*)
    }
}

/// Assert an absolute difference is greater than an expression.
///
/// Deprecated. Please rename from `debug_assert_abs_diff_gt` into `debug_assert_abs_diff_gt_x`.
///
#[deprecated(
    note = "Please rename from `debug_assert_abs_diff_gt` into `debug_assert_abs_diff_gt_x`."
)]
#[macro_export]
macro_rules! debug_assert_abs_diff_gt {
    ($($arg:tt)*) => {
        $crate::debug_assert_abs_diff_gt_x!($($arg)*)
    }
}
