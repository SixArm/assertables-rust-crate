//! Assert an expression is Ready.
//!
//! Deprecated. Please rename from `assert_poll_ready` into `assert_ready` because more developers prefer the shorter name.

/// Assert an expression is Ready.
///
/// Deprecated. Please rename from `assert_poll_ready_as_result` into `assert_ready_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_poll_ready_as_result` into `assert_ready_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_poll_ready_as_result {
    ($($arg:tt)*) => {
        $crate::assert_ready_as_result!($($arg)*)
    }
}

/// Assert an expression is Ready.
///
/// Deprecated. Please rename from `assert_poll_ready` into `assert_ready` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_poll_ready` into `assert_ready` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_poll_ready {
    ($($arg:tt)*) => {
        $crate::assert_ready!($($arg)*)
    }
}

/// Assert poll.is_ready() is true.
///
/// Deprecated. lease rename from `debug_assert_poll_ready` into `debug_assert_ready` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_poll_ready` into `debug_assert_ready` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_poll_ready {
    ($($arg:tt)*) => {
        $crate::debug_assert_ready!($($arg)*)
    }
}
