/// Assert one function ok() is equal to another function ok().
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
/// let a = 10;
/// let b = 10;
/// let x = assert_fn_err_eq_other_as_result!(example_digit_to_string, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = 10;
/// let b = 20;
/// let x = assert_fn_err_eq_other_as_result!(example_digit_to_string, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_eq_other!(pair_function, left_input, right_input)`\n",
///     " pair_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `10`,\n",
///     "   right_input label: `b`,\n",
///     "   right_input debug: `20`,\n",
///     "                left: `\"10 is out of range\"`,\n",
///     "               right: `\"20 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_eq_other_as_result {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        let a_result = $function($a_input);
        let b_result = $function($b_input);
        let a_is_err = a_result.is_err();
        let b_is_err = b_result.is_err();
        if !a_is_err || !b_is_err {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_err_eq_other!(pair_function, left_input, right_input)`\n",
                    " pair_function label: `{}`,\n",
                    "    left_input label: `{}`,\n",
                    "    left_input debug: `{:?}`,\n",
                    "   right_input label: `{}`,\n",
                    "   right_input debug: `{:?}`,\n",
                    "         left result: `{:?}`,\n",
                    "        right result: `{:?}`"
                ),
                stringify!($function),
                stringify!($a_input), $a_input,
                stringify!($b_input), $b_input,
                a_result,
                b_result
            ))
        } else {
            let a_err = a_result.unwrap_err();
            let b_err = b_result.unwrap_err();
            if a_err == b_err {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fn_err_eq_other!(pair_function, left_input, right_input)`\n",
                        " pair_function label: `{}`,\n",
                        "    left_input label: `{}`,\n",
                        "    left_input debug: `{:?}`,\n",
                        "   right_input label: `{}`,\n",
                        "   right_input debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($function),
                    stringify!($a_input), $a_input,
                    stringify!($b_input), $b_input,
                    a_err,
                    b_err
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
    fn test_assert_fn_err_eq_other_as_result_x_arity_2_success() {
        let a: i32 = 10;
        let b: i32 = 10;
        let x = assert_fn_err_eq_other_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_err_eq_other_as_result_x_arity_2_failure() {
        let a: i32 = 10;
        let b: i32 = 20;
        let x = assert_fn_err_eq_other_as_result!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_err_eq_other!(pair_function, left_input, right_input)`\n",
                " pair_function label: `example_digit_to_string`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `10`,\n",
                "   right_input label: `b`,\n",
                "   right_input debug: `20`,\n",
                "                left: `\"10 is out of range\"`,\n",
                "               right: `\"20 is out of range\"`"
            )
        );
    }
}

/// Assert a function err() is equal to another.
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
/// let a: i32 = 10;
/// let b: i32 = 10;
/// assert_fn_err_eq_other!(example_digit_to_string, a, b);
/// //-> ()
///
/// let a: i32 = 10;
/// let b: i32 = 20;
/// let result = panic::catch_unwind(|| {
/// assert_fn_err_eq_other!(example_digit_to_string, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_err_eq_other!(pair_function, left_input, right_input)`\n",
///     " pair_function label: `example_digit_to_string`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `10`,\n",
///     "   right_input label: `b`,\n",
///     "   right_input debug: `20`,\n",
///     "                left: `\"10 is out of range\"`,\n",
///     "               right: `\"20 is out of range\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_err_eq_other {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        match assert_fn_err_eq_other_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_fn_err_eq_other_as_result!($function, $a_input, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
