/// Assert a function output is greater than another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_fn_gt`]
/// * [`assert_fn_gt_as_result`]
/// * [`debug_assert_fn_gt`]
///
#[macro_export]
macro_rules! assert_fn_gt_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr) => ({
        let a_output = $a_function($a_param);
        let b_output = $b_function($b_param);
        if a_output > b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_gt!(left_function, left_param, right_function, right_param)`\n",
                    "  left_function label: `{}`,\n",
                    "     left_param label: `{}`,\n",
                    "     left_param debug: `{:?}`,\n",
                    " right_function label: `{}`,\n",
                    "    right_param label: `{}`,\n",
                    "    right_param debug: `{:?}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($a_param), $a_param,
                stringify!($b_function),
                stringify!($b_param), $b_param,
                a_output,
                b_output
            ))
        }
    });

    //// Arity 0

    ($a_function:path, $b_function:path) => ({
        let a_output = $a_function();
        let b_output = $b_function();
        if a_output > b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_gt!(left_function, right_function)`\n",
                    "  left_function label: `{}`,\n",
                    " right_function label: `{}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($b_function),
                a_output,
                b_output
            ))
        }
    });

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
                let x = assert_fn_gt_as_result!(f, a, g, b);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let x = assert_fn_gt_as_result!(f, a, g, b);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(left_function, left_param, right_function, right_param)`\n",
                        "  left_function label: `f`,\n",
                        "     left_param label: `a`,\n",
                        "     left_param debug: `1`,\n",
                        " right_function label: `g`,\n",
                        "    right_param label: `b`,\n",
                        "    right_param debug: `1`,\n",
                        "                 left: `1`,\n",
                        "                right: `1`"
                    )
                );
            }

            #[test]
            fn test_lt() {
                let a: i8 = 1;
                let b: i8 = 2;
                let x = assert_fn_gt_as_result!(f, a, g, b);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(left_function, left_param, right_function, right_param)`\n",
                        "  left_function label: `f`,\n",
                        "     left_param label: `a`,\n",
                        "     left_param debug: `1`,\n",
                        " right_function label: `g`,\n",
                        "    right_param label: `b`,\n",
                        "    right_param debug: `2`,\n",
                        "                 left: `1`,\n",
                        "                right: `2`"
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
                let x = assert_fn_gt_as_result!(g, f);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let x = assert_fn_gt_as_result!(f, f);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(left_function, right_function)`\n",
                        "  left_function label: `f`,\n",
                        " right_function label: `f`,\n",
                        "                 left: `1`,\n",
                        "                right: `1`"
                    )
                );
            }

            #[test]
            fn test_lt() {
                let x = assert_fn_gt_as_result!(f, g);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_gt!(left_function, right_function)`\n",
                        "  left_function label: `f`,\n",
                        " right_function label: `g`,\n",
                        "                 left: `1`,\n",
                        "                right: `2`"
                    )
                );
            }

        }

    }

}

/// Assert a function output is greater than another.
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
/// // Return Ok
/// let a: i8 = -2;
/// let b: i8 = 1;
/// assert_fn_gt!(i8::abs, a, i8::abs, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a: i8 = 1;
/// let b: i8 = -2;
/// assert_fn_gt!(i8::abs, a, i8::abs, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_gt!(left_function, left_param, right_function, right_param)`\n",
///     "  left_function label: `i8::abs`,\n",
///     "     left_param label: `a`,\n",
///     "     left_param debug: `1`,\n",
///     " right_function label: `i8::abs`,\n",
///     "    right_param label: `b`,\n",
///     "    right_param debug: `-2`,\n",
///     "                 left: `1`,\n",
///     "                right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_gt`]
/// * [`assert_fn_gt_as_result`]
/// * [`debug_assert_fn_gt`]
///
#[macro_export]
macro_rules! assert_fn_gt {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr) => ({
        match assert_fn_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr, $($message:tt)+) => ({
        match assert_fn_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

    //// Arity 0

    ($a_function:path, $b_function:path) => ({
        match assert_fn_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $b_function:path, $($message:tt)+) => ({
        match assert_fn_gt_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
    
}

/// Assert a function output is greater than another.
///
/// This macro provides the same statements as [`assert_fn_gt`],
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
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Related
///
/// * [`assert_fn_gt`]
/// * [`assert_fn_gt`]
/// * [`debug_assert_fn_gt`]
///
#[macro_export]
macro_rules! debug_assert_fn_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_gt!($($arg)*);
        }
    };
}
