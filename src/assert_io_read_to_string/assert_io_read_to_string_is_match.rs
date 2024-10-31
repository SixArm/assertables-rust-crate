//! Assert a ::std::io::Read read_to_string() is a match to a regex.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a_string) ⇒ a_string) matches matcher
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
//! assert_io_read_to_string_is_match!(reader, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
//! * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
//! * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// * If true, return Result `Ok((a_string, b_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_io_read_to_string_is_match_as_result {
    ($reader:expr, $matcher:expr $(,)?) => {{
        match (/*&$reader,*/ &$matcher) {
            matcher => {
                let mut string = String::new();
                match ($reader.read_to_string(&mut string)) {
                    Ok(size) => {
                        if $matcher.is_match(&string) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
                                        "  reader label: `{}`,\n",
                                        "  reader debug: `{:?}`,\n",
                                        " matcher label: `{}`,\n",
                                        " matcher debug: `{:?}`,\n",
                                        "   reader size: `{:?}`,\n",
                                        " reader string: `{:?}`",
                                    ),
                                    stringify!($reader),
                                    $reader,
                                    stringify!($matcher),
                                    matcher,
                                    size,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
                                    "  reader label: `{}`,\n",
                                    "  reader debug: `{:?}`,\n",
                                    " matcher label: `{}`,\n",
                                    " matcher debug: `{:?}`,\n",
                                    "           err: `{:?}`"
                                ),
                                stringify!($reader),
                                $reader,
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
    use std::io::Read;

    #[test]
    fn test_assert_io_read_to_string_is_match_as_result_x_success() {
        let mut reader = "alfa".as_bytes();
        let matcher = Regex::new(r"alfa").unwrap();
        let result = assert_io_read_to_string_is_match_as_result!(reader, &matcher);
        assert_eq!(result.unwrap(), String::from("alfa"));
    }

    #[test]
    fn test_assert_io_read_to_string_is_match_as_result_x_failure() {
        let mut reader = "alfa".as_bytes();
        let matcher = Regex::new(r"zz").unwrap();
        let result = assert_io_read_to_string_is_match_as_result!(reader, &matcher);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
                "  reader label: `reader`,\n",
                "  reader debug: `[]`,\n",
                " matcher label: `&matcher`,\n",
                " matcher debug: `Regex(\"zz\")`,\n",
                "   reader size: `4`,\n",
                " reader string: `\"alfa\"`"
            )
        );
    }
}

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// * If true, return `(a_string, b_string)`.
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
/// assert_io_read_to_string_is_match!(reader, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"zz").unwrap();
/// assert_io_read_to_string_is_match!(reader, &matcher);
/// # });
/// // assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_is_match.html
/// //   reader label: `reader`,
/// //   reader debug: `[]`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zz\")`,
/// //    reader size: `5`
/// //  reader string: `\"hello\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
/// #     "  reader label: `reader`,\n",
/// #     "  reader debug: `[]`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zz\")`,\n",
/// #     "   reader size: `5`,\n",
/// #     " reader string: `\"hello\"`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_io_read_to_string_is_match {
    ($a_reader:expr, $b_matcher:expr $(,)?) => {{
        match $crate::assert_io_read_to_string_is_match_as_result!($a_reader, $b_matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_reader:expr, $b_matcher:expr, $($message:tt)+) => {{
        match $crate::assert_io_read_to_string_is_match_as_result!($a_reader, $b_matcher) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// This macro provides the same statements as [`assert_io_read_to_string_is_match`](macro.assert_io_read_to_string_is_match.html),
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
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_is_match!($($arg)*);
        }
    };
}
