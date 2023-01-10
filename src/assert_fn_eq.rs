/// Assert a function output is equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a: i32 = -1;
/// let b: i32 = 1;
/// let x = assert_fn_eq_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// assert_eq!(x.unwrap(), ());
///
/// let a: i32 = -1;
/// let b: i32 = 2;
/// let x = assert_fn_eq_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_eq!(left_function, left_input, right_expr)`\n",
///     " left_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `2`,\n",
///     "                left: `1`,\n",
///     "               right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_fn_eq`]
/// * [`assert_fn_eq_as_result`]
/// * [`debug_assert_fn_eq`]
///
#[macro_export]
macro_rules! assert_fn_eq_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_output = $function($a_input);
        if a_output == $b_expr {
            Ok(())
        } else {
            let a_output = $function($a_input);
            if a_output == $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_eq!(left_function, left_input, right_expr)`\n",
                        " left_function label: `{}`,\n",
                        "    left_input label: `{}`,\n",
                        "    left_input debug: `{:?}`,\n",
                        "    right_expr label: `{}`,\n",
                        "    right_expr debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($function),
                    stringify!($a_input), $a_input,
                    stringify!($b_expr), $b_expr,
                    a_output,
                    $b_expr
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_eq_as_result_x_success() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_eq_as_result!(i32::abs, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_eq_as_result_x_failure() {
        let a: i32 = -1;
        let b: i32 = 2;
        let x = assert_fn_eq_as_result!(i32::abs, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_eq!(left_function, left_input, right_expr)`\n",
                " left_function label: `i32::abs`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `-1`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `2`,\n",
                "                left: `1`,\n",
                "               right: `2`"
            )
        );
    }
}

/// Assert a function output is equal to an expression.
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
/// let a: i32 = -1;
/// let b: i32 = 1;
/// assert_fn_eq!(i32::abs, a, b);
/// //-> ()
///
/// let a: i32 = -1;
/// let b: i32 = 2;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_eq!(i32::abs, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_eq!(left_function, left_input, right_expr)`\n",
///     " left_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `2`,\n",
///     "                left: `1`,\n",
///     "               right: `2`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_eq!(i32::abs, a, b, "message");
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
/// * [`assert_fn_eq`]
/// * [`assert_fn_eq_as_result`]
/// * [`debug_assert_fn_eq`]
///
#[macro_export]
macro_rules! assert_fn_eq {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_eq_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_eq_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a function output is equal to an expression.
///
/// This macro provides the same statements as [`assert_fn_eq`],
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
/// * [`assert_fn_eq`]
/// * [`assert_fn_eq`]
/// * [`debug_assert_fn_eq`]
/// 
#[macro_export]
macro_rules! debug_assert_fn_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_eq!($($arg)*);
        }
    };
}
