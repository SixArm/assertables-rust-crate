/// Assert one function output is less than or equal to another function output.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
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
/// let x = assert_fn_le_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// # assert_eq!(x.unwrap(), ());
///
/// let a: i32 = -2;
/// let b: i32 = 1;
/// let x = assert_fn_le_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// assert_eq!actual, expect);

/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_le!(function, left_input, right_value)`

    function name: `i32::abs`,\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`".to_string());
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_le_as_result {
    ($function:path, $a_input:expr, $b_expr:expr $(,)?) => ({
        let a_output = $function($a_input);
        if a_output <= $b_expr {
            Ok(())
        } else {
            Err(format!("assertion failed: `assert_fn_le!(function, left_input, right_value)`

    function name: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_output, $b_expr))
        }
    });
}

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_fn_le_as_result_x_arity_2_lt_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_le_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_le_as_result_x_arity_2_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_le_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_fn_le_as_result_x_arity_2_gt_failure() {
        let a: i32 = -2;
        let b: i32 = 1;
        let x = assert_fn_le_as_result!(i32::abs, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertion failed: `assert_fn_le!(function, left_input, right_value)`

    function name: `i32::abs`,\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`"
        );
    }
}

/// Assert a function output is less than or equal to another.
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
/// assert_fn_le!(i32::abs, a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a: i32 = -2;
/// let b: i32 = 1;
/// assert_fn_le!(i32::abs, a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_le!(function, left_input, right_expr)`\n",
///     "    function name: `i32::abs`,\n",
///     "  left input: `-2`,\n",
///     " right value: `1`,\n",
///     "        left: `2`,\n",
///     "       right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_fn_le {
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
    fn test_assert_fn_le_x_arity_2_lt_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_le!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_fn_le_x_arity_2_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_le!(i32::abs, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_le!(function, left_input, right_expr)`\n    function name: `i32::abs`,\n   left input: `-2`,\n  right value: `1`,\n         left: `2`,\n        right: `1`")]
    fn test_assert_fn_le_x_arity_2_gt_failure() {
        let a: i32 = -2;
        let b: i32 = 1;
        let _x = assert_fn_le!(i32::abs, a, b);
    }

    #[test]
    fn test_assert_fn_le_x_arity_3_lt_success() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_le!(i32::abs, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_fn_le_x_arity_3_eq_success() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_le!(i32::abs, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_le_x_arity_3_failure() {
        let a: i32 = -2;
        let b: i32 = 1;
        let _x = assert_fn_le!(i32::abs, a, b, "message");
    }

}
