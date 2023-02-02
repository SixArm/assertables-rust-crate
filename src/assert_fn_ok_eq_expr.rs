/// Assert a function ok() is equal to an expression.
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
/// * [`assert_fn_ok_eq_expr`]
/// * [`assert_fn_ok_eq_expr_as_result`]
/// * [`debug_assert_fn_ok_eq_expr`]
///
#[macro_export]
macro_rules! assert_fn_ok_eq_expr_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_result = $function($a_input);
        let a_is_ok = a_result.is_ok();
        if !a_is_ok {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ok_eq_expr!(left_function, left_input, right_expr)`\n",
                    " left_function label: `{}`,\n",
                    "    left_input label: `{}`,\n",
                    "    left_input debug: `{:?}`,\n",
                    "    right_expr label: `{}`,\n",
                    "    right_expr debug: `{:?}`,\n",
                    "         left result: `{:?}`",
                ),
                stringify!($function),
                stringify!($a_input), $a_input,
                stringify!($b_expr), $b_expr,
                a_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            if a_ok == $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_ok_eq_expr!(left_function, left_input, right_expr)`\n",
                        " left_function label: `{}`,\n",
                        "    left_input label: `{}`,\n",
                        "    left_input debug: `{:?}`,\n",
                        "    right_expr label: `{}`,\n",
                        "    right_expr debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($function),
                    stringify!($a_input), $a_input,
                    stringify!($b_expr), $b_expr,
                    a_ok,
                    $b_expr
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
    fn test_assert_fn_ok_eq_expr_as_result_x_success() {
        let a: i32 = 1;
        let b = String::from("1");
        let x = assert_fn_ok_eq_expr_as_result!(example_digit_to_string, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ok_eq_expr_as_result_x_failure() {
        let a: i32 = 1;
        let b = String::from("2");
        let x = assert_fn_ok_eq_expr_as_result!(example_digit_to_string, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ok_eq_expr!(left_function, left_input, right_expr)`\n",
                " left_function label: `example_digit_to_string`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `1`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `\"2\"`,\n",
                "                left: `\"1\"`,\n",
                "               right: `\"2\"`"
            )
        );
    }
}

/// Assert a function ok() is equal to an expression.
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
/// let a = 1;
/// let b = String::from("1");
/// assert_fn_ok_eq_expr!(example_digit_to_string, a, b);
/// //-> ()
///
/// let a = 1;
/// let b = String::from("2");
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_ok_eq_expr!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_eq_expr!(left_function, left_input, right_expr)`\n",
///     " left_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"2\"`,\n",
///     "                left: `\"1\"`,\n",
///     "               right: `\"2\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_ok_eq_expr`]
/// * [`assert_fn_ok_eq_expr_as_result`]
/// * [`debug_assert_fn_ok_eq_expr`]
///
#[macro_export]
macro_rules! assert_fn_ok_eq_expr {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_ok_eq_expr_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_ok_eq_expr_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a function ok() is equal to an expression.
///
/// This macro provides the same statements as [`assert_fn_ok_eq_expr`],
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
/// * [`assert_fn_ok_eq_expr`]
/// * [`assert_fn_ok_eq_expr`]
/// * [`debug_assert_fn_ok_eq_expr`]
///
#[macro_export]
macro_rules! debug_assert_fn_ok_eq_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_ok_eq_expr!($($arg)*);
        }
    };
}
