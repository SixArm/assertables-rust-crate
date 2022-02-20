/// Assert one function ok() is less than or equal to another function ok().
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
/// let x = assertable_fn_err_string_le!(example_digit_to_string, 10, 20);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_fn_err_string_le!(example_digit_to_string, 20, 10);
/// //-> Err("…")
/// // assertable failed: `assertable_fn_err_string_le!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `20`,
/// //   right input: `10`,
/// //   left output: `\"20 is out of range\"`,
/// //  right output: `\"10 is out of range\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_err_string_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `20`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"20 is out of range\"`,\n right output: `\"10 is out of range\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_err_string_le {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        let a_string = String::from(a_output.unwrap_err());
        let b_string = String::from(b_output.unwrap_err());
        if a_is_err && b_is_err && a_string <= b_string {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_fn_err_string_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_is_err, b_is_err, a_string, b_string))
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        let a_string = String::from(a_output.unwrap_err());
        let b_string = String::from(b_output.unwrap_err());
        if a_is_err && b_is_err && a_string <= b_string {
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
    fn test_assertable_fn_err_string_le_x_arity_2_lt_success() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_le_x_arity_2_eq_success() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_le_x_arity_2_gt_failure() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_string_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `20`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"20 is out of range\"`,\n right output: `\"10 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_string_le_x_arity_3_lt_success() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_le_x_arity_3_eq_success() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_le_x_arity_3_gt_failure() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_string_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
