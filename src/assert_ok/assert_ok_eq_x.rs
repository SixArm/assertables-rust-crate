//! Assert an expression is Ok and its value is equal to an expression.
//!
//! Pseudocode:<br>
//! (a ⇒ Ok(a1) ⇒ a1) = b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Result<i8, i8> = Ok(1);
//! let b: i8 = 1;
//! assert_ok_eq_x!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_ok_eq_x`](macro@crate::assert_ok_eq_x)
//! * [`assert_ok_eq_x_as_result`](macro@crate::assert_ok_eq_x_as_result)
//! * [`debug_assert_ok_eq_x`](macro@crate::debug_assert_ok_eq_x)

/// Assert an expression is Ok and its value is equal to an expression.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = b
///
/// * If true, return Result `Ok(a1)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_ok_eq_x`](macro.assert_ok_eq_x.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ok_eq_x`](macro@crate::assert_ok_eq_x)
/// * [`assert_ok_eq_x_as_result`](macro@crate::assert_ok_eq_x_as_result)
/// * [`debug_assert_ok_eq_x`](macro@crate::debug_assert_ok_eq_x)
///
#[macro_export]
macro_rules! assert_ok_eq_x_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a) {
            Ok(a1) => {
                if a1 == $b {
                    Ok(a1)
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_ok_eq_x!(a, b)`\n",
                                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " a inner: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`",
                            ),
                            stringify!($a),
                            $a,
                            a1,
                            stringify!($b),
                            $b
                        )
                    )
                }
            },
            _ => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_ok_eq_x!(a, b)`\n",
                            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b
                    )
                )
            }
        }
    };
}

#[cfg(test)]
mod test_assert_ok_eq_x_as_result {

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Ok(1);
        let b: i8 = 1;
        let actual = assert_ok_eq_x_as_result!(a, b);
        assert_eq!(actual.unwrap(), 1);
    }

    #[test]
    fn ne() {
        let a: Result<i8, i8> = Ok(1);
        let b: i8 = 2;
        let actual = assert_ok_eq_x_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ok_eq_x!(a, b)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `2`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn not_ok() {
        let a: Result<i8, i8> = Err(1);
        let b: i8 = 1;
        let actual = assert_ok_eq_x_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ok_eq_x!(a, b)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " b label: `b`,\n",
            " b debug: `1`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an expression is Ok and its value is equal to an expression.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = b
///
/// * If true, return `a1`.
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
/// let a: Result<i8, i8> = Ok(1);
/// let b: i8 = 1;
/// assert_ok_eq_x!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Result<i8, i8> = Ok(1);
/// let b: i8 = 2;
/// assert_ok_eq_x!(a, b);
/// # });
/// // assertion failed: `assert_ok_eq_x!(a, b)`
/// // https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html
/// //  a label: `a`,
/// //  a debug: `Ok(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_ok_eq_x!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ok(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ok_eq_x`](macro@crate::assert_ok_eq_x)
/// * [`assert_ok_eq_x_as_result`](macro@crate::assert_ok_eq_x_as_result)
/// * [`debug_assert_ok_eq_x`](macro@crate::debug_assert_ok_eq_x)
///
#[macro_export]
macro_rules! assert_ok_eq_x {
    ($a:expr, $b:expr $(,)?) => {{
        match $crate::assert_ok_eq_x_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_ok_eq_x_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_ok_eq_x {
    use std::panic;

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Ok(1);
        let b: i8 = 1;
        let actual = assert_ok_eq_x!(a, b);
        assert_eq!(actual, 1);
    }

    #[test]
    fn ne() {
        let a: Result<i8, i8> = Ok(1);
        let b: i8 = 2;
        let result = panic::catch_unwind(|| {
            let _actual = assert_ok_eq_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq_x!(a, b)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `2`"
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
    fn not_ok() {
        let a: Result<i8, i8> = Err(1);
        let b: i8 = 1;
        let result = panic::catch_unwind(|| {
            let _actual = assert_ok_eq_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq_x!(a, b)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ok_eq_x.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " b label: `b`,\n",
            " b debug: `1`",
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

/// Assert an expression is Ok and its value is equal to an expression.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = b
///
/// This macro provides the same statements as [`assert_ok_eq_x`](macro.assert_ok_eq_x.html),
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
/// * [`assert_ok_eq_x`](macro@crate::assert_ok_eq_x)
/// * [`assert_ok_eq_x`](macro@crate::assert_ok_eq_x)
/// * [`debug_assert_ok_eq_x`](macro@crate::debug_assert_ok_eq_x)
///
#[macro_export]
macro_rules! debug_assert_ok_eq_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_ok_eq_x!($($arg)*);
        }
    };
}
