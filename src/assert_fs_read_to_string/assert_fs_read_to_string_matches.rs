//! Assert a ::std::fs::read_to_string(path) is a match to a regex.
//!
//! Deprecated. Please rename from `assert_fs_read_to_string_matches` into `assert_fs_read_to_string_is_match`.

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Deprecated. Please rename from `assert_fs_read_to_string_matches_as_result` into `assert_fs_read_to_string_is_match_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_fs_read_to_string_matches_as_result` into `assert_fs_read_to_string_is_match_as_result`."
)]
#[macro_export]
macro_rules! assert_fs_read_to_string_matches_as_result {
    ($($arg:tt)*) => {
        $crate::assert_fs_read_to_string_is_match_as_result!($($arg)*)
    }
}

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Deprecated. Please rename from `assert_fs_read_to_string_matches` into `assert_fs_read_to_string_is_match`.
///
#[deprecated(
    note = "Please rename from `assert_fs_read_to_string_matches` into `assert_fs_read_to_string_is_match`."
)]
#[macro_export]
macro_rules! assert_fs_read_to_string_matches {
    ($($arg:tt)*) => {
        $crate::assert_fs_read_to_string_is_match!($($arg)*)
    }
}

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Deprecated. Please rename from `debug_assert_fs_read_to_string_matches` into `debug_assert_fs_read_to_string_is_match`.
///
#[deprecated(
    note = "Please rename from `debug_assert_fs_read_to_string_matches` into `debug_assert_fs_read_to_string_is_match`."
)]
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_matches {
    ($($arg:tt)*) => {
        $crate::debug_assert_fs_read_to_string_is_match!($($arg)*)
    }
}
