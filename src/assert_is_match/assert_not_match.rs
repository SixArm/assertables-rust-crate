//! Assert an expression (such as a regex) is not a match for an expression (such as a string).
//!
//! Pseudocode:<br>
//! ¬ a.is_match(b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! # fn main() {
//! let a = Regex::new(r"lf").unwrap();
//! let b = "bravo";
//! assert_not_match!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_not_match`](macro@crate::assert_not_match)
//! * [`assert_not_match_as_result`](macro@crate::assert_not_match_as_result)
//! * [`debug_assert_not_match`](macro@crate::debug_assert_not_match)

/// Assert an expression (such as a regex) is not a match for an expression (such as a string).
///
/// Pseudocode:<br>
/// ¬ a.is_match(b)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_not_match`](macro.assert_not_match.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_not_match`](macro@crate::assert_not_match)
/// * [`assert_not_match_as_result`](macro@crate::assert_not_match_as_result)
/// * [`debug_assert_not_match`](macro@crate::debug_assert_not_match)
///
#[macro_export]
macro_rules! assert_not_match_as_result {
    ($matcher:expr, $matchee:expr $(,)?) => {{
        match (&$matcher, &$matchee) {
            (matcher, matchee) => {
                if !($matcher.is_match($matchee)) {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_not_match!(matcher, matchee)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_not_match.html\n",
                                " matcher label: `{}`,\n",
                                " matcher debug: `{:?}`,\n",
                                " matchee label: `{}`,\n",
                                " matchee debug: `{:?}`",
                            ),
                            stringify!($matcher),
                            matcher,
                            stringify!($matchee),
                            matchee,
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use regex::Regex;

    #[test]
    fn test_assert_not_match_as_result_x_success() {
        let a = Regex::new(r"lf").unwrap();
        let b = "bravo";
        let result = assert_not_match_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_not_match_as_result_x_failure() {
        let a = Regex::new(r"lf").unwrap();
        let b = "alfa";
        let result = assert_not_match_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_not_match!(matcher, matchee)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_not_match.html\n",
            " matcher label: `a`,\n",
            " matcher debug: `Regex(\"lf\")`,\n",
            " matchee label: `b`,\n",
            " matchee debug: `\"alfa\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert an expression (such as a regex) is not a match for an expression (such as a string).
///
/// Pseudocode:<br>
/// ¬ a.is_match(b)
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
/// use regex::Regex;
///
/// # fn main() {
/// let a = Regex::new(r"lf").unwrap();
/// let b = "bravo";
/// assert_not_match!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = Regex::new(r"lf").unwrap();
/// let b = "alfa";
/// assert_not_match!(a, b);
/// # });
/// // assertion failed: `assert_not_match!(matcher, matchee)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_not_match.html
/// //  matcher label: `a`,
/// //  matcher debug: `Regex(\"lf\")`,
/// //  matchee label: `b`,
/// //  matchee debug: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_not_match!(matcher, matchee)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_not_match.html\n",
/// #     " matcher label: `a`,\n",
/// #     " matcher debug: `Regex(\"lf\")`,\n",
/// #     " matchee label: `b`,\n",
/// #     " matchee debug: `\"alfa\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_not_match`](macro@crate::assert_not_match)
/// * [`assert_not_match_as_result`](macro@crate::assert_not_match_as_result)
/// * [`debug_assert_not_match`](macro@crate::debug_assert_not_match)
///
#[macro_export]
macro_rules! assert_not_match {
    ($matcher:expr, $matchee:expr $(,)?) => {{
        match $crate::assert_not_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($matcher:expr, $matchee:expr, $($message:tt)+) => {{
        match $crate::assert_not_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an expression (such as a regex) is not a match for an expression (such as a string).
///
/// Pseudocode:<br>
/// ¬ a.is_match(b)
///
/// This macro provides the same statements as [`assert_not_match`](macro.assert_not_match.html),
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
/// * [`assert_not_match`](macro@crate::assert_not_match)
/// * [`assert_not_match`](macro@crate::assert_not_match)
/// * [`debug_assert_not_match`](macro@crate::debug_assert_not_match)
///
#[macro_export]
macro_rules! debug_assert_not_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_not_match!($($arg)*);
        }
    };
}
