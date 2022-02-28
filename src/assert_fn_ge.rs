/// Assert one function output is not equal to another function output.
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
/// let a: i32 = -2;
/// let b: i32 = 1;
/// let x = assert_fn_ge_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = 1;
/// let b: i32 = -2;
/// let x = assert_fn_ge_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ge_other!(function, left_input, right_input)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input name: `a`,\n",
///     " right input name: `b`,\n",
///     "       left input: `1`,\n",
///     "      right input: `-2`,\n",
///     "      left output: `1`,\n",
///     "     right output: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ge_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_output = $function($a_input);
        if a_output >= $b_expr {
            Ok(())
        } else {
            Err(format!("assertion failed: `assert_fn_ge!(function, left_input, right_value)`

    function name: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_output, $b_expr))
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        let a_output = $function($a_input);
        if a_output >= $b_expr {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_2_gt_success() {
        let a: i32 = -2;
        let b: i32 = 1;
        let x = assert_fn_ge_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_2_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_ge_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ge_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertion failed: `assert_fn_ge!(function, left_input, right_value)`

    function name: `i32::abs`,\n   left input: `1`,\n  right input: `-2`,\n  left output: `1`,\n right output: `2`"
        );
    }

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_3_gt_success() {
        let a: i32 = -2;
        let b: i32 = 1;
        let x = assert_fn_ge_as_result!(i32::abs, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_3_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_ge_as_result!(i32::abs, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ge_as_result_x_arity_3_lt_failure() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ge_as_result!(i32::abs, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}

/// Assert a function output is greater than or equal to another.
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
/// let a: i32 = -2;
/// let b: i32 = 1;
/// assert_fn_ge!(i32::abs, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a: i32 = 1;
/// let b: i32 = -2;
/// assert_fn_ge!(i32::abs, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ge!(function, left_input, right_expr)`\n",
///     "    function name: `i32::abs`,\n",
///     "   left input: `1`,\n",
///     "  right value: `-2`,\n",
///     "         left: `1`,\n",
///     "        right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ge {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_eq_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_fn_ge_x_arity_2_gt_success() {
        let a: i32 = -2;
        let b: i32 = 1;
        let x = assert_fn_ge!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_fn_ge_x_arity_2_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_ge!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ge!(function, left_input, right_expr)`\n    function name: `i32::abs`,\n   left input: `1`,\n  right value: `-2`,\n         left: `1`,\n        right: `2`")]
    fn test_assert_fn_ge_x_arity_2_lt_failure() {
        let a: i32 = 1;
        let b: i32 = -2;
        let _x = assert_fn_ge!(i32::abs, a, b);
    }
}
