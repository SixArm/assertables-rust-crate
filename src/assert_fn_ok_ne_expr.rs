/// Assert a function ok() is not equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_fn_ok_ne_expr`]
/// * [`assert_fn_ok_ne_expr_as_result`]
/// * [`debug_assert_fn_ok_ne_expr`]
///
#[macro_export]
macro_rules! assert_fn_ok_ne_expr_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_expr:expr) => ({
        let a_result = $a_function($a_param);
        let a_is_ok = a_result.is_ok();
        if !a_is_ok {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ok_ne_expr!(left_function, left_param, right_expr)`\n",
                    " left_function label: `{}`,\n",
                    "    left_param label: `{}`,\n",
                    "    left_param debug: `{:?}`,\n",
                    "    right_expr label: `{}`,\n",
                    "    right_expr debug: `{:?}`,\n",
                    "         left result: `{:?}`",
                ),
                stringify!($a_function),
                stringify!($a_param), $a_param,
                stringify!($b_expr), $b_expr,
                a_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            if a_ok != $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_ok_ne_expr!(left_function, left_param, right_expr)`\n",
                        " left_function label: `{}`,\n",
                        "    left_param label: `{}`,\n",
                        "    left_param debug: `{:?}`,\n",
                        "    right_expr label: `{}`,\n",
                        "    right_expr debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($a_function),
                    stringify!($a_param), $a_param,
                    stringify!($b_expr), $b_expr,
                    a_ok,
                    $b_expr
                ))
            }
        }
    });

    //// Arity 0

    ($a_function:path, $b_expr:expr) => ({
        let a_result = $a_function();
        let a_is_ok = a_result.is_ok();
        if !a_is_ok {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ok_ne_expr!(left_function, right_expr)`\n",
                    " left_function label: `{}`,\n",
                    "    right_expr label: `{}`,\n",
                    "    right_expr debug: `{:?}`,\n",
                    "         left result: `{:?}`",
                ),
                stringify!($a_function),
                stringify!($b_expr), $b_expr,
                a_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            if a_ok != $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_ok_ne_expr!(left_function, right_expr)`\n",
                        " left_function label: `{}`,\n",
                        "    right_expr label: `{}`,\n",
                        "    right_expr debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($a_function),
                    stringify!($b_expr), $b_expr,
                    a_ok,
                    $b_expr
                ))
            }
        }
    });

}

#[cfg(test)]
mod tests {

    mod assert_fn_ok_ne_expr_as_result {

        mod arity_1 {

            fn f(i: i8) -> Result<i8, i8> {
                return Ok(i);
            }

            #[test]
            fn test_ne() {
                let a: i8 = 1;
                let b: i8 = 2;
                let x = assert_fn_ok_ne_expr_as_result!(f, a, b);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let x = assert_fn_ok_ne_expr_as_result!(f, a, b);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_ok_ne_expr!(left_function, left_param, right_expr)`\n",
                        " left_function label: `f`,\n",
                        "    left_param label: `a`,\n",
                        "    left_param debug: `1`,\n",
                        "    right_expr label: `b`,\n",
                        "    right_expr debug: `1`,\n",
                        "                left: `1`,\n",
                        "               right: `1`"
                    )
                );
            }
        }

        mod arity_0 {

            fn f() -> Result<i8, i8> {
                return Ok(1);
            }

            #[test]
            fn test_ne() {
                let b: i8 = 2;
                let x = assert_fn_ok_ne_expr_as_result!(f, b);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let b: i8 = 1;
                let x = assert_fn_ok_ne_expr_as_result!(f, b);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_ok_ne_expr!(left_function, right_expr)`\n",
                        " left_function label: `f`,\n",
                        "    right_expr label: `b`,\n",
                        "    right_expr debug: `1`,\n",
                        "                left: `1`,\n",
                        "               right: `1`"
                    )
                );
            }
        }
    }
}

/// Assert a function ok() is not equal to an expression.
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
/// fn f(i: i8) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// // Return Ok
/// let a: i8 = 1;
/// let b = String::from("2");
/// assert_fn_ok_ne_expr!(f, a, b);
/// //-> ()
///
/// let a: i8 = 1;
/// let b = String::from("1");
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_ok_ne_expr!(f, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_ne_expr!(left_function, left_param, right_expr)`\n",
///     " left_function label: `f`,\n",
///     "    left_param label: `a`,\n",
///     "    left_param debug: `1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"1\"`,\n",
///     "                left: `\"1\"`,\n",
///     "               right: `\"1\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_ok_ne_expr`]
/// * [`assert_fn_ok_ne_expr_as_result`]
/// * [`debug_assert_fn_ok_ne_expr`]
///
#[macro_export]
macro_rules! assert_fn_ok_ne_expr {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_expr:expr) => ({
        match assert_fn_ok_ne_expr_as_result!($a_function, $a_param, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $a_param:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_ok_ne_expr_as_result!($a_function, $a_param, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

    //// Arity 0

    ($a_function:path, $b_expr:expr) => ({
        match assert_fn_ok_ne_expr_as_result!($a_function, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_ok_ne_expr_as_result!($a_function, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

}

/// Assert a function ok() is not equal to an expression.
///
/// This macro provides the same statements as [`assert_fn_ok_ne_expr`],
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
/// * [`assert_fn_ok_ne_expr`]
/// * [`assert_fn_ok_ne_expr`]
/// * [`debug_assert_fn_ok_ne_expr`]
///
#[macro_export]
macro_rules! debug_assert_fn_ok_ne_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_ok_ne_expr!($($arg)*);
        }
    };
}
