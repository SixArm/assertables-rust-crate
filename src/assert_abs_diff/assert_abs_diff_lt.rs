//! Assert an absolute difference is less than a delta expression.
//!
//! Pseudocode:<br>
//! | a - b | < Δ
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = 10;
//! let b = 13;
//! let delta = 4;
//! assert_abs_diff_lt!(a, b, delta);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_abs_diff_lt`](macro@crate::assert_abs_diff_lt)
//! * [`assert_abs_diff_lt_as_result`](macro@crate::assert_abs_diff_lt_as_result)
//! * [`debug_assert_abs_diff_lt`](macro@crate::debug_assert_abs_diff_lt)

/// Assert an absolute difference is less than a delta expression.
///
/// Pseudocode:<br>
/// | a - b | < Δ
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
/// * [`assert_abs_diff_lt`](macro@crate::assert_abs_diff_lt)
/// * [`assert_abs_diff_lt_as_result`](macro@crate::assert_abs_diff_lt_as_result)
/// * [`debug_assert_abs_diff_lt`](macro@crate::debug_assert_abs_diff_lt)
///
#[macro_export]
macro_rules! assert_abs_diff_lt_as_result {
    ($a:expr, $b:expr, $delta:expr $(,)?) => {{
        match (&$a, &$b, &$delta) {
            (a, b, delta) => {
                let abs_diff = if (a >= b) { a - b } else { b - a };
                if abs_diff.lt(delta) {
                    Ok(abs_diff)
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_abs_diff_lt!(a, b, delta)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_abs_diff_lt.html\n",
                                "       a label: `{}`,\n",
                                "       a debug: `{:?}`,\n",
                                "       b label: `{}`,\n",
                                "       b debug: `{:?}`,\n",
                                "       Δ label: `{}`,\n",
                                "       Δ debug: `{:?}`,\n",
                                "     | a - b |: `{:?}`,\n",
                                " | a - b | < Δ: {}"
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b,
                            stringify!($delta),
                            delta,
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
mod test {

    #[test]
    fn lt() {
        let a = 10;
        let b = 13;
        let delta = 4;
        let result = assert_abs_diff_lt_as_result!(a, b, delta);
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn eq() {
        let a = 10;
        let b = 13;
        let delta = 3;
        let result = assert_abs_diff_lt_as_result!(a, b, delta);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_abs_diff_lt!(a, b, delta)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_abs_diff_lt.html\n",
                "       a label: `a`,\n",
                "       a debug: `10`,\n",
                "       b label: `b`,\n",
                "       b debug: `13`,\n",
                "       Δ label: `delta`,\n",
                "       Δ debug: `3`,\n",
                "     | a - b |: `3`,\n",
                " | a - b | < Δ: false"
            )
        );
    }

    #[test]
    fn gt() {
        let a = 10;
        let b = 13;
        let delta = 2;
        let result = assert_abs_diff_lt_as_result!(a, b, delta);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_abs_diff_lt!(a, b, delta)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_abs_diff_lt.html\n",
                "       a label: `a`,\n",
                "       a debug: `10`,\n",
                "       b label: `b`,\n",
                "       b debug: `13`,\n",
                "       Δ label: `delta`,\n",
                "       Δ debug: `2`,\n",
                "     | a - b |: `3`,\n",
                " | a - b | < Δ: false"
            )
        );
    }
}

/// Assert an absolute difference is less than a delta expression.
///
/// Pseudocode:<br>
/// | a - b | < Δ
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
/// let delta = 4;
/// assert_abs_diff_lt!(a, b, delta);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 10;
/// let b = 13;
/// let delta = 2;
/// assert_abs_diff_lt!(a, b, delta);
/// # });
/// // assertion failed: `assert_abs_diff_lt!(a, b)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_abs_diff_lt.html
/// //        a label: `a`,
/// //        a debug: `10`,
/// //        b label: `b`,
/// //        b debug: `13`,
/// //        Δ label: `delta`,
/// //        Δ debug: `2`,
/// //      | a - b |: `3`,
/// //  | a - b | < Δ: false
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_abs_diff_lt!(a, b, delta)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_abs_diff_lt.html\n",
/// #     "       a label: `a`,\n",
/// #     "       a debug: `10`,\n",
/// #     "       b label: `b`,\n",
/// #     "       b debug: `13`,\n",
/// #     "       Δ label: `delta`,\n",
/// #     "       Δ debug: `2`,\n",
/// #     "     | a - b |: `3`,\n",
/// #     " | a - b | < Δ: false"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_abs_diff_lt`](macro@crate::assert_abs_diff_lt)
/// * [`assert_abs_diff_lt_as_result`](macro@crate::assert_abs_diff_lt_as_result)
/// * [`debug_assert_abs_diff_lt`](macro@crate::debug_assert_abs_diff_lt)
///
#[macro_export]
macro_rules! assert_abs_diff_lt {
    ($a:expr, $b:expr, $delta:expr $(,)?) => {{
        match $crate::assert_abs_diff_lt_as_result!($a, $b, $delta) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $delta:expr, $($message:tt)+) => {{
        match $crate::assert_abs_diff_lt_as_result!($a, $b, $delta) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an absolute difference is less than a delta expression.
///
/// Pseudocode:<br>
/// | a - b | < c
///
/// This macro provides the same statements as [`assert_abs_diff_lt`](macro.assert_abs_diff_lt.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-Δ debug-assertions` is passed to the compiler.
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
/// * [`assert_abs_diff_lt`](macro@crate::assert_abs_diff_lt)
/// * [`assert_abs_diff_lt`](macro@crate::assert_abs_diff_lt)
/// * [`debug_assert_abs_diff_lt`](macro@crate::debug_assert_abs_diff_lt)
///
#[macro_export]
macro_rules! debug_assert_abs_diff_lt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_abs_diff_lt!($($arg)*);
        }
    };
}
