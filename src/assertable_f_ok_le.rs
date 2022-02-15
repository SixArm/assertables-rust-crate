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
/// let x = assertable_f_ok_le!(example_digit_to_string, 1, 2);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_f_ok_le!(example_digit_to_string, 2, 1);
/// //-> Err("…")
/// // assertable failed: `assertable_f_ok_le!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `2`,
/// //   right input: `1`,
/// //   left output: `\"2\"`,
/// //  right output: `\"1\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_f_ok_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `2`,\n  right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_f_ok_le {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if !left_output.is_ok() || !right_output.is_ok() {
            Err(format!("assertable failed: `assertable_f_ok_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is ok: `{:?}`,\n right output is ok: `{:?}`", stringify!($function), $left, $right, left_output.is_ok(), right_output.is_ok()))
        } else {
            let left_output = left_output.unwrap();
            let right_output = right_output.unwrap();
            if (left_output <= right_output) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_f_ok_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left_output, right_output))
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if !left_output.is_ok() || !right_output.is_ok() {
            Err($($arg)+)
        } else {
            let left_output = left_output.unwrap();
            let right_output = right_output.unwrap();
            if (left_output <= right_output) {
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
    fn test_assertable_f_ok_le_x_arity_2_lt_success() {
        let a = 1;
        let b = 2;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_le_x_arity_2_eq_success() {
        let a = 1;
        let b = 1;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_le_x_arity_2_gt_failure() {
        let a = 2;
        let b = 1;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_f_ok_le!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `2`,\n  right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`"
        );
    }

    #[test]
    fn test_assertable_f_ok_le_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_le_x_arity_3_eq_success() {
        let a = 1;
        let b = 1;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_le_x_arity_3_gt_failure() {
        let a = 2;
        let b = 1;
        let x = assertable_f_ok_le!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
