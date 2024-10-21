//! Assert two expressions are Ok and their values are equal.
//!
//! Deprecated. Please rename from `assert_result_ok_eq` into `assert_ok_eq` because more developers prefer the shorter name.

/// Assert two expressions are Ok and their values are equal.
///
/// Deprecated. Please rename from `assert_result_ok_eq_as_result` into `assert_ok_eq_as_result` because more developers prefer the shorter name.
/// 
#[deprecated(
    note = "Please rename from `assert_result_ok_eq_as_result` into `assert_ok_eq_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok_eq_as_result {
    ($($arg:tt)*) => {
        $crate::assert_ok_eq!($($arg)*)
    }
}

/// Assert two expressions are Ok and their values are equal.
///
/// Deprecated. Please rename from `assert_result_ok_eq` into `assert_ok_eq` because more developers prefer the shorter name.
/// 
#[deprecated(
    note = "Please rename from `assert_result_ok_eq` into `assert_ok_eq` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok_eq {
    ($($arg:tt)*) => {
        $crate::assert_ok_eq!($($arg)*)
    }
}

/// Assert two expressions are Ok and their values are equal.
///
/// Deprecated. Please rename from `debug_assert_result_ok_eq` into `debug_assert_ok_eq_as_result` because more developers prefer the shorter name.
/// 
#[deprecated(
    note = "Please rename from `debug_assert_result_ok_eq` into `debug_assert_result_ok_eq` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_result_ok_eq {
    ($($arg:tt)*) => {
        $crate::assert_ok_eq!($($arg)*)
    }
}
