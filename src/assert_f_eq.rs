/// Assert a function output is equal to another.
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
/// # fn main() {
/// assert_f_eq!(i32::abs, 1 as i32, -1 as i32);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_eq!(i32::abs, 1 as i32, -2 as i32);
/// //-> panic!("…")
/// // assertion failed: `assert_f_eq!(function, left, right)`
/// //    left input: `1`,
/// //   right input: `-2`,
/// //   left output: `1`,
/// //  right output: `2`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_eq!(function, left, right)`\n   left input: `1`,\n  right input: `-2`,\n  left output: `1`,\n right output: `2`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left == right) {
            ()
        } else {
            panic!("assertion failed: `assert_f_eq!(function, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right);
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left == right) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_f_eq_x_arity_2_eq_success() {
        let a = 1;
        let b = -1;
        let x = assert_f_eq!(i32::abs, a as i32, b as i32);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_eq!(function, left, right)`\n   left input: `1`,\n  right input: `-2`,\n  left output: `1`,\n right output: `2`")]
    fn test_assert_f_eq_x_arity_2_ne_failure() {
        let a = 1;
        let b = -2;
        let _x = assert_f_eq!(i32::abs, a as i32, b as i32);
    }

    #[test]
    fn test_assert_f_eq_x_arity_3_eq_success() {
        let a = 1;
        let b = -1;
        let x = assert_f_eq!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_eq_x_arity_3_ne_failure() {
        let a = 1;
        let b = -2;
        let _x = assert_f_eq!(i32::abs, a as i32, b as i32, "message");
    }

}
