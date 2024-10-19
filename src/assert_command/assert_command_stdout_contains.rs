//! Assert a command stdout string contains a given containee.
//!
//! Deprecated. Please rename from `assert_command_stdout_contains` into `assert_command_stdout_string_contains`.

/// Assert a command stdout string contains a given containee.
///
/// Deprecated. Please rename from `assert_command_stdout_contains_as_result` into `assert_command_stdout_string_contains_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_command_stdout_contains_as_result` into `assert_command_stdout_string_contains_as_result`."
)]
#[macro_export]
macro_rules! assert_command_stdout_contains_as_result {
    ($($arg:tt)*) => {
        $crate::assert_command_stdout_string_contains_as_result!($($arg)*)
    }
}

/// Assert a command stdout string contains a given containee.
///
/// Deprecated. Please rename from `assert_command_stdout_contains` into `assert_command_stdout_string_contains`.")]
///
#[deprecated(
    note = "Please rename from `assert_command_stdout_contains` into `assert_command_stdout_string_contains`."
)]
#[macro_export]
macro_rules! assert_command_stdout_contains {
    ($($arg:tt)*) => {
        $crate::assert_command_stdout_string_contains!($($arg)*)
    }
}

/// Assert a command stdout string contains a given containee.
///
/// Deprecated. Please rename from `debug_assert_command_stdout_contains` into `debug_assert_command_stdout_string_contains`.
///
#[deprecated(
    note = "Please rename from `debug_assert_command_stdout_contains` into `debug_assert_command_stdout_string_contains`."
)]
#[macro_export]
macro_rules! debug_assert_command_stdout_contains {
    ($($arg:tt)*) => {
        $crate::debug_assert_command_stdout_string_contains!($($arg)*)
    }
}
