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
/// let a: i32 = 1;
/// let b: i32 = -2;
/// let x = assert_fn_ne_other_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = 1;
/// let b: i32 = -1;
/// let x = assert_fn_ne_other_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ne_other!(function, left_input, right_input)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input name: `a`,\n",
///     " right input name: `b`,\n",
///     "       left input: `1`,\n",
///     "      right input: `-1`,\n",
///     "      left output: `1`,\n",
///     "     right output: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ne_other_as_result {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        let a_output = $function($a_input);
        let b_output = $function($b_input);
        if a_output != b_output {
            Ok(())
        } else {
            Err(msg_with_pair_function_and_left_input_and_right_input!(
                "assertion failed",
                "assert_fn_ne_other!",
                stringify!($function),
                stringify!($a_input),
                stringify!($b_input),
                $a_input,
                $b_input,
                a_output,
                b_output
            ))
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_ne_other_as_result_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ne_other_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_ne_other_as_result_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_ne_other_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ne_other!(function, left_input, right_input)`\n",
                "    function name: `i32::abs`,\n",
                "  left input name: `a`,\n",
                " right input name: `b`,\n",
                "       left input: `1`,\n",
                "      right input: `-1`,\n",
                "      left output: `1`,\n",
                "     right output: `1`"
            )
        );
    }
}

/// Assert a function output is not equal to another.
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
/// let a: i32 = 1;
/// let b: i32 = -2;
/// assert_fn_ne_other!(i32::abs, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a: i32 = 1;
/// let b: i32 = -1;
/// assert_fn_ne_other!(i32::abs, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ne_other!(function, left_input, right_input)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input name: `a`,\n",
///     " right input name: `b`,\n",
///     "       left input: `1`,\n",
///     "      right input: `-1`,\n",
///     "      left output: `1`,\n",
///     "     right output: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_ne_other {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        match assert_fn_ne_other_as_result!($function, $a_input, $b_input) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_input:expr, $($arg:tt)+) => ({
        match assert_fn_ne_other_as_result!($function, $a_input, $b_input) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_fn_ne_other_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ne_other!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ne_other!(function, left_input, right_input)`\n    function name: `i32::abs`,\n  left input name: `a`,\n right input name: `b`,\n       left input: `1`,\n      right input: `-1`,\n      left output: `1`,\n     right output: `1`")]
    fn test_assert_fn_ne_other_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = -1;
        let _x = assert_fn_ne_other!(i32::abs, a, b);
    }

    #[test]
    fn test_assert_fn_ne_other_x_arity_3_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_ne_other!(i32::abs, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ne_other_x_arity_3_failure() {
        let a: i32 = 1;
        let b: i32 = -1;
        let _x = assert_fn_ne_other!(i32::abs, a, b, "message");
    }

}
