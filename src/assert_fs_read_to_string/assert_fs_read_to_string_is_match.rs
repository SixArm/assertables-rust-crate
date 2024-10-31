//! Assert a ::std::fs::read_to_string(path) is a match to a regex.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path) matches expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! # fn main() {
//! let path = "alfa.txt";
//! let matcher = Regex::new(r"alfa").unwrap();
//! assert_fs_read_to_string_is_match!(&path, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
//! * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
//! * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// * If true, return Result `Ok(path_into_string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_is_match`](macro.assert_fs_read_to_string_is_match.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_is_match_as_result {
    ($path:expr, $matcher:expr $(,)?) => {{
        match (&$path, &$matcher) {
            (path, matcher) => {
                match (::std::fs::read_to_string(path)) {
                    Ok(string) => {
                        if $matcher.is_match(&string) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                                        "    path label: `{}`,\n",
                                        "    path debug: `{:?}`,\n",
                                        " matcher label: `{}`,\n",
                                        " matcher debug: `{:?}`,\n",
                                        "        string: `{:?}`",
                                    ),
                                    stringify!($path),
                                    path,
                                    stringify!($matcher),
                                    matcher,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                                    "    path label: `{}`,\n",
                                    "    path debug: `{:?}`,\n",
                                    " matcher label: `{}`,\n",
                                    " matcher debug: `{:?}`,\n",
                                    "           err: `{:?}`"
                                ),
                                stringify!($path),
                                path,
                                stringify!($matcher),
                                matcher,
                                err
                            )
                        )
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn test_read_to_string_is_match_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"alfa").unwrap();
        let result = assert_fs_read_to_string_is_match_as_result!(&path, &matcher);
        assert_eq!(result.unwrap(), String::from("alfa\n"));
    }

    #[test]
    fn test_read_to_string_is_match_as_result_x_failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zz").unwrap();
        let result = assert_fs_read_to_string_is_match_as_result!(&path, &matcher);
        assert_eq!(
            result.unwrap_err(),
            format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                    "    path label: `&path`,\n",
                    "    path debug: `{:?}`,\n",
                    " matcher label: `&matcher`,\n",
                    " matcher debug: `Regex(\"zz\")`,\n",
                    "        string: `\"alfa\\n\"`",
                ),
                path
            )
        );
    }
}

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// * If true, return `path_into_string`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"alfa").unwrap();
/// assert_fs_read_to_string_is_match!(&path, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"zz").unwrap();
/// assert_fs_read_to_string_is_match!(&path, &matcher);
/// # });
/// // assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_is_match.html
/// //     path label: `&path`,
/// //     path debug: `\"alfa.txt\"`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zz\")`,
/// //         string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
/// #     "    path label: `&path`,\n",
/// #     "    path debug: `\"alfa.txt\"`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zz\")`,\n",
/// #     "        string: `\"alfa\\n\"`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_is_match {
    ($path:expr, $matcher:expr $(,)?) => {{
        match $crate::assert_fs_read_to_string_is_match_as_result!($path, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($path:expr, $matcher:expr, $($message:tt)+) => {{
        match $crate::assert_fs_read_to_string_is_match_as_result!($path, $matcher) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// This macro provides the same statements as [`assert_fs_read_to_string_is_match`](macro.assert_fs_read_to_string_is_match.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intended to work in a similar way to
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_is_match!($($arg)*);
        }
    };
}
