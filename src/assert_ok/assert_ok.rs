//! Assert expression is Ok(_).
//!
//! Pseudocode:<br>
//! a is Ok(_)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: Result<(), i8> = Ok(());
//! assert_ok!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_ok`](macro@crate::assert_ok)
//! * [`assert_ok_as_result`](macro@crate::assert_ok_as_result)
//! * [`debug_assert_ok`](macro@crate::debug_assert_ok)

/// Assert expression is Ok(_).
///
/// Pseudocode:<br>
/// a is Ok(_)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_ok`](macro.assert_ok.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ok`](macro@crate::assert_ok)
/// * [`assert_ok_as_result`](macro@crate::assert_ok_as_result)
/// * [`debug_assert_ok`](macro@crate::debug_assert_ok)
///
#[macro_export]
macro_rules! assert_ok_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a => {
                match (a) {
                    Ok(_) => {
                        Ok(())
                    },
                    _ => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_ok!(a)`\n",
                                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_ok.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`",
                            ),
                            stringify!($a),
                            a
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_ok_as_result_x_success() {
        let a: Result<(), i8> = Ok(());
        let result = assert_ok_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_ok_as_result_x_failure() {
        let a: Result<(), i8> = Err(1);
        let result = assert_ok_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_ok!(a)`\n",
                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_ok.html\n",
                " a label: `a`,\n",
                " a debug: `Err(1)`",
            )
        );
    }
}

/// Assert expression is Ok(_).
///
/// Pseudocode:<br>
/// a is Ok(_)
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
/// let a: Result<(), i8> = Ok(());
/// assert_ok!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Result<(), i8> = Err(1);
/// assert_ok!(a);
/// # });
/// // assertion failed: `assert_ok!(a)`
/// // https://docs.rs/assertables/8.16.0/assertables/macro.assert_ok.html
/// //  a label: `a`,
/// //  a debug: `Err(1)`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_ok!(a)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_ok.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Err(1)`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ok`](macro@crate::assert_ok)
/// * [`assert_ok_as_result`](macro@crate::assert_ok_as_result)
/// * [`debug_assert_ok`](macro@crate::debug_assert_ok)
///
#[macro_export]
macro_rules! assert_ok {
    ($a:expr $(,)?) => {{
        match $crate::assert_ok_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $($message:tt)+) => {{
        match $crate::assert_ok_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert expression is Ok(_).
///
/// Pseudocode:<br>
/// a is Ok(_)
///
/// This macro provides the same statements as [`assert_ok`](macro.assert_ok.html),
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
/// * [`assert_ok`](macro@crate::assert_ok)
/// * [`assert_ok`](macro@crate::assert_ok)
/// * [`debug_assert_ok`](macro@crate::debug_assert_ok)
///
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_ok!($($arg)*);
        }
    };
}
