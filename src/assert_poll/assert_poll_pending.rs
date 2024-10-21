//! Assert an expression is Pending.
//!
//! Deprecated. Please rename from `assert_poll_pending` into `assert_pending_as_result` because more developers prefer the shorter name.

/// Assert an expression.is_pending() is true.
///
/// Deprecated. Please rename from `assert_poll_pending_as_result` into `assert_pending_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_poll_pending_as_result` into `assert_pending_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_poll_pending_as_result {
    ($($arg:tt)*) => {
        $crate::assert_pending_as_result!($($arg)*)
    }
}

/// Assert an expression is Pending.
///
/// Deprecated. Please rename from `assert_poll_pending` into `assert_pending` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_poll_pending` into `assert_pending` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_poll_pending {
    ($($arg:tt)*) => {
        $crate::assert_pending!($($arg)*)
    }
}

/// Assert an expression is Pending.
///
/// Deprecated. Please rename from `debug_assert_poll_pending` into `debug_assert_pending` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_poll_pending` into `debug_assert_pending` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_poll_pending {
    ($($arg:tt)*) => {
        $crate::debug_assert_pending_as_result!($($arg)*)
    }
}
