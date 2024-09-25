//! Assert a function output is greater than another.
//!
//! Pseudocode:<br>
//! function1(a) > function2(b)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: i8 = -2;
//! let b: i8 = 1;
//! assert_fn_gt!(i8::abs, a, i8::abs, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fn_gt`](macro@crate::assert_fn_gt)
//! * [`assert_fn_gt_as_result`](macro@crate::assert_fn_gt_as_result)
//! * [`debug_assert_fn_gt`](macro@crate::debug_assert_fn_gt)

/// Assert a function output is greater than another.
///
/// Pseudocode:<br>
/// function1(a) > function2(b)
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
/// * [`assert_fn_gt`](macro@crate::assert_fn_gt)
/// * [`assert_fn_gt_as_result`](macro@crate::assert_fn_gt_as_result)
/// * [`debug_assert_fn_gt`](macro@crate::debug_assert_fn_gt)
///
#[macro_export]
macro_rules! assert_fn_gt_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {{
        match (&$a_function, &$a_param, &$b_function, &$b_param) {
            (_a_function, a_param, _b_function, b_param) => {
                let a_output = $a_function($a_param);
                let b_output = $b_function($b_param);
                if a_output > b_output {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_fn_gt!(a_function, a_param, b_function, b_param)`\n",
                            "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
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
                        a_output,
                        b_output
                    ))
                }
            }
        }
    }};

    //// Arity 0

    ($a_function:path, $b_function:path) => {{
        let a_output = $a_function();
        let b_output = $b_function();
        if a_output > b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_gt!(a_function, b_function)`\n",
                    "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
                    " a_function label: `{}`,\n",
                    " b_function label: `{}`,\n",
                    "                a: `{:?}`,\n",
                    "                b: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($b_function),
                a_output,
                b_output
            ))
        }
    }};

}

#[cfg(test)]
mod tests {

    mod assert_fn_gt_as_result {

        mod arity_1 {

            fn f(i: i8) -> i8 {
                return i;
            }

            fn g(i: i8) -> i8 {
                return i;
            }

            #[test]
            fn test_gt() {
                let a: i8 = 2;
                let b: i8 = 1;
                let result = assert_fn_gt_as_result!(f, a, g, b);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let result = assert_fn_gt_as_result!(f, a, g, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(a_function, a_param, b_function, b_param)`\n",
                        "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
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
                let result = assert_fn_gt_as_result!(f, a, g, b);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(a_function, a_param, b_function, b_param)`\n",
                        "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
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

            fn f() -> i8 {
                return 1;
            }

            fn g() -> i8 {
                return 2;
            }

            #[test]
            fn test_gt() {
                let result = assert_fn_gt_as_result!(g, f);
                assert_eq!(result, Ok(()));
            }

            #[test]
            fn test_eq() {
                let result = assert_fn_gt_as_result!(f, f);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(a_function, b_function)`\n",
                        "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
                        " a_function label: `f`,\n",
                        " b_function label: `f`,\n",
                        "                a: `1`,\n",
                        "                b: `1`"
                    )
                );
            }

            #[test]
            fn test_lt() {
                let result = assert_fn_gt_as_result!(f, g);
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(a_function, b_function)`\n",
                        "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
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

/// Assert a function output is greater than another.
///
/// Pseudocode:<br>
/// function1(a) > function2(b)
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
/// let a: i8 = -2;
/// let b: i8 = 1;
/// assert_fn_gt!(i8::abs, a, i8::abs, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a: i8 = 1;
/// let b: i8 = -2;
/// assert_fn_gt!(i8::abs, a, i8::abs, b);
/// # });
/// // assertion failed: `assert_fn_gt!(a_function, a_param, b_function, b_param)`
/// // https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html
/// //  a_function label: `i8::abs`,
/// //     a_param label: `a`,
/// //     a_param debug: `1`,
/// //  b_function label: `i8::abs`,
/// //     b_param label: `b`,
/// //     b_param debug: `-2`,
/// //                 a: `1`,
/// //                 b: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fn_gt!(a_function, a_param, b_function, b_param)`\n",
/// #     "https://docs.rs/assertables/8.9.0/assertables/macro.assert_fn_gt.html\n",
/// #     " a_function label: `i8::abs`,\n",
/// #     "    a_param label: `a`,\n",
/// #     "    a_param debug: `1`,\n",
/// #     " b_function label: `i8::abs`,\n",
/// #     "    b_param label: `b`,\n",
/// #     "    b_param debug: `-2`,\n",
/// #     "                a: `1`,\n",
/// #     "                b: `2`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fn_gt`](macro@crate::assert_fn_gt)
/// * [`assert_fn_gt_as_result`](macro@crate::assert_fn_gt_as_result)
/// * [`debug_assert_fn_gt`](macro@crate::debug_assert_fn_gt)
///
#[macro_export]
macro_rules! assert_fn_gt {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {{
        match assert_fn_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr, $($message:tt)+) => {{
        match assert_fn_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};

    //// Arity 0

    ($a_function:path, $b_function:path) => {{
        match assert_fn_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};

    ($a_function:path, $b_function:path, $($message:tt)+) => {{
        match assert_fn_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};

}

/// Assert a function output is greater than another.
///
/// Pseudocode:<br>
/// function1(a) > function2(b)
///
/// This macro provides the same statements as [`assert_fn_gt`](macro.assert_fn_gt.html),
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
/// * [`assert_fn_gt`](macro@crate::assert_fn_gt)
/// * [`assert_fn_gt`](macro@crate::assert_fn_gt)
/// * [`debug_assert_fn_gt`](macro@crate::debug_assert_fn_gt)
///
#[macro_export]
macro_rules! debug_assert_fn_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_gt!($($arg)*);
        }
    };
}
