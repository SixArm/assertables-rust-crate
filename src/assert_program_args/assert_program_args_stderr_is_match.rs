//! Assert a command (built with program and args) stderr string is a match to a regex.
//!
//! Deprecated. Please rename from `assert_program_args_stderr_is_match` to `assert_program_args_stderr_string_is_match`.

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Deprecated. Please rename from `assert_program_args_stderr_is_match_as_result` to `assert_program_args_stderr_string_is_match_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_program_args_stderr_is_match_as_result` to `assert_program_args_stderr_string_is_match_as_result`."
)]
#[macro_export]
macro_rules! assert_program_args_stderr_is_match_as_result {
    ($($arg:tt)*) => {
        $crate::assert_program_args_stderr_string_is_match_as_result!($($arg)*)
    }
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Deprecated. Please rename from `assert_program_args_stderr_is_match` to `assert_program_args_stderr_string_is_match`.
///
#[deprecated(
    note = "Please rename from `assert_program_args_stderr_is_match` to `assert_program_args_stderr_string_is_match`."
)]
#[macro_export]
macro_rules! assert_program_args_stderr_is_match {
    ($($arg:tt)*) => {
        $crate::assert_program_args_stderr_string_is_match!($($arg)*)
    }
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Deprecated. Please rename from `debug_assert_program_args_stderr_is_match` to `debug_assert_program_args_stderr_string_is_match`.
///
#[deprecated(
    note = "Please rename from `debug_assert_program_args_stderr_is_match` to `debug_assert_program_args_stderr_string_is_match`."
)]
#[macro_export]
macro_rules! debug_assert_program_args_stderr_is_match {
    ($($arg:tt)*) => {
        $crate::debug_assert_program_args_stderr_string_is_match!($($arg)*)
    }
}
