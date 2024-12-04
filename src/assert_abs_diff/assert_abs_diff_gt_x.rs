//! Assert an absolute difference is greater than an expression.
//!
//! Pseudocode:<br>
//! | a - b | > x
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 10;
//! let b = 13;
//! let x = 2;
//! assert_abs_diff_gt_x!(a, b, x);
//! ```
//!
//! # Module macros
//!
//! * [`assert_abs_diff_gt_x`](macro@crate::assert_abs_diff_gt_x)
//! * [`assert_abs_diff_gt_x_as_result`](macro@crate::assert_abs_diff_gt_x_as_result)
//! * [`debug_assert_abs_diff_gt_x`](macro@crate::debug_assert_abs_diff_gt_x)

/// Assert an absolute difference is greater than an expression.
///
/// Pseudocode:<br>
/// | a - b | > x
///
/// * If true, return `Ok(abs_diff)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_abs_diff_gt_x`](macro@crate::assert_abs_diff_gt_x)
/// * [`assert_abs_diff_gt_x_as_result`](macro@crate::assert_abs_diff_gt_x_as_result)
/// * [`debug_assert_abs_diff_gt_x`](macro@crate::debug_assert_abs_diff_gt_x)
///
#[macro_export]
macro_rules! assert_abs_diff_gt_x_as_result {
    ($a:expr, $b:expr, $x:expr $(,)?) => {{
        match (&$a, &$b, &$x) {
            (a, b, x) => {
                let abs_diff = if (a >= b) { a - b } else { b - a };
                if abs_diff.gt(x) {
                    Ok(abs_diff)
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
                                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
                                "       a label: `{}`,\n",
                                "       a debug: `{:?}`,\n",
                                "       b label: `{}`,\n",
                                "       b debug: `{:?}`,\n",
                                "       x label: `{}`,\n",
                                "       x debug: `{:?}`,\n",
                                "     | a - b |: `{:?}`,\n",
                                " | a - b | > x: {}"
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b,
                            stringify!($x),
                            x,
                            abs_diff,
                            false
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_abs_diff_gt_x_as_result {

    #[test]
    fn gt() {
        let a = 10;
        let b = 13;
        let x = 2;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        assert_eq!(actual.unwrap(), 3);
    }

    #[test]
    fn eq() {
        let a = 10;
        let b = 13;
        let x = 3;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `13`,\n",
            "       x label: `x`,\n",
            "       x debug: `3`,\n",
            "     | a - b |: `3`,\n",
            " | a - b | > x: false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn lt() {
        let a = 10;
        let b = 13;
        let x = 4;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `13`,\n",
            "       x label: `x`,\n",
            "       x debug: `4`,\n",
            "     | a - b |: `3`,\n",
            " | a - b | > x: false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an absolute difference is greater than an expression.
///
/// Pseudocode:<br>
/// | a - b | > x
///
/// * If true, return `abs_diff`.
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
/// let a = 10;
/// let b = 13;
/// let x = 2;
/// assert_abs_diff_gt_x!(a, b, x);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 10;
/// let b = 13;
/// let x = 4;
/// assert_abs_diff_gt_x!(a, b, x);
/// # });
/// // assertion failed: `assert_abs_diff_gt_x!(a, b)`
/// // https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html
/// //        a label: `a`,
/// //        a debug: `10`,
/// //        b label: `b`,
/// //        b debug: `13`,
/// //        x label: `x`,
/// //        x debug: `4`,
/// //      | a - b |: `3`,
/// //  | a - b | > x: false
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
/// #     "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
/// #     "       a label: `a`,\n",
/// #     "       a debug: `10`,\n",
/// #     "       b label: `b`,\n",
/// #     "       b debug: `13`,\n",
/// #     "       x label: `x`,\n",
/// #     "       x debug: `4`,\n",
/// #     "     | a - b |: `3`,\n",
/// #     " | a - b | > x: false"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_abs_diff_gt_x`](macro@crate::assert_abs_diff_gt_x)
/// * [`assert_abs_diff_gt_x_as_result`](macro@crate::assert_abs_diff_gt_x_as_result)
/// * [`debug_assert_abs_diff_gt_x`](macro@crate::debug_assert_abs_diff_gt_x)
///
#[macro_export]
macro_rules! assert_abs_diff_gt_x {
    ($a:expr, $b:expr, $x:expr $(,)?) => {{
        match $crate::assert_abs_diff_gt_x_as_result!($a, $b, $x) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $x:expr, $($message:tt)+) => {{
        match $crate::assert_abs_diff_gt_x_as_result!($a, $b, $x) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod assert_abs_diff_gt_x {
    use std::panic;

    #[test]
    fn gt() {
        let a = 10;
        let b = 13;
        let x = 2;
        let actual = assert_abs_diff_gt_x!(a, b, x);
        assert_eq!(actual, 3);
    }

    #[test]
    fn eq() {
        let result = panic::catch_unwind(|| {
            let a = 10;
            let b = 13;
            let x = 3;
            let _actual = assert_abs_diff_gt_x!(a, b, x);
        });
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `13`,\n",
            "       x label: `x`,\n",
            "       x debug: `3`,\n",
            "     | a - b |: `3`,\n",
            " | a - b | > x: false"
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

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let a = 10;
            let b = 13;
            let x = 4;
            let _actual = assert_abs_diff_gt_x!(a, b, x);
        });
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_abs_diff_gt_x.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `13`,\n",
            "       x label: `x`,\n",
            "       x debug: `4`,\n",
            "     | a - b |: `3`,\n",
            " | a - b | > x: false"
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

/// Assert an absolute difference is greater than an expression.
///
/// Pseudocode:<br>
/// | a - b | > c
///
/// This macro provides the same statements as [`assert_abs_diff_gt_x`](macro.assert_abs_diff_gt_x.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-x debug-assertions` is passed to the compiler.
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
/// * [`assert_abs_diff_gt_x`](macro@crate::assert_abs_diff_gt_x)
/// * [`assert_abs_diff_gt_x`](macro@crate::assert_abs_diff_gt_x)
/// * [`debug_assert_abs_diff_gt_x`](macro@crate::debug_assert_abs_diff_gt_x)
///
#[macro_export]
macro_rules! debug_assert_abs_diff_gt_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_abs_diff_gt_x!($($arg)*);
        }
    };
}
