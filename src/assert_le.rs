//! Assert a value is less than or equal to an expression.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = 1;
//! let b = 2;
//! assert_le!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_le`](macro@crate::assert_le)
//! * [`assert_le_as_result`](macro@crate::assert_le_as_result)
//! * [`debug_assert_le`](macro@crate::debug_assert_le)

/// Assert a value is less than or equal to an expression.
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
/// * [`assert_le`](macro@crate::assert_le)
/// * [`assert_le_as_result`](macro@crate::assert_le_as_result)
/// * [`debug_assert_le`](macro@crate::debug_assert_le)
///
#[macro_export]
macro_rules! assert_le_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a, b) => {
                if a <= b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_le!(a, b)`\n",
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
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_le_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let result = assert_le_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_le_as_result_x_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let result = assert_le_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_le!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `2`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
            )
        );
    }
}

/// Assert a value is less than or equal to an expression.
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
/// let a = 1;
/// let b = 2;
/// assert_le!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a = 2;
/// let b = 1;
/// assert_le!(a, b);
/// # });
/// // assertion failed: `assert_le!(a, b)`
/// //  a label: `a`,
/// //  a debug: `2`,
/// //  b label: `b`,
/// //  b debug: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_le!(a, b)`\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `2`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `1`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_le`](macro@crate::assert_le)
/// * [`assert_le_as_result`](macro@crate::assert_le_as_result)
/// * [`debug_assert_le`](macro@crate::debug_assert_le)
///
#[macro_export]
macro_rules! assert_le {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_le_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_le_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a value is less than or equal to an expression.
///
/// This macro provides the same statements as [`assert_le`](macro.assert_le.html),
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
/// * [`assert_le`](macro@crate::assert_le)
/// * [`assert_le`](macro@crate::assert_le)
/// * [`debug_assert_le`](macro@crate::debug_assert_le)
///
#[macro_export]
macro_rules! debug_assert_le {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_le!($($arg)*);
        }
    };
}
