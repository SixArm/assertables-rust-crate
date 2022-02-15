/// Assert one function ok() is equal to another function ok().
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
/// let x = assertable_f_err_eq!(example_digit_to_string, 10, 10);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_f_err_eq!(example_digit_to_string, 10, 20);
/// //-> Err("…")
/// // assertable failed: `assertable_f_err_eq!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `10`,
/// //   right input: `20`,
/// //   left output: `\"10 is out of range\"`,
/// //  right output: `\"20 is out of range\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_f_err_eq!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_f_err_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        let left_is_err = left_output.is_err();
        let right_is_err = right_output.is_err();
        if !left_is_err || !right_is_err {
            Err(format!("assertable failed: `assertable_f_err_eq!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`", stringify!($function), $left, $right, left_is_err, right_is_err))
        } else {
            let left_err = left_output.unwrap_err();
            let right_err = right_output.unwrap_err();
            if left_err == right_err {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_f_err_eq!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left_is_err, right_is_err, left_err, right_err))
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        let left_is_err = left_output.is_err();
        let right_is_err = right_output.is_err();
        if !left_is_err || !right_is_err {
            Err($($arg)+)
        } else {
            let left_err = left_output.unwrap_err();
            let right_err = right_output.unwrap_err();
            if left_err == right_err {
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
    fn test_assertable_f_err_eq_x_arity_2_success() {
        let a = 10;
        let b = 10;
        let x = assertable_f_err_eq!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_err_eq_x_arity_2_failure() {
        let a = 10;
        let b = 20;
        let x = assertable_f_err_eq!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_f_err_eq!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`"
        );
    }

    #[test]
    fn test_assertable_f_err_eq_x_arity_3_success() {
        let a = 10;
        let b = 10;
        let x = assertable_f_err_eq!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_err_eq_x_arity_3_failure() {
        let a = 10;
        let b = 20;
        let x = assertable_f_err_eq!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
