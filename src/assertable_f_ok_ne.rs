/// Assert one function ok() is not equal to another function ok().
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
/// let x = assertable_f_ok_ne!(example_digit_to_string, 1, 2);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_f_ok_ne!(example_digit_to_string, 1, 1);
/// //-> Err("…")
/// // assertable failed: `assertable_f_ok_ne!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `1`,
/// //   right input: `1`,
/// //   left output: `\"1\"`,
/// //  right output: `\"1\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_f_ok_ne!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_f_ok_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Err(format!("assertable failed: `assertable_f_ok_ne!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is ok: `{:?}`,\n right output is ok: `{:?}`", stringify!($function), $left, $right, left.is_ok(), right.is_ok()))
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left != right) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_f_ok_ne!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left, right))
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Err($($arg)+)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left != right) {
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
    fn test_assertable_f_ok_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assertable_f_ok_ne!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        let x = assertable_f_ok_ne!(example_digit_to_string, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_f_ok_ne!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`"
        );
    }

    #[test]
    fn test_assertable_f_ok_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assertable_f_ok_ne!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ok_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        let x = assertable_f_ok_ne!(example_digit_to_string, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
