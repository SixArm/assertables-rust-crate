/// Assert one function output is not equal to another function output.
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
/// assert_fn_ne!(i32::abs, 1 as i32, -2 as i32);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // assert_fn_ne!(i32::abs, 1 as i32, -1 as i32);
/// //-> panic!("assertion failed: `assert_fn_ne(fn, left, right)`\n  left input: `1`\n right input: `-1`\n  left output: `1`\n right output: `1`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left != right) {
            ()
        } else {
            panic!("assertion failed: `assert_fn_ne(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output: `{:?}`\n right output: `{:?}`", $left, $right, left, right);
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left != right) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_fn_ne_x_arity_2_success() {
        let a = 1;
        let b = -2;
        let x = assert_fn_ne!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ne(fn, left, right)`\n  left input: `1`\n right input: `-1`\n  left output: `1`\n right output: `1`")]
    fn test_assert_fn_ne_x_arity_2_failure() {
        let a = 1;
        let b = -1;
        let _ = assert_fn_ne!(i32::abs, a as i32, b as i32);
    }

    #[test]
    fn test_assert_fn_ne_x_arity_3_success() {
        let a = 1;
        let b = -2;
        let x = assert_fn_ne!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ne_x_arity_3_failure() {
        let a = 1;
        let b = -1;
        let _ = assert_fn_ne!(i32::abs, a as i32, b as i32, "message");
    }

}
