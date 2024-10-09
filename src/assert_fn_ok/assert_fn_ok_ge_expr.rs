//! Assert a function Ok(…) is greater than or equal to an expression.
//!
//! Pseudocode:<br>
//! (function1(param1) ⇒ Ok(a) ⇒ a) ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! fn f(i: i8) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! # fn main() {
//! let a: i8 = 2;
//! let b = String::from("1");
//! assert_fn_ok_ge_expr!(f, a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fn_ok_ge_expr`](macro@crate::assert_fn_ok_ge_expr)
//! * [`assert_fn_ok_ge_expr_as_result`](macro@crate::assert_fn_ok_ge_expr_as_result)
//! * [`debug_assert_fn_ok_ge_expr`](macro@crate::debug_assert_fn_ok_ge_expr)

/// Assert a function Ok(…) is greater than or equal to an expression.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Ok(a) ⇒ a) ≥ expr
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fn_ok_ge_expr`](macro@crate::assert_fn_ok_ge_expr)
/// * [`assert_fn_ok_ge_expr_as_result`](macro@crate::assert_fn_ok_ge_expr_as_result)
/// * [`debug_assert_fn_ok_ge_expr`](macro@crate::debug_assert_fn_ok_ge_expr)
///
#[macro_export]
macro_rules! assert_fn_ok_ge_expr_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_expr:expr $(,)?) => {{
        match (&$a_function, &$a_param, &$b_expr) {
            (_a_function, a_param, b_expr) => {
                let a_result = $a_function($a_param);
                let a_is_ok = a_result.is_ok();
                if !a_is_ok {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_fn_ok_ge_expr!(a_function, a_param, b_expr)`\n",
                            "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                            " a_function label: `{}`,\n",
                            "    a_param label: `{}`,\n",
                            "    a_param debug: `{:?}`,\n",
                            "     b_expr label: `{}`,\n",
                            "     b_expr debug: `{:?}`,\n",
                            "         a result: `{:?}`",
                        ),
                        stringify!($a_function),
                        stringify!($a_param),
                        a_param,
                        stringify!($b_expr),
                        b_expr,
                        a_result
                    ))
                } else {
                    let a_ok = a_result.unwrap();
                    if a_ok >= $b_expr {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_fn_ok_ge_expr!(a_function, a_param, b_expr)`\n",
                                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                                " a_function label: `{}`,\n",
                                "    a_param label: `{}`,\n",
                                "    a_param debug: `{:?}`,\n",
                                "     b_expr label: `{}`,\n",
                                "     b_expr debug: `{:?}`,\n",
                                "                a: `{:?}`,\n",
                                "                b: `{:?}`",
                            ),
                            stringify!($a_function),
                            stringify!($a_param),
                            a_param,
                            stringify!($b_expr),
                            b_expr,
                            a_ok,
                            $b_expr
                        ))
                    }
                }
            }
        }
    }};

    //// Arity 0

    ($a_function:path, $b_expr:expr $(,)?) => {{
        match (&$a_function, &$b_expr) {
            (_a_function, b_expr) => {
                let a_result = $a_function();
                let a_is_ok = a_result.is_ok();
                if !a_is_ok {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_fn_ok_ge_expr!(a_function, b_expr)`\n",
                            "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                            " a_function label: `{}`,\n",
                            "     b_expr label: `{}`,\n",
                            "     b_expr debug: `{:?}`,\n",
                            "         a result: `{:?}`",
                        ),
                        stringify!($a_function),
                        stringify!($b_expr),
                        b_expr,
                        a_result
                    ))
                } else {
                    let a_ok = a_result.unwrap();
                    if a_ok >= $b_expr {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_fn_ok_ge_expr!(a_function, b_expr)`\n",
                                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                                " a_function label: `{}`,\n",
                                "     b_expr label: `{}`,\n",
                                "     b_expr debug: `{:?}`,\n",
                                "                a: `{:?}`,\n",
                                "                b: `{:?}`",
                            ),
                            stringify!($a_function),
                            stringify!($b_expr),
                            b_expr,
                            a_ok,
                            $b_expr
                        ))
                    }
                }
            }
        }
    }};

}

#[cfg(test)]
mod tests {

    mod assert_fn_ok_ge_expr_as_result {

        mod arity_1 {

            fn f(i: i8) -> Result<i8, i8> {
                return Ok(i);
            }

            #[test]
            fn test_gt() {
                let a: i8 = 1;
                let b: i8 = 0;
                let result = assert_fn_ok_ge_expr_as_result!(f, a, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let result = assert_fn_ok_ge_expr_as_result!(f, a, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_lt() {
                let a: i8 = 1;
                let b: i8 = 2;
                let result = assert_fn_ok_ge_expr_as_result!(f, a, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_ok_ge_expr!(a_function, a_param, b_expr)`\n",
                        "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                        " a_function label: `f`,\n",
                        "    a_param label: `a`,\n",
                        "    a_param debug: `1`,\n",
                        "     b_expr label: `b`,\n",
                        "     b_expr debug: `2`,\n",
                        "                a: `1`,\n",
                        "                b: `2`"
                    )
                );
            }
        }

        mod arity_0 {

            fn f() -> Result<i8, i8> {
                return Ok(1);
            }

            #[test]
            fn test_gt() {
                let b: i8 = 0;
                let result = assert_fn_ok_ge_expr_as_result!(f, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let b: i8 = 1;
                let result = assert_fn_ok_ge_expr_as_result!(f, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_lt() {
                let b: i8 = 2;
                let result = assert_fn_ok_ge_expr_as_result!(f, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_ok_ge_expr!(a_function, b_expr)`\n",
                        "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
                        " a_function label: `f`,\n",
                        "     b_expr label: `b`,\n",
                        "     b_expr debug: `2`,\n",
                        "                a: `1`,\n",
                        "                b: `2`"
                    )
                );
            }
        }
    }
}

/// Assert a function Ok(…) is greater than or equal to an expression.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Ok(a) ⇒ a) ≥ expr
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
/// fn f(i: i8) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// let a: i8 = 2;
/// let b = String::from("1");
/// assert_fn_ok_ge_expr!(f, a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: i8 = 1;
/// let b = String::from("2");
/// assert_fn_ok_ge_expr!(f, a, b);
/// # });
/// // assertion failed: `assert_fn_ok_ge_expr!(a_function, a_param, b_expr)`
/// // https://docs.rs/assertables/8.18.0/assertables/macro.assert_fn_ok_ge_expr.html
/// //  a_function label: `f`,
/// //     a_param label: `a`,
/// //     a_param debug: `1`,
/// //      b_expr label: `b`,
/// //      b_expr debug: `\"2\"`,
/// //                 a: `\"1\"`,
/// //                 b: `\"2\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fn_ok_ge_expr!(a_function, a_param, b_expr)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_fn_ok_ge_expr.html\n",
/// #     " a_function label: `f`,\n",
/// #     "    a_param label: `a`,\n",
/// #     "    a_param debug: `1`,\n",
/// #     "     b_expr label: `b`,\n",
/// #     "     b_expr debug: `\"2\"`,\n",
/// #     "                a: `\"1\"`,\n",
/// #     "                b: `\"2\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// /// # Module macros
///
/// * [`assert_fn_ok_ge_expr`](macro@crate::assert_fn_ok_ge_expr)
/// * [`assert_fn_ok_ge_expr_as_result`](macro@crate::assert_fn_ok_ge_expr_as_result)
/// * [`debug_assert_fn_ok_ge_expr`](macro@crate::debug_assert_fn_ok_ge_expr)
///
#[macro_export]
macro_rules! assert_fn_ok_ge_expr {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_expr:expr $(,)?) => {{
        match $crate::assert_fn_ok_ge_expr_as_result!($a_function, $a_param, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $a_param:expr, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_fn_ok_ge_expr_as_result!($a_function, $a_param, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};

    //// Arity 0

    ($a_function:path, $b_expr:expr $(,)?) => {{
        match $crate::assert_fn_ok_ge_expr_as_result!($a_function, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_fn_ok_ge_expr_as_result!($a_function, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a function Ok(…) is greater than or equal to an expression.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Ok(a) ⇒ a) ≥ expr
///
/// This macro provides the same statements as [`assert_fn_ok_ge_expr`](macro.assert_fn_ok_ge_expr.html),
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
/// * [`assert_fn_ok_ge_expr`](macro@crate::assert_fn_ok_ge_expr)
/// * [`assert_fn_ok_ge_expr`](macro@crate::assert_fn_ok_ge_expr)
/// * [`debug_assert_fn_ok_ge_expr`](macro@crate::debug_assert_fn_ok_ge_expr)
///
#[macro_export]
macro_rules! debug_assert_fn_ok_ge_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_ok_ge_expr!($($arg)*);
        }
    };
}
