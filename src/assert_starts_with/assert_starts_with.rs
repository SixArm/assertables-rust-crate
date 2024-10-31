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
//! # fn main() {
//! // String starts with substring?
//! let whole: &str = "alfa";
//! let part: &str = "al";
//! assert_starts_with!(sequence, x);
//!
//! // Vector starts with element?
//! let whole = vec![1, 2, 3];
//! let part = [1];
//! assert_starts_with!(sequence, x);
//! # }
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
/// This macro provides the same statements as [`assert_starts_with`](macro.assert_starts_with.html),
/// except this macro returns a Result, rather than doing a panic.
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
    ($whole:expr, $part:expr $(,)?) => {{
        match (&$whole, &$part) {
            (sequence, x) => {
                if sequence.starts_with(x) {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_starts_with!(sequence, x)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_starts_with.html\n",
                                " whole label: `{}`,\n",
                                " whole debug: `{:?}`,\n",
                                "  part label: `{}`,\n",
                                "  part debug: `{:?}`",
                            ),
                            stringify!($whole),
                            whole,
                            stringify!($part),
                            part,
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_starts_with_as_result_success() {
        let whole = "alfa";
        let part = "al";
        let result = assert_starts_with_as_result!(sequence, x);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_starts_with_as_result_x_failure() {
        let whole = "alfa";
        let part = "fa";
        let result = assert_starts_with_as_result!(sequence, x);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_starts_with!(sequence, x)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_starts_with.html\n",
            " whole label: `whole`,\n",
            " whole debug: `\"alfa\"`,\n",
            "  part label: `part`,\n",
            "  part debug: `\"fa\"`"
        );
        assert_eq!(actual, expect);
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
/// let whole: &str = "alfa";
/// let part: &str = "al";
/// assert_starts_with!(sequence, x);
///
/// // Vector starts with element?
/// let whole = vec![1, 2, 3];
/// let part = [1];
/// assert_starts_with!(sequence, x);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let whole = "alfa";
/// let part = "fa";
/// assert_starts_with!(sequence, x);
/// // assertion failed: `assert_starts_with!(sequence, x)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_starts_with.html
/// //  whole label: `whole`,
/// //  whole debug: `\"alfa\"`,
/// //   part label: `part`,
/// //   part debug: `\"fa\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_starts_with!(sequence, x)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_starts_with.html\n",
/// #     " whole label: `whole`,\n",
/// #     " whole debug: `\"alfa\"`,\n",
/// #     "  part label: `part`,\n",
/// #     "  part debug: `\"fa\"`"
/// # );
/// # assert_eq!(actual, expect);
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
    ($whole:expr, $part:expr $(,)?) => {{
        match $crate::assert_starts_with_as_result!($whole, $part) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($whole:expr, $part:expr, $($message:tt)+) => {{
        match $crate::assert_starts_with_as_result!($whole, $part) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
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
