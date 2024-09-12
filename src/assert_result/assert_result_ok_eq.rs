//! Assert a.is_ok() and a.unwrap() are equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Result<i8, i8> = Result::Ok(1);
//! let b: Result<i8, i8> = Result::Ok(1);
//! assert_result_ok_eq!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_result_ok_eq`](macro@crate::assert_result_ok_eq)
//! * [`assert_result_ok_eq_as_result`](macro@crate::assert_result_ok_eq_as_result)
//! * [`debug_assert_result_ok_eq`](macro@crate::debug_assert_result_ok_eq)

/// Assert a.is_ok() and a.unwrap() are equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_result_ok_eq`](macro.assert_result_ok_eq.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_result_ok_eq`](macro@crate::assert_result_ok_eq)
/// * [`assert_result_ok_eq_as_result`](macro@crate::assert_result_ok_eq_as_result)
/// * [`debug_assert_result_ok_eq`](macro@crate::debug_assert_result_ok_eq)
///
#[macro_export]
macro_rules! assert_result_ok_eq_as_result {
    ($a_result:expr, $b_result:expr $(,)?) => {{
        match (&$a_result, &$b_result) {
            (a_result, b_result) => {
                if !a_result.is_ok() || !b_result.is_ok() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_result_ok_eq!(a, b)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a_result),
                        $a_result,
                        stringify!($b_result),
                        $b_result,
                    ))
                } else {
                    let a = a_result.unwrap();
                    let b = b_result.unwrap();
                    if a == b {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_result_ok_eq!(a, b)`\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_result),
                            $a_result,
                            stringify!($b_result),
                            $b_result,
                            a,
                            b
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
    fn test_assert_result_ok_eq_as_result_x_success() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Ok(1);
        let result = assert_result_ok_eq_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_result_ok_eq_as_result_x_failure_because_ne() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Ok(2);
        let result = assert_result_ok_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_result_ok_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ok(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Ok(2)`,\n",
                "       a: `1`,\n",
                "       b: `2`",
            )
        );
    }

    #[test]
    fn test_assert_result_ok_eq_as_result_x_failure_because_not_ok() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Err(1);
        let result = assert_result_ok_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_result_ok_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ok(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Err(1)`",
            )
        );
    }

}

/// Assert a.is_ok() and a.unwrap() are equal to another.
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
/// let a: Result<i8, i8> = Result::Ok(1);
/// let b: Result<i8, i8> = Result::Ok(1);
/// assert_result_ok_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Result<i8, i8> = Result::Ok(1);
/// let b: Result<i8, i8> = Result::Ok(2);
/// assert_result_ok_eq!(a, b);
/// # });
/// // assertion failed: `assert_result_ok_eq!(a, b)`
/// //  a label: `a`,
/// //  a debug: `Ok(1)`,
/// //  b label: `b`,
/// //  b debug: `Ok(2)`,
/// //        a: `1`,
/// //        b: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_result_ok_eq!(a, b)`\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ok(1)`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Ok(2)`,\n",
/// #     "       a: `1`,\n",
/// #     "       b: `2`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_result_ok_eq`](macro@crate::assert_result_ok_eq)
/// * [`assert_result_ok_eq_as_result`](macro@crate::assert_result_ok_eq_as_result)
/// * [`debug_assert_result_ok_eq`](macro@crate::debug_assert_result_ok_eq)
///
#[macro_export]
macro_rules! assert_result_ok_eq {
    ($a_result:expr, $b_result:expr $(,)?) => ({
        match assert_result_ok_eq_as_result!($a_result, $b_result) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_result:expr, $b_result:expr, $($message:tt)+) => ({
        match assert_result_ok_eq_as_result!($a_result, $b_result) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a.is_ok() and a.unwrap() are equal to another.
///
/// This macro provides the same statements as [`assert_result_ok_eq`](macro.assert_result_ok_eq.html),
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
/// * [`assert_result_ok_eq`](macro@crate::assert_result_ok_eq)
/// * [`assert_result_ok_eq`](macro@crate::assert_result_ok_eq)
/// * [`debug_assert_result_ok_eq`](macro@crate::debug_assert_result_ok_eq)
///
#[macro_export]
macro_rules! debug_assert_result_ok_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_result_ok_eq!($($arg)*);
        }
    };
}