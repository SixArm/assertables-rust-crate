//! Assert expression is ok, and its value is equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Result<i8, i8> = Result::Ok(1);
//! let b: Result<i8, i8> = Result::Ok(2);
//! assert_result_ok_ne!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_result_ok_ne`](macro@crate::assert_result_ok_ne)
//! * [`assert_result_ok_ne_as_result`](macro@crate::assert_result_ok_ne_as_result)
//! * [`debug_assert_result_ok_ne`](macro@crate::debug_assert_result_ok_ne)

/// Assert expression is ok, and its value is equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_result_ok_ne`](macro.assert_result_ok_ne.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_result_ok_ne`](macro@crate::assert_result_ok_ne)
/// * [`assert_result_ok_ne_as_result`](macro@crate::assert_result_ok_ne_as_result)
/// * [`debug_assert_result_ok_ne`](macro@crate::debug_assert_result_ok_ne)
///
#[macro_export]
macro_rules! assert_result_ok_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a_result, b_result) => {
                if a_result.is_err() || b_result.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_result_ok_ne!(a, b)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                    ))
                } else {
                    let a_ok = a_result.unwrap();
                    let b_ok = b_result.unwrap();
                    if a_ok != b_ok {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_result_ok_ne!(a, b)`\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "    a ok: `{:?}`,\n",
                                "    b ok: `{:?}`"
                            ),
                            stringify!($a),
                            $a,
                            stringify!($b),
                            $b,
                            a_ok,
                            b_ok
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
    fn test_assert_result_ok_ne_as_result_x_success() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Ok(2);
        let result = assert_result_ok_ne_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_result_ok_ne_as_result_x_failure_because_ok_eq() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Ok(1);
        let result = assert_result_ok_ne_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_result_ok_ne!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ok(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Ok(1)`,\n",
                "    a ok: `1`,\n",
                "    b ok: `1`",
            )
        );
    }

    #[test]
    fn test_assert_result_ok_ne_as_result_x_failure_because_err() {
        let a: Result<i8, i8> = Result::Ok(1);
        let b: Result<i8, i8> = Result::Err(1);
        let result = assert_result_ok_ne_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_result_ok_ne!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ok(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Err(1)`",
            )
        );
    }

}

/// Assert expression is ok, and its value is equal to another.
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
/// let b: Result<i8, i8> = Result::Ok(2);
/// assert_result_ok_ne!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let a: Result<i8, i8> = Result::Ok(1);
/// let b: Result<i8, i8> = Result::Ok(1);
/// let result = panic::catch_unwind(|| {
/// assert_result_ok_ne!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_result_ok_ne!(a, b)`\n",
///     " a label: `a`,\n",
///     " a debug: `Ok(1)`,\n",
///     " b label: `b`,\n",
///     " b debug: `Ok(1)`,\n",
///     "    a ok: `1`,\n",
///     "    b ok: `1`",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_result_ok_ne!(a, b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_result_ok_ne`](macro@crate::assert_result_ok_ne)
/// * [`assert_result_ok_ne_as_result`](macro@crate::assert_result_ok_ne_as_result)
/// * [`debug_assert_result_ok_ne`](macro@crate::debug_assert_result_ok_ne)
///
#[macro_export]
macro_rules! assert_result_ok_ne {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_result_ok_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_result_ok_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert expression is ok, and its value is equal to another.
///
/// This macro provides the same statements as [`assert_result_ok_ne`](macro.assert_result_ok_ne.html),
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
/// * [`assert_result_ok_ne`](macro@crate::assert_result_ok_ne)
/// * [`assert_result_ok_ne`](macro@crate::assert_result_ok_ne)
/// * [`debug_assert_result_ok_ne`](macro@crate::debug_assert_result_ok_ne)
///
#[macro_export]
macro_rules! debug_assert_result_ok_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_result_ok_ne!($($arg)*);
        }
    };
}
