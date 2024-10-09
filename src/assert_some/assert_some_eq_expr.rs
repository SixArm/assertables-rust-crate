//! Assert an expression is Some(_) and its value is equal to an expression.
//!
//! Pseudocode:<br>
//! (a ⇒ Some(a̅) ⇒ a̅) = b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: Option<i8> = Option::Some(1);
//! let b: i8 = 1;
//! assert_some_eq_expr!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_some_eq_expr`](macro@crate::assert_some_eq_expr)
//! * [`assert_some_eq_expr_as_result`](macro@crate::assert_some_eq_expr_as_result)
//! * [`debug_assert_some_eq_expr`](macro@crate::debug_assert_some_eq_expr)

/// Assert a.is_some() and a.unwrap() are equal to another.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a̅) ⇒ a̅) = b
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_some_eq_expr`](macro.assert_some_eq_expr.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_some_eq_expr`](macro@crate::assert_some_eq_expr)
/// * [`assert_some_eq_expr_as_result`](macro@crate::assert_some_eq_expr_as_result)
/// * [`debug_assert_some_eq_expr`](macro@crate::debug_assert_some_eq_expr)
///
#[macro_export]
macro_rules! assert_some_eq_expr_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                match a {
                    Some(a_inner) => {
                        if a_inner == b {
                            Ok(())
                        } else {
                            Err(format!(
                                concat!(
                                    "assertion failed: `assert_some_eq_expr!(a, b)`\n",
                                    "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_some_eq_expr.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`,\n",
                                    " a inner: `{:?}`,\n",
                                    " b label: `{}`,\n",
                                    " b debug: `{:?}`"
                                ),
                                stringify!($a),
                                a,
                                a_inner,
                                stringify!($b),
                                b
                            ))
                        }
                    },
                    _ => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_some_eq_expr!(a, b)`\n",
                                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_some_eq_expr.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`",
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b,
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
    fn test_assert_some_eq_expr_as_result_x_success() {
        let a: Option<i8> = Option::Some(1);
        let b: i8 = 1;
        let result = assert_some_eq_expr_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_some_eq_expr_as_result_x_failure_because_ne() {
        let a: Option<i8> = Option::Some(1);
        let b: i8 = 2;
        let result = assert_some_eq_expr_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_some_eq_expr!(a, b)`\n",
                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_some_eq_expr.html\n",
                " a label: `a`,\n",
                " a debug: `Some(1)`,\n",
                " a inner: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`"
            )
        );
    }

    #[test]
    fn test_assert_some_eq_expr_as_result_x_failure_because_not_some() {
        let a: Option<i8> = Option::None;
        let b: i8 = 1;
        let result = assert_some_eq_expr_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_some_eq_expr!(a, b)`\n",
                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_some_eq_expr.html\n",
                " a label: `a`,\n",
                " a debug: `None`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
            )
        );
    }

}

/// Assert an expression is Some(_) and its value is equal to an expression.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a̅) ⇒ a̅) = b
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
/// let a: Option<i8> = Option::Some(1);
/// let b: i8 = 1;
/// assert_some_eq_expr!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Option<i8> = Option::Some(1);
/// let b: i8 = 2;
/// assert_some_eq_expr!(a, b);
/// # });
/// // assertion failed: `assert_some_eq_expr!(a, b)`
/// // https://docs.rs/assertables/8.17.0/assertables/macro.assert_some_eq_expr.html
/// //  a label: `a`,
/// //  a debug: `Some(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_some_eq_expr!(a, b)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_some_eq_expr.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Some(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `2`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_some_eq_expr`](macro@crate::assert_some_eq_expr)
/// * [`assert_some_eq_expr_as_result`](macro@crate::assert_some_eq_expr_as_result)
/// * [`debug_assert_some_eq_expr`](macro@crate::debug_assert_some_eq_expr)
///
#[macro_export]
macro_rules! assert_some_eq_expr {
    ($a:expr, $b:expr $(,)?) => {{
        match $crate::assert_some_eq_expr_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_some_eq_expr_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an expression is Some(_) and its value is equal to an expression.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a̅) ⇒ a̅) = b
///
/// This macro provides the same statements as [`assert_some_eq_expr`](macro.assert_some_eq_expr.html),
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
/// * [`assert_some_eq_expr`](macro@crate::assert_some_eq_expr)
/// * [`assert_some_eq_expr`](macro@crate::assert_some_eq_expr)
/// * [`debug_assert_some_eq_expr`](macro@crate::debug_assert_some_eq_expr)
///
#[macro_export]
macro_rules! debug_assert_some_eq_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_some_eq_expr!($($arg)*);
        }
    };
}
