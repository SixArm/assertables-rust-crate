/// Assert a function ok() is greater than or equal to another.
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
/// fn example_digit_to_string(i: isize) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// assert_f_ok_ge!(example_digit_to_string, 2, 1);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_ok_ge!(example_digit_to_string, 1, 2);
/// //-> panic!("…")
/// // assertion failed: `assert_f_ok_ge!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `1`,
/// //   right input: `2`,
/// //   left output: `\"1\"`,
/// //  right output: `\"2\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_ok_ge!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `2`,\n  left output: `\"1\"`,\n right output: `\"2\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_ok_ge {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if !left_output.is_ok() || !right_output.is_ok() {
            panic!("assertion failed: `assert_f_ok_ge!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is ok: `{:?}`,\n right output is ok: `{:?}`", stringify!($function), $left, $right, left_output.is_ok(), right_output.is_ok());
        } else {
            let left_output = left_output.unwrap();
            let right_output = right_output.unwrap();
            if (left_output >= right_output) {
                ()
            } else {
                panic!("assertion failed: `assert_f_ok_ge!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left_output, right_output);
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if !left_output.is_ok() || !right_output.is_ok() {
            panic!("{:?}", $($arg)+)
        } else {
            let left_output = left_output.unwrap();
            let right_output = right_output.unwrap();
            if (left_output >= right_output) {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    fn example_digit_to_string(i: isize) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_f_ok_ge_x_arity_2_gt_success() {
        let a = 2;
        let b = 1;
        let x = assert_f_ok_ge!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_f_ok_ge_x_arity_2_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_f_ok_ge!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_ok_ge!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `2`,\n  left output: `\"1\"`,\n right output: `\"2\"`")]
    fn test_assert_f_ok_ge_x_arity_2_lt_failure() {
        let a = 1;
        let b = 2;
        let _x = assert_f_ok_ge!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_f_ok_ge_x_arity_3_gt_success_gt() {
        let a = 2;
        let b = 1;
        let x = assert_f_ok_ge!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_f_ok_ge_x_arity_3_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_f_ok_ge!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_ok_ge_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let _x = assert_f_ok_ge!(example_digit_to_string, a, b, "message");
    }

}
