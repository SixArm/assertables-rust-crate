/// Assert a function err() is greater than or equal to another.
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
/// * [`assert_fn_err_ge`]
/// * [`assert_fn_err_ge_as_result`]
/// * [`debug_assert_fn_err_ge`]
///
#[macro_export]
macro_rules! assert_fn_err_ge_as_result {

    //// Arity 1
    
    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr) => ({
        let a_result = $a_function($a_param);
        let b_result = $b_function($b_param);
        let a_is_err = a_result.is_err();
        let b_is_err = b_result.is_err();
        if !a_is_err || !b_is_err {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_err_eq!(left_function, left_param, right_function, right_param)`\n",
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
                a_result,
                b_result
            ))
        } else {
            let a_err = a_result.unwrap_err();
            let b_err = b_result.unwrap_err();
            if a_err >= b_err {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_err_ge!(left_function, left_param, right_function, right_param)`\n",
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
                    a_err,
                    b_err
                ))
            }
        }
    });
    
    //// Arity 0
    
    ($a_function:path, $b_function:path) => ({
        let a_result = $a_function();
        let b_result = $b_function();
        let a_is_err = a_result.is_err();
        let b_is_err = b_result.is_err();
        if !a_is_err || !b_is_err {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_err_eq!(left_function, right_function)`\n",
                    "  left_function label: `{}`,\n",
                    " right_function label: `{}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($b_function),
                a_result,
                b_result
            ))
        } else {
            let a_err = a_result.unwrap_err();
            let b_err = b_result.unwrap_err();
            if a_err >= b_err {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_err_ge!(left_function, right_function)`\n",
                        "  left_function label: `{}`,\n",
                        " right_function label: `{}`,\n",
                        "                 left: `{:?}`,\n",
                        "                right: `{:?}`"
                    ),
                    stringify!($a_function),
                    stringify!($b_function),
                    a_err,
                    b_err
                ))
            }
        }
    });
    
}

#[cfg(test)]
mod tests {

    mod assert_fn_err_ge_as_result {
        
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
                let x = assert_fn_err_ge_as_result!(f, a, g, b);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let a: i8 = 1;
                let b: i8 = 1;
                let x = assert_fn_err_ge_as_result!(f, a, g, b);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_lt() {
                let a: i8 = 1;
                let b: i8 = 2;
                let x = assert_fn_err_ge_as_result!(f, a, g, b);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_ge!(left_function, left_param, right_function, right_param)`\n",
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
                    
            fn f() -> Result<i8, i8> {
                Err(1)
            }

            fn g() -> Result<i8, i8> {
                Err(2)
            }

            #[test]
            fn test_gt() {
                let x = assert_fn_err_ge_as_result!(g, f);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_eq() {
                let x = assert_fn_err_ge_as_result!(f, f);
                assert_eq!(x, Ok(()));
            }

            #[test]
            fn test_lt() {
                let x = assert_fn_err_ge_as_result!(f, g);
                assert!(x.is_err());
                assert_eq!(
                    x.unwrap_err(),
                    concat!(
                        "assertion failed: `assert_fn_err_ge!(left_function, right_function)`\n",
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

/// Assert a function err() is greater than or equal to another.
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
/// let a: i8 = 20;
/// let b: i8 = 10;
/// assert_fn_err_ge!(f, a, f, b);
/// //-> ()
///
/// let a: i8 = 10;
/// let b: i8 = 20;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_err_ge!(f, a, f, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_ge!(left_function, left_param, right_function, right_param)`\n",
///     "  left_function label: `f`,\n",
///     "     left_param label: `a`,\n",
///     "     left_param debug: `10`,\n",
///     " right_function label: `f`,\n",
///     "    right_param label: `b`,\n",
///     "    right_param debug: `20`,\n",
///     "                 left: `\"10 is out of range\"`,\n",
///     "                right: `\"20 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_err_ge!(f, a, f, b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_err_ge`]
/// * [`assert_fn_err_ge_as_result`]
/// * [`debug_assert_fn_err_ge`]
///
#[macro_export]
macro_rules! assert_fn_err_ge {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr) => ({
        match assert_fn_err_ge_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr, $($message:tt)+) => ({
        match assert_fn_err_ge_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

    //// Arity 0

    ($a_function:path, $b_function:path) => ({
        match assert_fn_err_ge_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $b_function:path, $($message:tt)+) => ({
        match assert_fn_err_ge_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

}

/// Assert a function err() is greater than or equal to another.
///
/// This macro provides the same statements as [`assert_fn_err_ge`],
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
/// * [`assert_fn_err_ge`]
/// * [`assert_fn_err_ge`]
/// * [`debug_assert_fn_err_ge`]
///
#[macro_export]
macro_rules! debug_assert_fn_err_ge {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_err_ge!($($arg)*);
        }
    };
}
