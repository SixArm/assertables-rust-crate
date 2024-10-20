//! Assert a function Err(…) is greater than another.
//!
//! Pseudocode:<br>
//! (function1(param1) ⇒ Err(a) ⇒ a) > (function2(param2) ⇒ Err(b) ⇒ b)
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
//! let a: i8 = 20;
//! let b: i8 = 10;
//! assert_fn_err_gt!(f, a, f, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fn_err_gt`](macro@crate::assert_fn_err_gt)
//! * [`assert_fn_err_gt_as_result`](macro@crate::assert_fn_err_gt_as_result)
//! * [`debug_assert_fn_err_gt`](macro@crate::debug_assert_fn_err_gt)

/// Assert a function error is greater than another.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Err(a) ⇒ a) > (function2(param2) ⇒ Err(b) ⇒ b)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fn_err_gt`](macro@crate::assert_fn_err_gt)
/// * [`assert_fn_err_gt_as_result`](macro@crate::assert_fn_err_gt_as_result)
/// * [`debug_assert_fn_err_gt`](macro@crate::debug_assert_fn_err_gt)
///
#[macro_export]
macro_rules! assert_fn_err_gt_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {{
        match (&$a_function, &$a_param, &$b_function, &$b_param) {
            (_a_function, a_param, _b_function, b_param) => {
                let a_result = $a_function($a_param);
                let b_result = $b_function($b_param);
                let a_is_err = a_result.is_err();
                let b_is_err = b_result.is_err();
                if !a_is_err || !b_is_err {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
                                "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                                " a_function label: `{}`,\n",
                                "    a_param label: `{}`,\n",
                                "    a_param debug: `{:?}`,\n",
                                " b_function label: `{}`,\n",
                                "    b_param label: `{}`,\n",
                                "    b_param debug: `{:?}`,\n",
                                "                a: `{:?}`,\n",
                                "                b: `{:?}`"
                            ),
                            stringify!($a_function),
                            stringify!($a_param),
                            a_param,
                            stringify!($b_function),
                            stringify!($b_param),
                            b_param,
                            a_result,
                            b_result
                        )
                    )
                } else {
                    let a_err = a_result.unwrap_err();
                    let b_err = b_result.unwrap_err();
                    if a_err > b_err {
                        Ok(())
                    } else {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
                                    "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                                    " a_function label: `{}`,\n",
                                    "    a_param label: `{}`,\n",
                                    "    a_param debug: `{:?}`,\n",
                                    " b_function label: `{}`,\n",
                                    "    b_param label: `{}`,\n",
                                    "    b_param debug: `{:?}`,\n",
                                    "                a: `{:?}`,\n",
                                    "                b: `{:?}`"
                                ),
                                stringify!($a_function),
                                stringify!($a_param),
                                a_param,
                                stringify!($b_function),
                                stringify!($b_param),
                                b_param,
                                a_err,
                                b_err
                            )
                        )
                    }
                }
            }
        }
    }};

    //// Arity 0

    ($a_function:path, $b_function:path) => {{
        let a_result = $a_function();
        let b_result = $b_function();
        let a_is_err = a_result.is_err();
        let b_is_err = b_result.is_err();
        if !a_is_err || !b_is_err {
            Err(
                format!(
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(a_function, b_function)`\n",
                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                        " a_function label: `{}`,\n",
                        " b_function label: `{}`,\n",
                        "                a: `{:?}`,\n",
                        "                b: `{:?}`"
                    ),
                    stringify!($a_function),
                    stringify!($b_function),
                    a_result,
                    b_result
                )
            )
        } else {
            let a_err = a_result.unwrap_err();
            let b_err = b_result.unwrap_err();
            if a_err > b_err {
                Ok(())
            } else {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_fn_err_gt!(a_function, b_function)`\n",
                            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                            " a_function label: `{}`,\n",
                            " b_function label: `{}`,\n",
                            "                a: `{:?}`,\n",
                            "                b: `{:?}`"
                        ),
                        stringify!($a_function),
                        stringify!($b_function),
                        a_err,
                        b_err
                    )
                )
            }
        }
    }};

}

#[cfg(test)]
mod tests {

    mod assert_fn_err_gt_as_result {

        mod arity_1 {

            fn f(i: i8) -> Result<i8, i8> {
                Err(i)
            }

            fn g(i: i8) -> Result<i8, i8> {
                Err(i)
            }

