/// Assert one function ok() is less than anoter.
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
/// fn example_digit_to_string(i: isize) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// let x = assertable_fn_err_string_lt!(example_digit_to_string, 10, 20);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_fn_err_string_lt!(example_digit_to_string, 20, 10);
/// //-> Err("…")
/// // assertable failed: `assertable_fn_err_string_lt!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `20`,
/// //   right input: `10`,
/// //   left output: `\"20 is out of range\"`,
/// //  right output: `\"10 is out of range\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_err_string_lt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `20`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"20 is out of range\"`,\n right output: `\"10 is out of range\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_err_string_lt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        let left_is_err = left_output.is_err();
        let right_is_err = right_output.is_err();
        let left_string = String::from(left_output.unwrap_err());
        let right_string = String::from(right_output.unwrap_err());
        if left_is_err && right_is_err && left_string < right_string {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_fn_err_string_lt!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left_is_err, right_is_err, left_string, right_string))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        let left_is_err = left_output.is_err();
        let right_is_err = right_output.is_err();
        let left_string = String::from(left_output.unwrap_err());
        let right_string = String::from(right_output.unwrap_err());
        if left_is_err && right_is_err && left_string < right_string {
            Ok(())
        } else {
            Err($($arg)+)
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
    fn test_assertable_fn_err_string_lt_x_arity_2_lt_success() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_lt_x_arity_2_eq_failure() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_string_lt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"10 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_string_lt_x_arity_2_gt_failure() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_string_lt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `20`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"20 is out of range\"`,\n right output: `\"10 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_string_lt_x_arity_3_lt_success() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_lt_x_arity_3_eq_failure() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assertable_fn_err_string_lt_x_arity_3_gt_failure() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_string_lt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
