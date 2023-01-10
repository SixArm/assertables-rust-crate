/// Assert one function ok() is greater than another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
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
/// // Return Ok
/// let a = 20;
/// let b = String::from("10 is out of range");
/// let x = assert_fn_err_gt_as_result!(example_digit_to_string, a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = 10;
/// let b = String::from("20 is out of range");
/// let x = assert_fn_err_gt_as_result!(example_digit_to_string, a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
///     " left_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `10`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"20 is out of range\"`,\n",
///     "                left: `\"10 is out of range\"`,\n",
///     "               right: `\"20 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_gt_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_result = $function($a_input);
        let a_is_err = a_result.is_err();
        if !a_is_err {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
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
            let a_err = a_result.unwrap_err();
            if a_err > $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
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
                    a_err,
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
    fn test_assert_fn_err_gt_as_result_x_success() {
        let a: i32 = 20;
        let b = String::from("10 is out of range");
        let x = assert_fn_err_gt_as_result!(example_digit_to_string, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_err_gt_as_result_x_failure_because_eq() {
        let a: i32 = 10;
        let b = String::from("10 is out of range");
        let x = assert_fn_err_gt_as_result!(example_digit_to_string, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
                " left_function label: `example_digit_to_string`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `10`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `\"10 is out of range\"`,\n",
                "                left: `\"10 is out of range\"`,\n",
                "               right: `\"10 is out of range\"`"
            )
        );
    }

    #[test]
    fn test_assert_fn_err_gt_as_result_x_failure_because_lt() {
        let a: i32 = 10;
        let b = String::from("20 is out of range");
        let x = assert_fn_err_gt_as_result!(example_digit_to_string, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
                " left_function label: `example_digit_to_string`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `10`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `\"20 is out of range\"`,\n",
                "                left: `\"10 is out of range\"`,\n",
                "               right: `\"20 is out of range\"`"
            )
        );
    }
}

/// Assert a function err() is greater than another.
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
/// let a = 20;
/// let b = String::from("10 is out of range");
/// assert_fn_err_gt!(example_digit_to_string, a, b);
/// //-> ()
///
/// let a = 10;
/// let b = String::from("20 is out of range");
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_err_gt!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_gt!(left_function, left_input, right_expr)`\n",
///     " left_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `10`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"20 is out of range\"`,\n",
///     "                left: `\"10 is out of range\"`,\n",
///     "               right: `\"20 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// 
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_fn_err_gt!(example_digit_to_string, a, b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_gt {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_err_gt_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fn_err_gt_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
