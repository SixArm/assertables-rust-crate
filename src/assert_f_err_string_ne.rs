/// Assert a function err() is not equal to another.
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
/// fn digit_to_string(i: isize) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// assert_f_err_string_ne!(digit_to_string, 10, 20);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_err_string_ne!(digit_to_string, 10, 10);
/// //-> panic!("…")
/// // assertion failed: `assert_f_err_string_ne!(function, left, right)`
/// //    left input: `10`,
/// //   right input: `10`,
/// //   left output: `\"10 is out of range\"`,
/// //  right output: `\"10 is out of range\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_err_string_ne!(function, left, right)`\n   left input: `10`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"10 is out of range\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_err_string_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        let left_is_err = left.is_err();
        let right_is_err = right.is_err();
        let left_string = if left_is_err { left.unwrap_err().to_string() } else { "".to_string() };
        let right_string = if right_is_err { right.unwrap_err().to_string() } else { "".to_string() };
        if left_is_err && right_is_err && left_string != right_string {
            ()
        } else {
            panic!("assertion failed: `assert_f_err_string_ne!(function, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right,  left_is_err, right_is_err, left_string, right_string);
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        let left_is_err = left.is_err();
        let right_is_err = right.is_err();
        let left_string = if left_is_err { left.unwrap_err().to_string() } else { "".to_string() };
        let right_string = if right_is_err { right.unwrap_err().to_string() } else { "".to_string() };
        if left_is_err && right_is_err && left_string != right_string {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    // Replicate this function relevant tests in this crate.
    fn digit_to_string(i: isize) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_f_err_string_ne_x_arity_2_success() {
        let a = 10;
        let b = 20;
        let x = assert_f_err_string_ne!(digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_err_string_ne!(function, left, right)`\n   left input: `10`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"10 is out of range\"`")]
    fn test_assert_f_err_string_ne_x_arity_2_failure() {
        let a = 10;
        let b = 10;
        let _x = assert_f_err_string_ne!(digit_to_string, a, b);
    }

    #[test]
    fn test_assert_f_err_string_ne_x_arity_3_success() {
        let a = 10;
        let b = 20;
        let x = assert_f_err_string_ne!(digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_err_string_ne_x_arity_3_failure() {
        let a = 10;
        let b = 10;
        let _x = assert_f_err_string_ne!(digit_to_string, a, b, "message");
    }

}
