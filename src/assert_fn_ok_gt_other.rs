/// Assert one function ok() is greater than another.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// fn example_digit_to_string(i: i32) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// let a: i32 = 2;
/// let b: i32 = 1;
/// let x = assert_fn_ok_gt_other_as_result!(example_digit_to_string, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = 1;
/// let b: i32 = 2;
/// let x = assert_fn_ok_gt_other_as_result!(example_digit_to_string, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n",
///     "    function name: `example_digit_to_string`,\n",
///     "  left input name: `a`,\n",
///     " right input name: `b`,\n",
///     "       left input: `1`,\n",
///     "      right input: `2`,\n",
///     "      left output: `\"1\"`,\n",
///     "     right output: `\"2\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ok_gt_other_as_result {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        let a_result = $function($a_input);
        let b_result = $function($b_input);
        let a_is_ok = a_result.is_ok();
        let b_is_ok = b_result.is_ok();
        if !a_is_ok || !b_is_ok {
            Err(msg_with_pair_function_and_left_input_and_right_input!(
                "assertion failed",
                "assert_fn_ok_gt_other!",
                stringify!($function),
                stringify!($a_input),
                stringify!($b_input),
                $a_input,
                $b_input,
                a_result,
                b_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            let b_ok = b_result.unwrap();
            if a_ok > b_ok {
                Ok(())
            } else {
                Err(msg_with_pair_function_and_left_input_and_right_input!(
                    "assertion failed",
                    "assert_fn_ok_gt_other!",
                    stringify!($function),
                    stringify!($a_input),
                    stringify!($b_input),
                    $a_input,
                    $b_input,
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
    fn test_assert_fn_ok_gt_other_as_result_x_arity_2_gt_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_fn_ok_gt_other_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ok_gt_other_as_result_x_arity_2_eq_failure() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_fn_ok_gt_other_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n",
                "    function name: `example_digit_to_string`,\n",
                "  left input name: `a`,\n",
                " right input name: `b`,\n",
                "       left input: `1`,\n",
                "      right input: `1`,\n",
                "      left output: `\"1\"`,\n",
                "     right output: `\"1\"`"
            )
        );
    }

    #[test]
    fn test_assert_fn_ok_gt_other_as_result_x_arity_2_lt_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_fn_ok_gt_other_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n",
                "    function name: `example_digit_to_string`,\n",
                "  left input name: `a`,\n",
                " right input name: `b`,\n",
                "       left input: `1`,\n",
                "      right input: `2`,\n",
                "      left output: `\"1\"`,\n",
                "     right output: `\"2\"`"
            )
        );
    }
}

/// Assert a function ok() is greater than another.
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
/// fn example_digit_to_string(i: i32) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// let a: i32 = 2;
/// let b: i32 = 1;
/// assert_fn_ok_gt_other!(example_digit_to_string, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a: i32 = 1;
/// let b: i32 = 2;
/// assert_fn_ok_gt_other!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n",
///     "    function name: `example_digit_to_string`,\n",
///     "  left input name: `a`,\n",
///     " right input name: `b`,\n",
///     "       left input: `1`,\n",
///     "      right input: `2`,\n",
///     "      left output: `\"1\"`,\n",
///     "     right output: `\"2\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ok_gt_other {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_ok_gt_other_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_fn_ok_gt_other_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    fn example_digit_to_string(i: i32) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_fn_ok_gt_other_x_arity_2_gt_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_fn_ok_gt_other!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n    function name: `example_digit_to_string`,\n  left input name: `a`,\n right input name: `b`,\n       left input: `1`,\n      right input: `1`,\n      left output: `\"1\"`,\n     right output: `\"1\"`")]
    fn test_assert_fn_ok_gt_other_x_arity_2_eq_failure() {
        let a: i32 = 1;
        let b: i32 = 1;
        let _x = assert_fn_ok_gt_other!(example_digit_to_string, a, b);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ok_gt_other!(function, left_input, right_input)`\n    function name: `example_digit_to_string`,\n  left input name: `a`,\n right input name: `b`,\n       left input: `1`,\n      right input: `2`,\n      left output: `\"1\"`,\n     right output: `\"2\"`")]
    fn test_assert_fn_ok_gt_other_x_arity_2_lt_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let _x = assert_fn_ok_gt_other!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_fn_ok_gt_other_x_arity_3_gt_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_fn_ok_gt_other!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ok_gt_other_x_arity_3_eq_failure() {
        let a: i32 = 1;
        let b: i32 = 1;
        let _x = assert_fn_ok_gt_other!(example_digit_to_string, a, b, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ok_gt_other_x_arity_3_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let _x = assert_fn_ok_gt_other!(example_digit_to_string, a, b, "message");
    }

}
