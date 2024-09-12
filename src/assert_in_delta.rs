//! Assert a number is within delta of another number.
//!
//! Calculate | a - b | ≤ delta
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: i8 = 10;
//! let b: i8 = 11;
//! let delta: i8 = 1;
//! assert_in_delta!(a, b, delta);
//! # }
//! ```
//!
//! The macros `assert_in_delta` and `assert_in_epsilon` can test
//! approximations:
//!
//! * For an approximation, the absolute error (i.e. delta) is the magnitude of
//!   the difference between the exact value and the approximation. For this,
//!  use the macro
//!
//! * For an approximation, the relative error (i.e. epsilon) is the absolute
//!   error divided by the magnitude of the exact value. This can be used to
//!   compare approximations of numbers of wildly differing size.
//!
//! * For example, approximating the number 1,000 with an absolute error of 3
//!   is, in most applications, much worse than approximating the number
//!   1,000,000 with an absolute error of 3; in the first case the relative
//!   error is 0.003 and in the second it is only 0.000003.
//!
//! * Thanks to Ruby minitest for the example and documentation.
//!
//! # Module macros
//!
//! * [`assert_in_delta`](macro@crate::assert_in_delta)
//! * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
//! * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)

/// Assert a number is within delta of another number.
///
/// Calculate | a - b | ≤ delta
///
/// * If true, return Result `Ok(())`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html), except this macro
/// returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters, or
/// sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! assert_in_delta_as_result {
    ($a:expr, $b:expr, $delta:expr $(,)?) => {{
        match (&$a, &$b, &$delta) {
            (a_val, b_val, delta_val) => {
                if a_val == b_val
                    || ((a_val < b_val) && (b_val - a_val) <= *delta_val)
                    || ((a_val > b_val) && (a_val - b_val) <= *delta_val)
                {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_in_delta!(a, b, delta)`\n",
                            "     a label: `{}`,\n",
                            "     a debug: `{:?}`,\n",
                            "     b label: `{}`,\n",
                            "     b debug: `{:?}`,\n",
                            " delta label: `{}`,\n",
                            " delta debug: `{:?}`,\n",
                            "     a value: `{:?}`,\n",
                            "     b value: `{:?}`,\n",
                            " delta value: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                        stringify!($delta),
                        $delta,
                        a_val,
                        b_val,
                        delta_val

                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_in_delta_as_result_x_success() {
        let a: i8 = 10;
        let b: i8 = 11;
        let delta: i8 = 1;
        let result = assert_in_delta_as_result!(a, b, delta);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_in_delta_as_result_x_failure() {
        let a: i8 = 10;
        let b: i8 = 12;
        let delta: i8 = 1;
        let result = assert_in_delta_as_result!(a, b, delta);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_in_delta!(a, b, delta)`\n",
                "     a label: `a`,\n",
                "     a debug: `10`,\n",
                "     b label: `b`,\n",
                "     b debug: `12`,\n",
                " delta label: `delta`,\n",
                " delta debug: `1`,\n",
                "     a value: `10`,\n",
                "     b value: `12`,\n",
                " delta value: `1`"
            )
        );
    }
}

/// Assert a number is within delta of another number.
///
/// Calculate | a - b | ≤ delta
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
/// let a: i8 = 10;
/// let b: i8 = 11;
/// let delta: i8 = 1;
/// assert_in_delta!(a, b, delta);
///
/// # let result = panic::catch_unwind(|| {
/// let a: i8 = 10;
/// let b: i8 = 12;
/// let delta: i8 = 1;
/// assert_in_delta!(a, b, delta);
/// # });
/// // assertion failed: `assert_in_delta!(a, b, delta)`
/// //      a label: `a`,
/// //      a debug: `10`,
/// //      b label: `b`,
/// //      b debug: `12`,
/// //  delta label: `delta`,
/// //  delta debug: `1`,
/// //      a value: `10`,
/// //      b value: `12`,
/// //  delta value: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_in_delta!(a, b, delta)`\n",
/// #     "     a label: `a`,\n",
/// #     "     a debug: `10`,\n",
/// #     "     b label: `b`,\n",
/// #     "     b debug: `12`,\n",
/// #     " delta label: `delta`,\n",
/// #     " delta debug: `1`,\n",
/// #     "     a value: `10`,\n",
/// #     "     b value: `12`,\n",
/// #     " delta value: `1`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// The macros `assert_in_delta` and `assert_in_epsilon` can test
/// approximations:
///
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation. For this,
///  use the macro
///
/// * For an approximation, the relative error (i.e. epsilon) is the absolute
///   error divided by the magnitude of the exact value. This can be used to
///   compare approximations of numbers of wildly differing size.
///
/// * For example, approximating the number 1,000 with an absolute error of 3
///   is, in most applications, much worse than approximating the number
///   1,000,000 with an absolute error of 3; in the first case the relative
///   error is 0.003 and in the second it is only 0.000003.
///
/// * Thanks to Ruby minitest for the example and documentation.
///
/// # Module macros
///
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! assert_in_delta {
    ($a:expr, $b:expr, $delta:expr $(,)?) => ({
        match assert_in_delta_as_result!($a, $b, $delta) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $delta:expr, $($message:tt)+) => ({
        match assert_in_delta_as_result!($a, $b, $delta) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a number is within delta of another number.
///
/// Calculate | a - b | ≤ delta
///
/// This macro provides the same statements as [`assert_in_delta`](macro.assert_in_delta.html),
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
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! debug_assert_in_delta {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_in_delta!($($arg)*);
        }
    };
}
