//! Assert a number is approximately not equal to another number.
//!
//! Pseudocode:<br>
//! | a - b | > 1e-6
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: f32 = 1.0000001;
//! let b: f32 = 1.0000012;
//! assert_approx_ne!(a, b);
//! # }
//! ```
//!
//!
//! ## Comparisons
//!
//! This crate provides macro groups that test approximations and nearness:
//!
//! * [`assert_approx_eq`](macro@crate::assert_approx_eq) and
//!   [`assert_approx_ne`](macro@crate::assert_approx_ne) test the approximate
//!   equality within 1e-6. The macro name and the approximate value are chosen
//!   to be similar to the longtime popular rust crate `assert_approx_eq`.
//!
//! * [`assert_in_delta`](macro@crate::assert_in_delta) tests the absolute error
//!   (i.e. delta). This is the magnitude of the difference between the exact
//!   value and the approximation.
//!
//! * [`assert_in_epsilon`](macro@crate::assert_in_epsilon) tests the relative
//!   error (i.e. epsilon). This is the absolute error divided by the magnitude
//!   of the exact value. This can be used to compare approximations of numbers
//!   of wildly differing size.
//!
//! Examples:
//!
//! * Approximating the number 100 and 103 has an absolute error (approx) of 3
//!   and a relative error (epsilon) of 0.03.
//!
//! * Approximating the number 1,000,000 and 1,000,003 has an absolute error
//!   (approx) of 3, and a relative error (epsilon) of 0.000003.
//!
//! * For many kinds of applications, the relative error is more important than
//!   the absolute error.
//!
//!
//! ## Thanks
//!
//! * Thanks to [Ashley Williams](https://github.com/ashleygwilliams) for
//!   creating and maintaining the `assert_approx_eq` crate.
//!
//! * Thanks to [Ryan Davis](https://github.com/zenspider) and Ruby minitest for
//!   creating and maintaining `assert_in_approx` and `assert_in_epsilon` code.
//!
//!
//! # Module macros
//!
//! * [`assert_approx_ne`](macro@crate::assert_approx_ne)
//! * [`assert_approx_ne_as_result`](macro@crate::assert_approx_ne_as_result)
//! * [`debug_assert_approx_ne`](macro@crate::debug_assert_approx_ne)

/// Assert a number is approximately not equal to another number.
///
/// Pseudocode:<br>
/// | a - b | > 1e-6
///
/// * If true, return Result `Ok(abs_diff, approx)`.
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
/// * [`assert_approx_ne`](macro@crate::assert_approx_ne)
/// * [`assert_approx_ne_as_result`](macro@crate::assert_approx_ne_as_result)
/// * [`debug_assert_approx_ne`](macro@crate::debug_assert_approx_ne)
///
#[macro_export]
macro_rules! assert_approx_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                let approx = 1e-6;
                let abs_diff = if (a >= b) { a - b } else { b - a };
                if abs_diff > approx {
                    Ok((abs_diff, approx))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_approx_ne!(a, b)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_approx_ne.html\n",
                                "            a label: `{}`,\n",
                                "            a debug: `{:?}`,\n",
                                "            b label: `{}`,\n",
                                "            b debug: `{:?}`,\n",
                                "          | a - b |: `{:?}`,\n",
                                "             approx: `{:?}`,\n",
                                " | a - b | > approx: false"
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b,
                            abs_diff,
                            approx
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
    fn ne() {
        let a: f32 = 1.0000001;
        let b: f32 = 1.0000012;
        let result = assert_approx_ne_as_result!(a, b);
        assert_eq!(result.unwrap(), (1.0728836e-6, 1e-6));
    }

    #[test]
    fn eq() {
        let a: f32 = 1.0000001;
        let b: f32 = 1.0000011;
        let result = assert_approx_ne_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_approx_ne!(a, b)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_approx_ne.html\n",
                "            a label: `a`,\n",
                "            a debug: `1.0000001`,\n",
                "            b label: `b`,\n",
                "            b debug: `1.0000011`,\n",
                "          | a - b |: `9.536743e-7`,\n",
                "             approx: `1e-6`,\n",
                " | a - b | > approx: false"
            )
        );
    }
}

/// Assert a number is approximately not equal to another number.
///
/// Pseudocode:<br>
/// | a - b | > 1e-6
///
/// * If true, return `(diff, approx)`.
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
/// let a: f32 = 1.0000001;
/// let b: f32 = 1.0000012;
/// assert_approx_ne!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: f32 = 1.0000001;
/// let b: f32 = 1.0000011;
/// assert_approx_ne!(a, b);
/// # });
/// // assertion failed: `assert_approx_ne!(a, b)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_approx_ne.html
/// //             a label: `a`,
/// //             a debug: `1.0000001`,
/// //             b label: `b`,
/// //             b debug: `1.0000011`,
/// //           | a - b |: `9.536743e-7`,
/// //              approx: `1e-6`,
/// //  | a - b | > approx: false
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_approx_ne!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_approx_ne.html\n",
/// #     "            a label: `a`,\n",
/// #     "            a debug: `1.0000001`,\n",
/// #     "            b label: `b`,\n",
/// #     "            b debug: `1.0000011`,\n",
/// #     "          | a - b |: `9.536743e-7`,\n",
/// #     "             approx: `1e-6`,\n",
/// #     " | a - b | > approx: false",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// The macros `assert_approx_ne` and `assert_in_epsilon` can test
/// approximations:
///
/// * For an approximation, the absolute error (i.e. approx) is the magnitude of
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
/// * [`assert_approx_ne`](macro@crate::assert_approx_ne)
/// * [`assert_approx_ne_as_result`](macro@crate::assert_approx_ne_as_result)
/// * [`debug_assert_approx_ne`](macro@crate::debug_assert_approx_ne)
///
#[macro_export]
macro_rules! assert_approx_ne {
    ($a:expr, $b:expr $(,)?) => {{
        match $crate::assert_approx_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_approx_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a number is approximately not equal to another number.
///
/// Pseudocode:<br>
/// | a - b | > 1e-6
///
/// This macro provides the same statements as [`assert_approx_ne`](macro.assert_approx_ne.html),
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
/// * [`assert_approx_ne`](macro@crate::assert_approx_ne)
/// * [`assert_approx_ne`](macro@crate::assert_approx_ne)
/// * [`debug_assert_approx_ne`](macro@crate::debug_assert_approx_ne)
///
#[macro_export]
macro_rules! debug_assert_approx_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_approx_ne!($($arg)*);
        }
    };
}
