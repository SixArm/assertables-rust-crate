/// Assert one function output is equal to another function output.
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
/// let x = assertable_fn_eq!(i32::abs, 1 as i32, -1 as i32);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assertable_fn_eq!(i32::abs, 1 as i32, -2 as i32);
/// //-> Err
/// // assertable failed: `assertable_fn_eq!(function, left, right)`
/// //      function: `\"i32::abs\"`,
/// //    left input: `1`,
/// //   right input: `-2`,
/// //   left output: `1`,
/// //  right output: `2`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_eq!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `1`,\n  right input: `-2`,\n  left output: `1`,\n right output: `2`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if (left_output == right_output) {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_fn_eq!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left_output, right_output))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left_output = $function($left);
        let right_output = $function($right);
        if (left_output == right_output) {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_fn_eq_x_arity_2_success() {
        let a = 1;
        let b = -1;
        let x = assertable_fn_eq!(i32::abs, a as i32, b as i32);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_fn_eq_x_arity_2_failure() {
        let a = 1;
        let b = -2;
        let x = assertable_fn_eq!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_eq!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `1`,\n  right input: `-2`,\n  left output: `1`,\n right output: `2`"
        );
    }

    #[test]
    fn test_assertable_fn_eq_x_arity_3_success() {
        let a = 1;
        let b = -1;
        let x = assertable_fn_eq!(i32::abs, a as i32, b as i32, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_fn_eq_x_arity_3_failure() {
        let a = 1;
        let b = -2;
        let x = assertable_fn_eq!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
