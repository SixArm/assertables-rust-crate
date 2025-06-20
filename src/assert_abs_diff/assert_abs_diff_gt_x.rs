//! Assert an absolute difference is greater than an expression.
//!
//! Pseudocode:<br>
//! |Δ| > x
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
/// |Δ| > x
///
/// * If true, return `Ok((lhs, rhs))`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
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
    ($a:expr, $b:expr, $x:expr $(,)?) => {
        match (&$a, &$b, &$x) {
            (a, b, x) => {
                match ::std::panic::catch_unwind(|| if (a >= b) { a - b } else { b - a }) {
                    Ok(abs_diff) => {
                        if abs_diff > *x {
                            Ok((abs_diff, *x))
                        } else {
                            Err(format!(
                                concat!(
                                    "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
                                    "https://docs.rs/assertables/",
                                    env!("CARGO_PKG_VERSION"),
                                    "/assertables/macro.assert_abs_diff_gt_x.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`,\n",
                                    " b label: `{}`,\n",
                                    " b debug: `{:?}`,\n",
                                    " x label: `{}`,\n",
                                    " x debug: `{:?}`,\n",
                                    "     |Δ|: `{:?}`,\n",
                                    " |Δ| > x: {}"
                                ),
                                stringify!($a),
                                a,
                                stringify!($b),
                                b,
                                stringify!($x),
                                x,
                                abs_diff,
                                false
                            ))
                        }
                    }
                    Err(_err) => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
                                "https://docs.rs/assertables/",
                                env!("CARGO_PKG_VERSION"),
                                "/assertables/macro.assert_abs_diff_gt_x.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                " x label: `{}`,\n",
                                " x debug: `{:?}`,\n",
                                "     |Δ|: panic", //TODO add the panic message
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b,
                            stringify!($x),
                            x
                        ))
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_abs_diff_gt_x_as_result {
    use std::sync::Once;

    #[test]
    fn gt() {
        let a: i8 = 10;
        let b: i8 = 13;
        let x: i8 = 2;
        for _ in 0..1 {
            let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
            assert_eq!(actual.unwrap(), (3, 2));
        }
    }

    #[test]
    fn gt_once() {
        static A: Once = Once::new();
        fn a() -> i8 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            10
        }

        static B: Once = Once::new();
        fn b() -> i8 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            13
        }

        static X: Once = Once::new();
        fn x() -> i8 {
            if X.is_completed() {
                panic!("X.is_completed()")
            } else {
                X.call_once(|| {})
            }
            2
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        assert_eq!(X.is_completed(), false);
        let result = assert_abs_diff_gt_x_as_result!(a(), b(), x());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        assert_eq!(X.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a: i8 = 10;
        let b: i8 = 13;
        let x: i8 = 3;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_abs_diff_gt_x.html\n",
            " a label: `a`,\n",
            " a debug: `10`,\n",
            " b label: `b`,\n",
            " b debug: `13`,\n",
            " x label: `x`,\n",
            " x debug: `3`,\n",
            "     |Δ|: `3`,\n",
            " |Δ| > x: false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn lt() {
        let a: i8 = 10;
        let b: i8 = 13;
        let x: i8 = 4;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        let message = concat!(
            "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_abs_diff_gt_x.html\n",
            " a label: `a`,\n",
            " a debug: `10`,\n",
            " b label: `b`,\n",
            " b debug: `13`,\n",
            " x label: `x`,\n",
            " x debug: `4`,\n",
            "     |Δ|: `3`,\n",
            " |Δ| > x: false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn overflow() {
        let a: i8 = i8::MAX;
        let b: i8 = i8::MIN;
        let x: i8 = 0;
        let actual = assert_abs_diff_gt_x_as_result!(a, b, x);
        let message = format!(
            concat!(
                "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
                "https://docs.rs/assertables/",
                env!("CARGO_PKG_VERSION"),
                "/assertables/macro.assert_abs_diff_gt_x.html\n",
                " a label: `a`,\n",
                " a debug: `{}`,\n",
                " b label: `b`,\n",
                " b debug: `{}`,\n",
                " x label: `x`,\n",
                " x debug: `{}`,\n",
                "     |Δ|: panic"
            ),
            a, b, x
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an absolute difference is greater than an expression.
///
/// Pseudocode:<br>
/// |Δ| > x
///
/// * If true, return `(lhs, rhs)`.
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
/// // https://docs.rs/assertables/9.7.0/assertables/macro.assert_abs_diff_gt_x.html
/// //  a label: `a`,
/// //  a debug: `10`,
/// //  b label: `b`,
/// //  b debug: `13`,
/// //  x label: `x`,
/// //  x debug: `4`,
/// //      |Δ|: `3`,
/// //  |Δ| > x: false
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_abs_diff_gt_x.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `10`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `13`,\n",
/// #     " x label: `x`,\n",
/// #     " x debug: `4`,\n",
/// #     "     |Δ|: `3`,\n",
/// #     " |Δ| > x: false"
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
    ($a:expr, $b:expr, $x:expr $(,)?) => {
        match $crate::assert_abs_diff_gt_x_as_result!($a, $b, $x) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $x:expr, $($message:tt)+) => {
        match $crate::assert_abs_diff_gt_x_as_result!($a, $b, $x) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod assert_abs_diff_gt_x {
    use std::panic;

    #[test]
    fn gt() {
        let a = 10;
        let b = 13;
        let x = 2;
        for _ in 0..1 {
            let actual = assert_abs_diff_gt_x!(a, b, x);
            assert_eq!(actual, (3, 2));
        }
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
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_abs_diff_gt_x.html\n",
            " a label: `a`,\n",
            " a debug: `10`,\n",
            " b label: `b`,\n",
            " b debug: `13`,\n",
            " x label: `x`,\n",
            " x debug: `3`,\n",
            "     |Δ|: `3`,\n",
            " |Δ| > x: false"
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
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_abs_diff_gt_x.html\n",
            " a label: `a`,\n",
            " a debug: `10`,\n",
            " b label: `b`,\n",
            " b debug: `13`,\n",
            " x label: `x`,\n",
            " x debug: `4`,\n",
            "     |Δ|: `3`,\n",
            " |Δ| > x: false"
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
    fn overflow() {
        let a: i8 = i8::MAX;
        let b: i8 = i8::MIN;
        let x: i8 = 0;
        let result = panic::catch_unwind(|| {
            let _actual = assert_abs_diff_gt_x!(a, b, x);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_abs_diff_gt_x!(a, b, x)`\n",
                "https://docs.rs/assertables/",
                env!("CARGO_PKG_VERSION"),
                "/assertables/macro.assert_abs_diff_gt_x.html\n",
                " a label: `a`,\n",
                " a debug: `{}`,\n",
                " b label: `b`,\n",
                " b debug: `{}`,\n",
                " x label: `x`,\n",
                " x debug: `{}`,\n",
                "     |Δ|: panic"
            ),
            a, b, x
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
/// |Δ| > c
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
