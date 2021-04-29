/// Assert one function ok() is less than another function ok().
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
/// # use std::str::FromStr;
/// assert_fn_ok_lt!(i32::from_str, "1", "2");
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // use std::str::FromStr;
/// // assert_fn_ok_lt!(i32::from_str, "2", "1");
/// //-> panic!("assertion failed: `assert_fn_ok_lt(fn, left, right)`\n  left input: `\"2\"`\n right input: `\"1\"`\n  left output: `2`\n right output: `1`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_ok_lt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            panic!("assertion failed: `assert_fn_ok_lt(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output is_ok(): `{:?}`\n right output is_ok(): `{:?}`", $left, $right, left.is_ok(), right.is_ok());
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left < right) {
                ()
            } else {
                panic!("assertion failed: `assert_fn_ok_lt(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output: `{:?}`\n right output: `{:?}`", $left, $right, left, right);
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            panic!("{:?}", $($arg)+)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left < right) {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_assert_fn_ok_lt_x_arity_2_lt_success() {
        let a = "1";
        let b = "2";
        let x = assert_fn_ok_lt!(i32::from_str, a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ok_lt(fn, left, right)`\n  left input: `\"1\"`\n right input: `\"1\"`\n  left output: `1`\n right output: `1`")]
    fn test_assert_fn_ok_lt_x_arity_2_eq_failure() {
        let a = "1";
        let b = "1";
        let _ = assert_fn_ok_lt!(i32::from_str, a, b);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ok_lt(fn, left, right)`\n  left input: `\"2\"`\n right input: `\"1\"`\n  left output: `2`\n right output: `1`")]
    fn test_assert_fn_ok_lt_x_arity_2_gt_failure() {
        let a = "2";
        let b = "1";
        let _ = assert_fn_ok_lt!(i32::from_str, a, b);
    }

    #[test]
    fn test_assert_fn_ok_lt_x_arity_3_lt_success() {
        let a = "1";
        let b = "2";
        let x = assert_fn_ok_lt!(i32::from_str, a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ok_lt_x_arity_3_eq_failure() {
        let a = "1";
        let b = "1";
        let _ = assert_fn_ok_lt!(i32::from_str, a, b, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ok_lt_x_arity_3_gt_failure() {
        let a = "2";
        let b = "1";
        let _ = assert_fn_ok_lt!(i32::from_str, a, b, "message");
    }

}
