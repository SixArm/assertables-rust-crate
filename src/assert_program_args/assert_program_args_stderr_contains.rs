//! Assert a command (built with program and args) stderr string contains a given containee.
//!
//! Deprecated. Please rename from `assert_program_args_stderr_contains` to `assert_program_args_stderr_string_contains`.

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Deprecated. Please rename from `assert_program_args_stderr_contains_as_result` to `assert_program_args_stderr_string_contains_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_program_args_stderr_contains_as_result` to `assert_program_args_stderr_string_contains_as_result`."
)]
#[macro_export]
macro_rules! assert_program_args_stderr_contains_as_result {
    ($($arg:tt)*) => {
        $crate::assert_program_args_stderr_string_contains_as_result!($($arg)*)
    }
}

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Deprecated. Please rename from `assert_program_args_stderr_contains` to `assert_program_args_stderr_string_contains`.
///
#[deprecated(
    note = "Please rename from `assert_program_args_stderr_contains` to `assert_program_args_stderr_string_contains`."
)]
#[macro_export]
macro_rules! assert_program_args_stderr_contains {
    ($($arg:tt)*) => {
        $crate::assert_program_args_stderr_string_contains!($($arg)*)
    }
}

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Deprecated. Please rename from `debug_assert_program_args_stderr_contains` to `debug_assert_program_args_stderr_string_contains`.
///
#[deprecated(
    note = "Please rename from `debug_assert_program_args_stderr_contains` to `debug_assert_program_args_stderr_string_contains`."
)]
#[macro_export]
macro_rules! debug_assert_program_args_stderr_contains {
    ($($arg:tt)*) => {
        $crate::debug_assert_program_args_stderr_string_contains!($($arg)*)
    }
}
