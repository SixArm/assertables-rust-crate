//! Assert a command stderr string is a match to a regex.
//!
//! Deprecated. Please rename from `assert_command_stderr_is_match` into `assert_command_stderr_string_is_match`.

/// Assert a command stderr string is a match to a regex.
///
/// Deprecated. Please rename from `assert_command_stderr_is_match_as_result`
/// into `assert_command_stderr_string_is_match_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_command_stderr_is_match_as_result` into `assert_command_stderr_string_is_match_as_result`."
)]
#[macro_export]
macro_rules! assert_command_stderr_is_match_as_result {
    ($($arg:tt)*) => {
        $crate::assert_command_stderr_string_is_match_as_result!($($arg)*)
    }
}

/// Assert a command stderr string is a match to a regex.
///
/// Deprecated. Please rename from `assert_command_stderr_is_match` into `assert_command_stderr_string_is_match`.
///
#[deprecated(
    note = "Please rename from `assert_command_stderr_is_match` into `assert_command_stderr_string_is_match`."
)]
#[macro_export]
macro_rules! assert_command_stderr_is_match {
    ($($arg:tt)*) => {
        $crate::assert_command_stderr_string_is_match!($($arg)*)
    }
}

/// Assert a command stderr string is a match to a regex.
///
/// Deprecated. Please rename from `debug_assert_command_stderr_is_match` into `debug_assert_command_stderr_string_is_match`.
///
#[deprecated(
    note = "Please rename from `debug_assert_command_stderr_is_match` into `debug_assert_command_stderr_string_is_match`."
)]
#[macro_export]
macro_rules! debug_assert_command_stderr_is_match {
    ($($arg:tt)*) => {
        $crate::debug_assert_command_stderr_string_is_match!($($arg)*)
    }
}