            #[test]
            fn test_gt() {
                let a: i8 = 2;
                let b: i8 = 1;
                let result = assert_fn_err_gt_as_result!(f, a, g, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let result = assert_fn_err_gt_as_result!(f, a, g, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                        " a_function label: `f`,\n",
                        "    a_param label: `a`,\n",
                        "    a_param debug: `1`,\n",
                        " b_function label: `g`,\n",
                        "    b_param label: `b`,\n",
                        "    b_param debug: `1`,\n",
                        "                a: `1`,\n",
                        "                b: `1`"
                    )
                );
            }

            #[test]
            fn test_lt() {
                let a: i8 = 1;
                let b: i8 = 2;
                let result = assert_fn_err_gt_as_result!(f, a, g, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                        " a_function label: `f`,\n",
                        "    a_param label: `a`,\n",
                        "    a_param debug: `1`,\n",
                        " b_function label: `g`,\n",
                        "    b_param label: `b`,\n",
                        "    b_param debug: `2`,\n",
                        "                a: `1`,\n",
                        "                b: `2`"
                    )
                );
            }
        }

        mod arity_0 {

            fn f() -> Result<i8, i8> {
                Err(1)
            }

            fn g() -> Result<i8, i8> {
                Err(2)
            }

            #[test]
            fn test_gt() {
                let result = assert_fn_err_gt_as_result!(g, f);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let result = assert_fn_err_gt_as_result!(f, f);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(a_function, b_function)`\n",
                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                        " a_function label: `f`,\n",
                        " b_function label: `f`,\n",
                        "                a: `1`,\n",
                        "                b: `1`"
                    )
                );
            }

            #[test]
            fn test_lt() {
                let result = assert_fn_err_gt_as_result!(f, g);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(a_function, b_function)`\n",
                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
                        " a_function label: `f`,\n",
                        " b_function label: `g`,\n",
                        "                a: `1`,\n",
                        "                b: `2`"
                    )
                );
            }
        }
    }
}

/// Assert a function error is greater than another.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Err(a) ⇒ a) > (function2(param2) ⇒ Err(b) ⇒ b)
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
/// let a: i8 = 20;
/// let b: i8 = 10;
/// assert_fn_err_gt!(f, a, f, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: i8 = 10;
/// let b: i8 = 20;
/// assert_fn_err_gt!(f, a, f, b);
/// # });
/// // assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`
/// // https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html
/// //  a_function label: `f`,
/// //     a_param label: `a`,
/// //     a_param debug: `10`,
/// //  b_function label: `f`,
/// //     b_param label: `b`,
/// //     b_param debug: `20`,
/// //                 a: `\"10 is out of range\"`,
/// //                 b: `\"20 is out of range\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
/// #     "https://docs.rs/assertables/9.0.0/assertables/macro.assert_fn_err_gt.html\n",
/// #     " a_function label: `f`,\n",
/// #     "    a_param label: `a`,\n",
/// #     "    a_param debug: `10`,\n",
/// #     " b_function label: `f`,\n",
/// #     "    b_param label: `b`,\n",
/// #     "    b_param debug: `20`,\n",
/// #     "                a: `\"10 is out of range\"`,\n",
/// #     "                b: `\"20 is out of range\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fn_err_gt`](macro@crate::assert_fn_err_gt)
/// * [`assert_fn_err_gt_as_result`](macro@crate::assert_fn_err_gt_as_result)
/// * [`debug_assert_fn_err_gt`](macro@crate::debug_assert_fn_err_gt)
///
#[macro_export]
macro_rules! assert_fn_err_gt {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {{
        match $crate::assert_fn_err_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr, $($message:tt)+) => {{
        match $crate::assert_fn_err_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};

    //// Arity 0

    ($a_function:path, $b_function:path) => {{
        match $crate::assert_fn_err_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $b_function:path, $($message:tt)+) => {{
        match $crate::assert_fn_err_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a function error is greater than another.
///
/// Pseudocode:<br>
/// (function1(param1) ⇒ Err(a) ⇒ a) > (function2(param2) ⇒ Err(b) ⇒ b)
///
/// This macro provides the same statements as [`assert_fn_err_gt`](macro.assert_fn_err_gt.html),
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
/// * [`assert_fn_err_gt`](macro@crate::assert_fn_err_gt)
/// * [`assert_fn_err_gt`](macro@crate::assert_fn_err_gt)
/// * [`debug_assert_fn_err_gt`](macro@crate::debug_assert_fn_err_gt)
///
#[macro_export]
macro_rules! debug_assert_fn_err_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_err_gt!($($arg)*);
        }
    };
}
