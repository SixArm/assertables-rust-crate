/// Assert one function ok() is greater than anoter.
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
/// let x = assertable_fn_err_gt!(example_digit_to_string, 20, 10);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_fn_err_gt!(example_digit_to_string, 10, 20);
/// //-> Err("…")
/// // assertable failed: `assertable_fn_err_gt!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `10`,
/// //   right input: `20`,
/// //   left output: `\"10 is out of range\"`,
/// //  right output: `\"20 is out of range\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_err_gt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_err_gt {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        if !a_is_err || !b_is_err {
            Err(format!("assertable failed: `assertable_fn_err_gt!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`", stringify!($function), $a, $b, a_is_err, b_is_err))
        } else {
            let a_err = a_output.unwrap_err();
            let b_err = b_output.unwrap_err();
            if a_err > b_err {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_fn_err_gt!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_is_err, b_is_err, a_err, b_err))
            }
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        if !a_is_err || !b_is_err {
            Err($($arg)+)
        } else {
            let a_err = a_output.unwrap_err();
            let b_err = b_output.unwrap_err();
            if a_err > b_err {
                Ok(())
            } else {
                Err($($arg)+)
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
    fn test_assertable_fn_err_gt_x_arity_2_gt_success() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_gt_x_arity_2_eq_failure() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_gt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"10 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_gt_x_arity_2_lt_failure() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_gt!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_gt_x_arity_3_gt_success() {
        let a = 20;
        let b = 10;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_gt_x_arity_3_eq_failure() {
        let a = 10;
        let b = 10;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assertable_fn_err_gt_x_arity_3_lt_failure() {
        let a = 10;
        let b = 20;
        let x = assertable_fn_err_gt!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}