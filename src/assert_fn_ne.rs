/// Assert one function output is not equal to another function output.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a: i32 = 1;
/// let b: i32 = -2;
/// let x = assert_fn_ne_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = -1;
/// let b: i32 = 1;
/// let x = assert_fn_ne_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ne!(left_function, left_input, right_expr)`\n",
///     " left_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `1`,\n",
///     "                left: `1`,\n",
///     "               right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ne_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_output = $function($a_input);
        if a_output != $b_expr {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ne!(left_function, left_input, right_expr)`\n",
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
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_ne_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ne_as_result!(i32::abs, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ne_as_result_x_failure() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_ne_as_result!(i32::abs, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ne!(left_function, left_input, right_expr)`\n",
                " left_function label: `i32::abs`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `-1`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `1`,\n",
                "                left: `1`,\n",
                "               right: `1`"
            )
        );
    }
}

/// Assert a function output is not equal to another.
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
/// let a: i32 = 1;
/// let b: i32 = -2;
/// assert_fn_ne!(i32::abs, a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a: i32 = -1;
/// let b: i32 = 1;
/// assert_fn_ne!(i32::abs, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ne!(left_function, left_input, right_expr)`\n",
///     " left_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-1`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `1`,\n",
///     "                left: `1`,\n",
///     "               right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ne {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_ne_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_ne_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
