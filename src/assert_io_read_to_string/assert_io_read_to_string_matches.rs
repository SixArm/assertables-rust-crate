//! Assert a std::io::Read read_to_string() is a match to a regex.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a) ⇒ a) matches matcher
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//! use regex::Regex;
//!
//! # fn main() {
//! let mut reader = "hello".as_bytes();
//! let matcher = Regex::new(r"ell").unwrap();
//! assert_io_read_to_string_matches!(reader, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_matches`](macro@crate::assert_io_read_to_string_matches)
//! * [`assert_io_read_to_string_matches_as_result`](macro@crate::assert_io_read_to_string_matches_as_result)
//! * [`debug_assert_io_read_to_string_matches`](macro@crate::debug_assert_io_read_to_string_matches)

/// Assert a std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a) ⇒ a) matches matcher
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_matches`](macro@crate::assert_io_read_to_string_matches)
/// * [`assert_io_read_to_string_matches_as_result`](macro@crate::assert_io_read_to_string_matches_as_result)
/// * [`debug_assert_io_read_to_string_matches`](macro@crate::debug_assert_io_read_to_string_matches)
///
#[macro_export]
macro_rules! assert_io_read_to_string_matches_as_result {
    ($reader:expr, $matcher:expr $(,)?) => {{
        match (/*&$reader,*/ &$matcher) {
            matcher => {
                let mut read_string = String::new();
                let read_result = $reader.read_to_string(&mut read_string);
                if let Err(read_err) = read_result {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_io_read_to_string_matches!(a_reader, &matcher)`\n",
                            "https://docs.rs/assertables/8.11.0/assertables/macro.assert_io_read_to_string_matches.html\n",
                            "  reader label: `{}`,\n",
                            "  reader debug: `{:?}`,\n",
                            " matcher label: `{}`,\n",
                            " matcher debug: `{:?}`,\n",
                            "      read err: `{:?}`"
                        ),
                        stringify!($reader),
                        $reader,
                        stringify!($matcher),
                        matcher,
                        read_err
                    ))
                } else {
                    let _size = read_result.unwrap();
                    if $matcher.is_match(read_string.as_str()) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_io_read_to_string_matches!(a_reader, &matcher)`\n",
                                "https://docs.rs/assertables/8.11.0/assertables/macro.assert_io_read_to_string_matches.html\n",
                                "  reader label: `{}`,\n",
                                "  reader debug: `{:?}`,\n",
                                " matcher label: `{}`,\n",
                                " matcher debug: `{:?}`,\n",
                                " reader string: `{:?}`",
                            ),
                            stringify!($reader),
                            $reader,
                            stringify!($matcher),
                            matcher,
                            read_string
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::io::Read;

    #[test]
    fn test_assert_io_read_to_string_matches_as_result_x_success() {
        let mut reader = "alfa".as_bytes();
        let matcher = Regex::new(r"alfa").unwrap();
        let result = assert_io_read_to_string_matches_as_result!(reader, &matcher);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_io_read_to_string_matches_as_result_x_failure() {
        let mut reader = "alfa".as_bytes();
        let matcher = Regex::new(r"zzz").unwrap();
        let result = assert_io_read_to_string_matches_as_result!(reader, &matcher);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_io_read_to_string_matches!(a_reader, &matcher)`\n",
                "https://docs.rs/assertables/8.11.0/assertables/macro.assert_io_read_to_string_matches.html\n",
                "  reader label: `reader`,\n",
                "  reader debug: `[]`,\n",
                " matcher label: `&matcher`,\n",
                " matcher debug: `Regex(\"zzz\")`,\n",
                " reader string: `\"alfa\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a) ⇒ a) matches matcher
///
/// * If true, return `()`.
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
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_io_read_to_string_matches!(reader, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"zzz").unwrap();
/// assert_io_read_to_string_matches!(reader, &matcher);
/// # });
/// // assertion failed: `assert_io_read_to_string_matches!(a_reader, &matcher)`
/// // https://docs.rs/assertables/8.11.0/assertables/macro.assert_io_read_to_string_matches.html
/// //   reader label: `reader`,
/// //   reader debug: `[]`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zzz\")`,
/// //  reader string: `\"hello\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_io_read_to_string_matches!(a_reader, &matcher)`\n",
/// #     "https://docs.rs/assertables/8.11.0/assertables/macro.assert_io_read_to_string_matches.html\n",
/// #     "  reader label: `reader`,\n",
/// #     "  reader debug: `[]`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zzz\")`,\n",
/// #     " reader string: `\"hello\"`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_matches`](macro@crate::assert_io_read_to_string_matches)
/// * [`assert_io_read_to_string_matches_as_result`](macro@crate::assert_io_read_to_string_matches_as_result)
/// * [`debug_assert_io_read_to_string_matches`](macro@crate::debug_assert_io_read_to_string_matches)
///
#[macro_export]
macro_rules! assert_io_read_to_string_matches {
    ($a_reader:expr, $b_matcher:expr $(,)?) => {{
        match $crate::assert_io_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_reader:expr, $b_matcher:expr, $($message:tt)+) => {{
        match $crate::assert_io_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a) ⇒ a) matches matcher
///
/// This macro provides the same statements as [`assert_io_read_to_string_matches`](macro.assert_io_read_to_string_matches.html),
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
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_io_read_to_string_matches`](macro@crate::assert_io_read_to_string_matches)
/// * [`assert_io_read_to_string_matches`](macro@crate::assert_io_read_to_string_matches)
/// * [`debug_assert_io_read_to_string_matches`](macro@crate::debug_assert_io_read_to_string_matches)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_matches {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_matches!($($arg)*);
        }
    };
}
