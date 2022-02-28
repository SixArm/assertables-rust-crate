/// Assert one function ok() is not equal to another function ok().
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
/// let a = 10;
/// let b = String::from("20 is out of range");
/// let x = assert_fn_err_ne_as_result!(example_digit_to_string, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = 10;
/// let b = String::from("10 is out of range");
/// let x = assert_fn_err_ne_as_result!(example_digit_to_string, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_ne!(function, left_input, right_expr)`\n",
///     "    function name: `example_digit_to_string`,\n",
///     "  left input name: `a`,\n",
///     "  right expr name: `b`,\n",
///     "       left input: `10`,\n",
///     "       right expr: `\"10 is out of range\"`,\n",
///     "      left output: `\"10 is out of range\"`,\n",
///     "             left: `\"10 is out of range\"`,\n",
///     "            right: `\"10 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_ne_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_result = $function($a_input);
        let a_is_err = a_result.is_err();
        if !a_is_err {
            Err(msg_with_left_function_and_left_input_and_right_expr!(
                "assertion failed",
                "assert_fn_err_ne!",
                stringify!($function),
                stringify!($a_input),
                stringify!($b_expr),
                $a_input,
                $b_expr,
                a_result
            ))
        } else {
            let a_err = a_result.unwrap_err();
            if a_err != $b_expr {
                Ok(())
            } else {
                Err(msg_with_left_function_and_left_input_and_right_expr!(
                    "assertion failed",
                    "assert_fn_err_ne!",
                    stringify!($function),
                    stringify!($a_input),
                    stringify!($b_expr),
                    $a_input,
                    $b_expr,
                    a_err
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
    fn test_assert_fn_err_ne_as_result_x_arity_2_success() {
        let a: i32 = 10;
        let b = String::from("20 is out of range");
        let x = assert_fn_err_ne_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_err_ne_as_result_x_arity_2_failure() {
        let a: i32 = 10;
        let b = String::from("10 is out of range");
        let x = assert_fn_err_ne_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_err_ne!(function, left_input, right_expr)`\n",
                "    function name: `example_digit_to_string`,\n",
                "  left input name: `a`,\n",
                "  right expr name: `b`,\n",
                "       left input: `10`,\n",
                "       right expr: `\"10 is out of range\"`,\n",
                "      left output: `\"10 is out of range\"`,\n",
                "             left: `\"10 is out of range\"`,\n",
                "            right: `\"10 is out of range\"`"
            )
        );
    }
}

/// Assert a function err() is not equal to another.
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
/// let a = 10;
/// let b = String::from("20 is out of range");
/// assert_fn_err_ne!(example_digit_to_string, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = 10;
/// let b = String::from("10 is out of range");
/// assert_fn_err_ne!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_ne!(function, left_input, right_expr)`\n",
///     "    function name: `example_digit_to_string`,\n",
///     "  left input name: `a`,\n",
///     "  right expr name: `b`,\n",
///     "       left input: `10`,\n",
///     "       right expr: `\"10 is out of range\"`,\n",
///     "      left output: `\"10 is out of range\"`,\n",
///     "             left: `\"10 is out of range\"`,\n",
///     "            right: `\"10 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_ne {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_err_ne_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_fn_err_ne_as_result!($function, $a_input, $b_expr) {
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
    fn test_assert_fn_err_ne_x_arity_2_success() {
        let a: i32 = 10;
        let b = String::from("20 is out of range");
        let x = assert_fn_err_ne!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_err_ne!(function, left_input, right_expr)`\n    function name: `example_digit_to_string`,\n  left input name: `a`,\n  right expr name: `b`,\n       left input: `10`,\n       right expr: `\"10 is out of range\"`,\n      left output: `\"10 is out of range\"`,\n             left: `\"10 is out of range\"`,\n            right: `\"10 is out of range\"`")]
    fn test_assert_fn_err_ne_x_arity_2_failure() {
        let a: i32 = 10;
        let b = String::from("10 is out of range");
        let _x = assert_fn_err_ne!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_fn_err_ne_x_arity_3_success() {
        let a: i32 = 10;
        let b = String::from("20 is out of range");
        let x = assert_fn_err_ne!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_err_ne_x_arity_3_failure() {
        let a: i32 = 10;
        let b = String::from("10 is out of range");
        let _x = assert_fn_err_ne!(example_digit_to_string, a, b, "message");
    }

}
