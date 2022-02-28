/// Assert a function output is equal to a given.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a: i32 = -1;
/// let b: i32 = 1;
/// let x = assert_fn_eq_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let a: i32 = -1;
/// let b: i32 = 2;
/// let x = assert_fn_eq_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_eq!(function, left_input, right_expr)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input name: `a`,\n",
///     "  right expr name: `b`,\n",
///     "       left input: `-1`,\n",
///     "       right expr: `2`,\n",
///     "      left output: `1`,\n",
///     "             left: `1`,\n",
///     "            right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
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
                Err(msg_with_left_function_and_left_input_and_right_expr!(
                    "assertion failed",
                    "assert_fn_eq!",
                    stringify!($function),
                    stringify!($a_input),
                    stringify!($b_expr),
                    $a_input,
                    $b_expr,
                    a_output
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_eq_as_result_x_arity_2_success() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_eq_as_result!(i32::abs, a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assert_fn_eq_as_result_x_arity_2_failure() {
        let a: i32 = -1;
        let b: i32 = 2;
        let x = assert_fn_eq_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_eq!(function, left_input, right_expr)`\n",
                "    function name: `i32::abs`,\n",
                "  left input name: `a`,\n",
                "  right expr name: `b`,\n",
                "       left input: `-1`,\n",
                "       right expr: `2`,\n",
                "      left output: `1`,\n",
                "             left: `1`,\n",
                "            right: `2`"
            )
        );
    }
}

/// Assert a function output is equal to a given value.
///
/// * When true, return `()`.
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
/// let a: i32 = -1;
/// let b: i32 = 1;
/// assert_fn_eq!(i32::abs, a, b);
/// //-> ()
///
/// let a: i32 = -1;
/// let b: i32 = 2;
/// let result = panic::catch_unwind(|| {
/// assert_fn_eq!(i32::abs, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_eq!(function, left_input, right_expr)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input name: `a`,\n",
///     "  right expr name: `b`,\n",
///     "       left input: `-1`,\n",
///     "       right expr: `2`,\n",
///     "      left output: `1`,\n",
///     "             left: `1`,\n",
///     "            right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_eq {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_eq_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_fn_eq_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_fn_eq_x_arity_2_eq_success() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_eq!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_eq!(function, left_input, right_expr)`\n    function name: `i32::abs`,\n  left input name: `a`,\n  right expr name: `b`,\n       left input: `-1`,\n       right expr: `2`,\n      left output: `1`,\n             left: `1`,\n            right: `2`")]
    fn test_assert_fn_eq_x_arity_2_ne_failure() {
        let a: i32 = -1;
        let b: i32 = 2;
        let _x = assert_fn_eq!(i32::abs, a, b);
    }

    #[test]
    fn test_assert_fn_eq_x_arity_3_eq_success() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_eq!(i32::abs, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_eq_x_arity_3_ne_failure() {
        let a: i32 = -1;
        let b: i32 = 2;
        let _x = assert_fn_eq!(i32::abs, a, b, "message");
    }

}
