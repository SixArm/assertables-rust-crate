//! Assert expression matches a case.
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 'a';
//! assert_not_matches!(a, 'b'..='z');
//! ```
//!
//! Note: this implementation of `assert_not_matches` is relatively basic.
//!
//! * If you want more capabilities, consider the crate `assert_not_matches`.
//!
//! * If you're using Rust nightly, use the std lib macro `assert_not_matches`.
//!
//! # Module macros
//!
//! * [`assert_not_matches`](macro@crate::assert_not_matches)
//! * [`assert_not_matches_as_result`](macro@crate::assert_not_matches_as_result)
//! * [`debug_assert_not_matches`](macro@crate::debug_assert_not_matches)

/// Assert expression matches a case.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_not_matches`](macro@crate::assert_not_matches)
/// * [`assert_not_matches_as_result`](macro@crate::assert_not_matches_as_result)
/// * [`debug_assert_not_matches`](macro@crate::debug_assert_not_matches)
///
#[macro_export]
macro_rules! assert_not_matches_as_result {
    ($($arg:tt)*) => {
        if !matches!($($arg)*) {
            Ok(())
        } else {
            Err(
                format!(
                    concat!(
                        "assertion failed: `assert_not_matches!(a)`\n",
                        "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
                        " args: `{}`",
                    ),
                    stringify!($($arg)*)
                )
            )
        }
    };
}

#[cfg(test)]
mod test_assert_not_matches_as_result {
    // use std::sync::Once;

    //// Use char as per https://doc.rust-lang.org/std/macro.matches.html
    mod use_char {

        #[test]
        fn success() {
            let a = 'a';
            let actual = assert_not_matches_as_result!(a, 'b'..='z');
            assert_eq!(actual.unwrap(), ());
        }

        #[test]
        fn failure() {
            let a = 'a';
            let actual = assert_not_matches_as_result!(a, 'a'..='z');
            let message = concat!(
                "assertion failed: `assert_not_matches!(a)`\n",
                "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
                " args: `a, 'a'..='z'`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    //// Use Some as per  https://doc.rust-lang.org/std/macro.matches.html
    mod use_some {

        #[test]
        fn success() {
            let a = Some(2);
            let actual = assert_not_matches_as_result!(a, Some(x) if x < 2);
            assert_eq!(actual.unwrap(), ());
        }

        #[test]
        fn failure() {
            let a = Some(1);
            let actual = assert_not_matches_as_result!(a, Some(x) if x < 2);
            let message = concat!(
                "assertion failed: `assert_not_matches!(a)`\n",
                "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
                " args: `a, Some(x) if x < 2`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
}

/// Assert expression is Some.
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
///
/// # fn main() {
/// let a = 'a';
/// assert_not_matches!(a, 'b'..='z');
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 'a';
/// assert_not_matches!(a, 'a'..='z');
/// # });
/// // assertion failed: `assert_not_matches!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_not_matches.html
/// //  args: `a, 'a'..='z'`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_not_matches!(a)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
/// #     " args: `a, 'a'..='z'`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_not_matches`](macro@crate::assert_not_matches)
/// * [`assert_not_matches_as_result`](macro@crate::assert_not_matches_as_result)
/// * [`debug_assert_not_matches`](macro@crate::debug_assert_not_matches)
///
#[macro_export]
macro_rules! assert_not_matches {
    ($expression:expr, $pattern:pat if $guard:expr $(,)?) => {
        match $crate::assert_not_matches_as_result!($expression, $pattern if $guard) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($expression:expr, $pattern:pat) => {
        match $crate::assert_not_matches_as_result!($expression, $pattern) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($expression:expr, $pattern:pat if $guard:expr, $($message:tt)+) => {
        match $crate::assert_not_matches_as_result!($expression, $pattern if $guard) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
    ($expression:expr, $pattern:pat, $($message:tt)+) => {
        match $crate::assert_not_matches_as_result!($expression, $pattern if $guard) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_not_matches {

    //// Use char as per https://doc.rust-lang.org/std/macro.matches.html
    mod use_char {
        use std::panic;

        #[test]
        fn success() {
            let a = 'a';
            let actual = assert_not_matches!(a, 'b'..='z');
            assert_eq!(actual, ());
        }

        #[test]
        fn failure() {
            let a = 'a';
            let result = panic::catch_unwind(|| {
                let _actual = assert_not_matches!(a, 'a'..='z');
            });
            let message = concat!(
                "assertion failed: `assert_not_matches!(a)`\n",
                "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
                " args: `a, 'a'..='z'`",
            );
            assert_eq!(
                result
                    .unwrap_err()
                    .downcast::<String>()
                    .unwrap()
                    .to_string(),
                message
            );
        }
    }

    //// Use Some as per  https://doc.rust-lang.org/std/macro.matches.html
    mod use_some {
        use std::panic;

        #[test]
        fn success() {
            let a = Some(2);
            let actual = assert_not_matches!(a, Some(x) if x < 2);
            assert_eq!(actual, ());
        }

        #[test]
        fn failure() {
            let a = Some(1);
            let result = panic::catch_unwind(|| {
                let _actual = assert_not_matches!(a, Some(x) if x < 2);
            });
            let message = concat!(
                "assertion failed: `assert_not_matches!(a)`\n",
                "https://docs.rs/assertables/9.8.1/assertables/macro.assert_not_matches.html\n",
                " args: `a, Some(x) if x < 2`",
            );
            assert_eq!(
                result
                    .unwrap_err()
                    .downcast::<String>()
                    .unwrap()
                    .to_string(),
                message
            );
        }
    }
}

/// Assert expression is Some.
///
/// This macro provides the same statements as [`assert_not_matches`](macro.assert_not_matches.html),
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
/// * [`assert_not_matches`](macro@crate::assert_not_matches)
/// * [`assert_not_matches`](macro@crate::assert_not_matches)
/// * [`debug_assert_not_matches`](macro@crate::debug_assert_not_matches)
///
#[macro_export]
macro_rules! debug_assert_not_matches {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_not_matches!($($arg)*);
        }
    };
}
