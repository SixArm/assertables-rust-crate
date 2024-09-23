//! Assert an expression (such as a string) starts with an expression (such as a string).
//!
//! Pseudocode:<br>
//! a.starts_with(b)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = "alfa";
//! let b = "al";
//! assert_starts_with!(a, b);
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
/// * Otherwise, return Result `Err` with a diagnostic message.
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
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if a.starts_with(b) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_starts_with!(a, b)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_starts_with.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_starts_with_as_result_x_success() {
        let a = "alfa";
        let b = "al";
        let result = assert_starts_with_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_starts_with_as_result_x_failure() {
        let a = "alfa";
        let b = "fa";
        let result = assert_starts_with_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_starts_with!(a, b)`\n",
            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_starts_with.html\n",
            " a label: `a`,\n",
            " a debug: `\"alfa\"`,\n",
            " b label: `b`,\n",
            " b debug: `\"fa\"`"
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
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = "alfa";
/// let b = "al";
/// assert_starts_with!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a = "alfa";
/// let b = "fa";
/// assert_starts_with!(a, b);
/// // assertion failed: `assert_starts_with!(a, b)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_starts_with.html
/// //  a label: `a`,
/// //  a debug: `\"alfa\"`,
/// //  b label: `b`,
/// //  b debug: `\"fa\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_starts_with!(a, b)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_starts_with.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"alfa\"`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"fa\"`"
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
    ($a:expr, $b:expr $(,)?) => {{
        match assert_starts_with_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match assert_starts_with_as_result!($a, $b) {
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
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
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
