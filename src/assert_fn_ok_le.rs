/// Assert a function ok() is less than or equal to another.
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
///
/// # fn main() {
/// assert_fn_ok_le!(example_digit_to_string, 1, 2);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// use std::str::FromStr;
/// assert_fn_ok_le!(example_digit_to_string, 2, 1);
/// //-> panic!("…")
/// // assertion failed: `assert_fn_ok_le!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `2`,
/// //   right input: `1`,
/// //   left output: `\"2\"`,
/// //  right output: `\"1\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_fn_ok_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `2`,\n  right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_ok_le {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_ok = a_output.is_ok();
        let b_is_ok = b_output.is_ok();
        if !a_is_ok || !b_is_ok {
            panic!("assertion failed: `assert_fn_ok_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is ok: `{:?}`,\n right output is ok: `{:?}`", stringify!($function), $a, $b, a_is_ok, b_is_ok);
        } else {
            let a_output = a_output.unwrap();
            let b_output = b_output.unwrap();
            if (a_output <= b_output) {
                ()
            } else {
                panic!("assertion failed: `assert_fn_ok_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_output, b_output);
            }
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_ok = a_output.is_ok();
        let b_is_ok = b_output.is_ok();
        if !a_is_ok || !b_is_ok {
            panic!("{:?}", $($arg)+)
        } else {
            let a_output = a_output.unwrap();
            let b_output = b_output.unwrap();
            if (a_output <= b_output) {
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
    fn test_assert_fn_ok_le_x_arity_2_lt_success() {
        let a = 1;
        let b = 2;
        let x = assert_fn_ok_le!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_fn_ok_le_x_arity_2_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_fn_ok_le!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ok_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `2`,\n  right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`")]
    fn test_assert_fn_ok_le_x_arity_2_gt_failure() {
        let a = 2;
        let b = 1;
        let _x = assert_fn_ok_le!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_fn_ok_le_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x = assert_fn_ok_le!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    fn test_assert_fn_ok_le_x_arity_3_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_fn_ok_le!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ok_le_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let _x = assert_fn_ok_le!(example_digit_to_string, a, b, "message");
    }

}
