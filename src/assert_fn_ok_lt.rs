/// Assert one function ok() is less than another.
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
/// let a: i32 = 1;
/// let b = String::from("2");
/// let x = assert_fn_ok_lt_as_result!(example_digit_to_string, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = 2;
/// let b = String::from("1");
/// let x = assert_fn_ok_lt_as_result!(example_digit_to_string, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_lt!(left_function, left_input, right_expr)`\n",
///     " left_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `2`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"1\"`,\n",
///     "                left: `\"2\"`,\n",
///     "               right: `\"1\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ok_lt_as_result {
    ($a_function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_result = $a_function($a_input);
        let a_is_ok = a_result.is_ok();
        if !a_is_ok {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ok_le!(left_function, left_input, right_expr)`\n",
                    " left_function label: `{}`,\n",
                    "    left_input label: `{}`,\n",
                    "    left_input debug: `{:?}`,\n",
                    "    right_expr label: `{}`,\n",
                    "    right_expr debug: `{:?}`,\n",
                    "         left result: `{:?}`",
                ),
                stringify!($a_function),
                stringify!($a_input), $a_input,
                stringify!($b_expr), $b_expr,
                a_result
            ))
        } else {
            let a_ok = a_result.unwrap();
            if a_ok < $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_ok_lt!(left_function, left_input, right_expr)`\n",
                        " left_function label: `{}`,\n",
                        "    left_input label: `{}`,\n",
                        "    left_input debug: `{:?}`,\n",
                        "    right_expr label: `{}`,\n",
                        "    right_expr debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($a_function),
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
    fn test_assert_fn_ok_lt_as_result_x_arity_2_lt_success() {
        let a: i32 = 1;
        let b = String::from("2");
        let x = assert_fn_ok_lt_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ok_lt_as_result_x_arity_2_eq_failure() {
        let a: i32 = 1;
        let b = String::from("1");
        let x = assert_fn_ok_lt_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ok_lt!(left_function, left_input, right_expr)`\n",
                " left_function label: `example_digit_to_string`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `1`,\n",
                "    right_expr label: `b`,\n",
                "    right_expr debug: `\"1\"`,\n",
                "                left: `\"1\"`,\n",
                "               right: `\"1\"`"
            )
        );
    }
}

/// Assert a function ok() is less than another.
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
/// let a: i32 = 1;
/// let b = String::from("2");
/// assert_fn_ok_lt!(example_digit_to_string, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a: i32 = 2;
/// let b = String::from("1");
/// assert_fn_ok_lt!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ok_lt!(left_function, left_input, right_expr)`\n",
///     " left_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `2`,\n",
///     "    right_expr label: `b`,\n",
///     "    right_expr debug: `\"1\"`,\n",
///     "                left: `\"2\"`,\n",
///     "               right: `\"1\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ok_lt {
    ($a_function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_ok_lt_as_result!($a_function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_fn_ok_lt_as_result!($a_function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
