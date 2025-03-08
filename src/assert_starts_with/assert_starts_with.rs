//! Assert an expression (such as a string) starts with an expression (such as a string).
//!
//! Pseudocode:<br>
//! a.starts_with(b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! // String starts with substring?
//! let sequence: &str = "alfa";
//! let subsequence: &str = "al";
//! assert_starts_with!(sequence, subsequence);
//!
//! // Vector starts with element?
//! let sequence = vec![1, 2, 3];
//! let subsequence = [1];
//! assert_starts_with!(sequence, subsequence);
//! ```
//!
//! # Module macros
//!
//! * [`assert_starts_with`](macro@crate::assert_starts_with)
//! * [`assert_starts_with_as_result`](macro@crate::assert_starts_with_as_result)
//! * [`debug_assert_starts_with`](macro@crate::debug_assert_starts_with)

/// Assert an expression (such as a string) starts with an expression (such as a substring).
///
/// Pseudocode:<br>
/// a.starts_with(b)
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
/// * [`assert_starts_with`](macro@crate::assert_starts_with)
/// * [`assert_starts_with_as_result`](macro@crate::assert_starts_with_as_result)
/// * [`debug_assert_starts_with`](macro@crate::debug_assert_starts_with)
///
#[macro_export]
macro_rules! assert_starts_with_as_result {
    ($sequence:expr, $subsequence:expr $(,)?) => {{
        match (&$sequence, &$subsequence) {
            (sequence, subsequence) => {
                if sequence.starts_with(subsequence) {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_starts_with!(sequence, subsequence)`\n",
                                "https://docs.rs/assertables/9.5.1/assertables/macro.assert_starts_with.html\n",
                                "     sequence label: `{}`,\n",
                                "     sequence debug: `{:?}`,\n",
                                "  subsequence label: `{}`,\n",
                                "  subsequence debug: `{:?}`",
                            ),
                            stringify!($sequence),
                            sequence,
                            stringify!($subsequence),
                            subsequence
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_starts_with_as_result {

    #[test]
    fn success() {
        let sequence = "alfa";
        let subsequence = "al";
        let actual = assert_starts_with_as_result!(sequence, subsequence);
        assert_eq!(actual.unwrap(), ());
    }

    #[test]
    fn failure() {
        let sequence = "alfa";
        let subsequence = "fa";
        let actual = assert_starts_with_as_result!(sequence, subsequence);
        let message = concat!(
            "assertion failed: `assert_starts_with!(sequence, subsequence)`\n",
            "https://docs.rs/assertables/9.5.1/assertables/macro.assert_starts_with.html\n",
            "     sequence label: `sequence`,\n",
            "     sequence debug: `\"alfa\"`,\n",
            "  subsequence label: `subsequence`,\n",
            "  subsequence debug: `\"fa\"`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an expression (such as a string) starts with an expression (such as a string).
///
/// Pseudocode:<br>
/// a.starts_with(b)
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
/// // String starts with substring?
/// let sequence: &str = "alfa";
/// let subsequence: &str = "al";
/// assert_starts_with!(sequence, subsequence);
///
/// // Vector starts with element?
/// let sequence = vec![1, 2, 3];
/// let subsequence = [1];
/// assert_starts_with!(sequence, subsequence);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let sequence = "alfa";
/// let subsequence = "fa";
/// assert_starts_with!(sequence, subsequence);
/// // assertion failed: `assert_starts_with!(sequence, subsequence)`
/// // https://docs.rs/assertables/9.5.1/assertables/macro.assert_starts_with.html
/// //  sequence label: `sequence`,
/// //  sequence debug: `\"alfa\"`,
/// //   part label: `subsequence`,
/// //   part debug: `\"fa\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_starts_with!(sequence, subsequence)`\n",
/// #     "https://docs.rs/assertables/9.5.1/assertables/macro.assert_starts_with.html\n",
/// #     "     sequence label: `sequence`,\n",
/// #     "     sequence debug: `\"alfa\"`,\n",
/// #     "  subsequence label: `subsequence`,\n",
/// #     "  subsequence debug: `\"fa\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_starts_with`](macro@crate::assert_starts_with)
/// * [`assert_starts_with_as_result`](macro@crate::assert_starts_with_as_result)
/// * [`debug_assert_starts_with`](macro@crate::debug_assert_starts_with)
///
#[macro_export]
macro_rules! assert_starts_with {
    ($sequence:expr, $subsequence:expr $(,)?) => {{
        match $crate::assert_starts_with_as_result!($sequence, $subsequence) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($sequence:expr, $subsequence:expr, $($message:tt)+) => {{
        match $crate::assert_starts_with_as_result!($sequence, $subsequence) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_starts_with {
    use std::panic;

    #[test]
    fn success() {
        let sequence = "alfa";
        let subsequence = "al";
        let actual = assert_starts_with!(sequence, subsequence);
        assert_eq!(actual, ());
    }

    #[test]
    fn failure() {
        let sequence = "alfa";
        let subsequence = "fa";
        let result = panic::catch_unwind(|| {
            let _actual = assert_starts_with!(sequence, subsequence);
        });
        let message = concat!(
            "assertion failed: `assert_starts_with!(sequence, subsequence)`\n",
            "https://docs.rs/assertables/9.5.1/assertables/macro.assert_starts_with.html\n",
            "     sequence label: `sequence`,\n",
            "     sequence debug: `\"alfa\"`,\n",
            "  subsequence label: `subsequence`,\n",
            "  subsequence debug: `\"fa\"`"
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

/// Assert an expression (such as a string) starts with an expression (such as a string).
///
/// Pseudocode:<br>
/// a.starts_with(b)
///
/// This macro provides the same statements as [`assert_starts_with`](macro.assert_starts_with.html),
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
/// * [`assert_starts_with`](macro@crate::assert_starts_with)
/// * [`assert_starts_with`](macro@crate::assert_starts_with)
/// * [`debug_assert_starts_with`](macro@crate::debug_assert_starts_with)
///
#[macro_export]
macro_rules! debug_assert_starts_with {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_starts_with!($($arg)*);
        }
    };
}
