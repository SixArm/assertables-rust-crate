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
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// # fn main() {
/// let x = assertable_f_err_string_eq!(f, 1, 1);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_f_err_string_eq!(f, 1, 2);
/// //-> Err("…")
/// // assertable failed: `assertable_f_err_string_eq!(function, left, right)`
/// //    left input: `1`,
/// //   right input: `2`,
/// //   left output: `\"1\"`,
/// //  right output: `\"2\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_f_err_string_eq!(function, left, right)`\n   left input: `1`,\n  right input: `2`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"1\"`,\n right output: `\"2\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_f_err_string_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        let left_is_err = left.is_err();
        let right_is_err = right.is_err();
        let left_string = if left_is_err { left.unwrap_err().to_string() } else { "".to_string() };
        let right_string = if right_is_err { right.unwrap_err().to_string() } else { "".to_string() };
        if left_is_err && right_is_err && left_string == right_string {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_f_err_string_eq!(function, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left_is_err, right_is_err, left_string, right_string))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        let left_is_err = left.is_err();
        let right_is_err = right.is_err();
        let left_string = if left_is_err { left.unwrap_err().to_string() } else { "".to_string() };
        let right_string = if right_is_err { right.unwrap_err().to_string() } else { "".to_string() };
        if left_is_err && right_is_err && left_string == right_string {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }

    #[test]
    fn test_assertable_f_err_string_eq_x_arity_2_success() {
        let a = 1;
        let b = 1;
        let x = assertable_f_err_string_eq!(f, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_err_string_eq_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assertable_f_err_string_eq!(f, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_f_err_string_eq!(function, left, right)`\n   left input: `1`,\n  right input: `2`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"1\"`,\n right output: `\"2\"`"
        );
    }

    #[test]
    fn test_assertable_f_err_string_eq_x_arity_3_success() {
        let a = 1;
        let b = 1;
        let x = assertable_f_err_string_eq!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_err_string_eq_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assertable_f_err_string_eq!(f, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
