//! Assert expression.is_err() is true.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Result<(), i8> = Result::Err(1);
//! assert_result_err!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_result_err`](macro@crate::assert_result_err)
//! * [`assert_result_err_as_result`](macro@crate::assert_result_err_as_result)
//! * [`debug_assert_result_err`](macro@crate::debug_assert_result_err)

/// Assert an expression.is_err() is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_result_err`](macro.assert_result_err.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_result_err`](macro@crate::assert_result_err)
/// * [`assert_result_err_as_result`](macro@crate::assert_result_err_as_result)
/// * [`debug_assert_result_err`](macro@crate::debug_assert_result_err)
///
#[macro_export]
macro_rules! assert_result_err_as_result {
    ($result:expr $(,)?) => {{
        match (&$result) {
            result_val => {
                let is_err = result_val.is_err();
                if is_err {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_result_err!(result)`\n",
                            "    result label: `{}`,\n",
                            "    result debug: `{:?}`,\n",
                            " result.is_err(): `{:?}`"
                        ),
                        stringify!($result),
                        $result,
                        is_err,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_result_err_as_result_x_success() {
        let a: Result<(), i8> = Result::Err(1);
        let result = assert_result_err_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_result_err_as_result_x_failure() {
        let a: Result<(), i8>  = Result::Ok(());
        let result = assert_result_err_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_result_err!(result)`\n",
                "    result label: `a`,\n",
                "    result debug: `Ok(())`,\n",
                " result.is_err(): `false`"
            )
        );
    }
}

/// Assert expression.is_err() is true.
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
/// let a: Result<(), i8> = Result::Err(1);
/// assert_result_err!(a);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Result<(), i8> = Result::Ok(());
/// assert_result_err!(a);
/// # });
/// // assertion failed: `assert_result_err!(result)`
/// //   result label: `a`,
/// //   result debug: `Ok(())`,
/// //  result.is_err(): `false`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_result_err!(result)`\n",
/// #     "    result label: `a`,\n",
/// #     "    result debug: `Ok(())`,\n",
/// #     " result.is_err(): `false`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_result_err`](macro@crate::assert_result_err)
/// * [`assert_result_err_as_result`](macro@crate::assert_result_err_as_result)
/// * [`debug_assert_result_err`](macro@crate::debug_assert_result_err)
///
#[macro_export]
macro_rules! assert_result_err {
    ($result:expr $(,)?) => ({
        match assert_result_err_as_result!($result) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($result:expr, $($message:tt)+) => ({
        match assert_result_err_as_result!($result) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert expression.is_err() is true.
///
/// This macro provides the same statements as [`assert_result_err`](macro.assert_result_err.html),
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
/// * [`assert_result_err`](macro@crate::assert_result_err)
/// * [`assert_result_err`](macro@crate::assert_result_err)
/// * [`debug_assert_result_err`](macro@crate::debug_assert_result_err)
///
#[macro_export]
macro_rules! debug_assert_result_err {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_result_err!($($arg)*);
        }
    };
}
