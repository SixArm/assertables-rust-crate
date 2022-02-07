/// Assert a function output is less than another.
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
/// assert_f_lt!(i32::abs, 1 as i32, -2 as i32);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_lt!(i32::abs, -2 as i32, 1 as i32);
/// //-> panic!("…")
/// // assertion failed: `assert_f_lt!(fn, left, right)`
/// //    left input: `-2`,
/// //   right input: `1`,
/// //   left output: `2`,
/// //  right output: `1`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_lt!(fn, left, right)`\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_lt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left < right) {
            ()
        } else {
            panic!("assertion failed: `assert_f_lt!(fn, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right);
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left < right) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_f_lt_x_arity_2_lt_success() {
        let a = 1;
        let b = -2;
        let x = assert_f_lt!(i32::abs, a as i32, b as i32);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_lt!(fn, left, right)`\n   left input: `1`,\n  right input: `-1`,\n  left output: `1`,\n right output: `1`")]
    fn test_assert_f_lt_x_arity_2_eq_failure() {
        let a = 1;
        let b = -1;
        let _x = assert_f_lt!(i32::abs, a as i32, b as i32);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_lt!(fn, left, right)`\n   left input: `2`,\n  right input: `-1`,\n  left output: `2`,\n right output: `1`")]
    fn test_assert_f_lt_x_arity_2_gt_failure() {
        let a = 2;
        let b = -1;
        let _x = assert_f_lt!(i32::abs, a as i32, b as i32);
    }

    #[test]
    fn test_assert_f_lt_x_arity_3_lt_success() {
        let a = 1;
        let b = -2;
        let x = assert_f_lt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_lt_x_arity_3_eq_failure() {
        let a = 1;
        let b = -1;
        let _x = assert_f_lt!(i32::abs, a as i32, b as i32, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_lt_x_arity_3_gt_failure() {
        let a = -2;
        let b = 1;
        let _x = assert_f_lt!(i32::abs, a as i32, b as i32, "message");
    }

}
