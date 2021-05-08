/// Assert one function ok() is less than or equal to another function ok().
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// assert_fn_err_string_le!(f, 1, 2);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// // assert_fn_err_string_le!(f, 2, 1);
/// //-> panic!("assertion failed: `assert_fn_err_string_le(fn, left, right)`\n  left input: `2`\n right input: `1`\n  left output: `\"2\"`\n right output: `\"1\"`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_err_string_le {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            panic!("assertion failed: `assert_fn_err_string_le(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output is_err(): `{:?}`\n right output is_err(): `{:?}`", $left, $right, left.is_err(), right.is_err());
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left <= right) {
                ()
            } else {
                panic!("assertion failed: `assert_fn_err_string_le(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output: `{:?}`\n right output: `{:?}`", $left, $right, left, right);
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left <= right) {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }

    #[test]
    fn test_assert_fn_err_string_le_x_arity_2_lt_success() {
        let a = 1;
        let b = 2;
        let x = assert_fn_err_string_le!(f, a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    fn test_assert_fn_err_string_le_x_arity_2_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_fn_err_string_le!(f, a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_err_string_le(fn, left, right)`\n  left input: `2`\n right input: `1`\n  left output: `\"2\"`\n right output: `\"1\"`")]
    fn test_assert_fn_err_string_le_x_arity_2_gt_failure() {
        let a = 2;
        let b = 1;
        let _ = assert_fn_err_string_le!(f, a, b);
    }

    #[test]
    fn test_assert_fn_err_string_le_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x = assert_fn_err_string_le!(f, a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    fn test_assert_fn_err_string_le_x_arity_3_eq_success() {
        let a = 1;
        let b = 1;
        let x = assert_fn_err_string_le!(f, a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_err_string_le_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let _ = assert_fn_err_string_le!(f, a, b, "message");
    }

}
