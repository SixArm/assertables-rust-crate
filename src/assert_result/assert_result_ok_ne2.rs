//! Assert two expressions are Ok and their values are not equal.
//!
//! Deprecated. Please rename from `assert_result_ok_ne2` into `assert_ok_ne2` because more developers prefer the shorter name.

/// Assert two expressions are Ok and their values are not equal.
///
/// Please rename from `assert_result_ok_ne2_as_result` into `assert_ok_ne2_as_result` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_result_ok_ne2_as_result` into `assert_ok_ne2_as_result` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok_ne2_as_result {
    ($($arg:tt)*) => {
        $crate::assert_ok_ne2_as_result!($($arg)*)
    }
}

/// Assert two expressions are Ok and their values are not equal.
///
/// Please rename from `assert_result_ok_ne2` into `assert_ok_ne2` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `assert_result_ok_ne2` into `assert_ok_ne2` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! assert_result_ok_ne2 {
    ($($arg:tt)*) => {
        $crate::assert_ok_ne2!($($arg)*)
    }
}

/// Assert two expressions are Ok and their values are not equal.
///
/// Please rename from `debug_assert_result_ok_ne2` into `deubg_assert_ok_ne` because more developers prefer the shorter name.
///
#[deprecated(
    note = "Please rename from `debug_assert_result_ok_ne2` into `debug_result_ok_ne` because more developers prefer the shorter name."
)]
#[macro_export]
macro_rules! debug_assert_result_ok_ne2 {
    ($($arg:tt)*) => {
        $crate::debug_assert_ok_ne2!($($arg)*)
    }
}
