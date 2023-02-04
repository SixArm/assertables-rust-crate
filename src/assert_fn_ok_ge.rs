/// Assert a function ok() is greater than or equal to another.
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
/// * [`assert_fn_ok_ge`]
/// * [`assert_fn_ok_ge_as_result`]
/// * [`debug_assert_fn_ok_ge`]
///
#[macro_export]
macro_rules! assert_fn_ok_ge_as_result {
    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr $(,)?) => ({
        let a_result = $a_function($a_input);
        let b_result = $a_function($b_input);
        let a_is_ok = a_result.is_ok();
        let b_is_ok = b_result.is_ok();
        if !a_is_ok || !b_is_ok {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_err_ge!(left_function, left_input, right_function, right_input)`\n",
                    "  left_function label: `{}`,\n",
                    "     left_input label: `{}`,\n",
                    "     left_input debug: `{:?}`,\n",
                    " right_function label: `{}`,\n",
                    "    right_input label: `{}`,\n",
                    "    right_input debug: `{:?}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($a_input), $a_input,
                stringify!($b_function),
                stringify!($b_input), $b_input,
                a_result,
                b_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            let b_ok = b_result.unwrap();
            if a_ok >= b_ok {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_ok_ge!(left_function, left_input, right_function, right_input)`\n",
                        "  left_function label: `{}`,\n",
                        "     left_input label: `{}`,\n",
                        "     left_input debug: `{:?}`,\n",
                        " right_function label: `{}`,\n",
                        "    right_input label: `{}`,\n",
                        "    right_input debug: `{:?}`,\n",
                        "                 left: `{:?}`,\n",
                        "                right: `{:?}`"
                    ),
                    stringify!($a_function),
                    stringify!($a_input), $a_input,
                    stringify!($b_function),
                    stringify!($b_input), $b_input,
                    a_ok,
                    b_ok
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    fn example_digit_to_string(i: i32) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_fn_ok_ge_as_result_x_success_because_gt() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_fn_ok_ge_as_result!(example_digit_to_string, a, example_digit_to_string, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ok_ge_as_result_x_success_because_eq() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_fn_ok_ge_as_result!(example_digit_to_string, a, example_digit_to_string, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ok_ge_as_result_x_failure_because_lt() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_fn_ok_ge_as_result!(example_digit_to_string, a, example_digit_to_string, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ok_ge!(left_function, left_input, right_function, right_input)`\n",
                "  left_function label: `example_digit_to_string`,\n",
                "     left_input label: `a`,\n",
                "     left_input debug: `1`,\n",
                " right_function label: `example_digit_to_string`,\n",
                "    right_input label: `b`,\n",
                "    right_input debug: `2`,\n",
                "                 left: `\"1\"`,\n",
                "                right: `\"2\"`"
            )
        );
    }
}

/// Assert a function ok() is greater than or equal to another.
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
/// fn example_digit_to_string(i: i32) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// // Return Ok
/// let a: i32 = 2;
/// let b: i32 = 1;
/// assert_fn_ok_ge!(example_digit_to_string, a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a: i32 = 1;
/// let b: i32 = 2;
/// assert_fn_ok_ge!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_ge!(left_function, left_input, right_function, right_input)`\n",
///     "  left_function label: `example_digit_to_string`,\n",
///     "     left_input label: `a`,\n",
///     "     left_input debug: `1`,\n",
///     " right_function label: `example_digit_to_string`,\n",
///     "    right_input label: `b`,\n",
///     "    right_input debug: `2`,\n",
///     "                 left: `\"1\"`,\n",
///     "                right: `\"2\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_ok_ge`]
/// * [`assert_fn_ok_ge_as_result`]
/// * [`debug_assert_fn_ok_ge`]
///
#[macro_export]
macro_rules! assert_fn_ok_ge {
    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr $(,)?) => ({
        match assert_fn_ok_ge_as_result!($a_function, $a_input, $b_function, $b_input) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr, $($message:tt)+) => ({
        match assert_fn_ok_ge_as_result!($a_function, $a_input, $b_function, $b_input) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a function ok() is greater than or equal to another.
///
/// This macro provides the same statements as [`assert_fn_ok_ge`],
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
/// * [`assert_fn_ok_ge`]
/// * [`assert_fn_ok_ge`]
/// * [`debug_assert_fn_ok_ge`]
///
#[macro_export]
macro_rules! debug_assert_fn_ok_ge {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_ok_ge!($($arg)*);
        }
    };
}
