/// Assert a value is greater than another.
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
/// assert_io_gt!(2, 1);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_io_gt!(1, 2);
/// //-> panic!("…")
/// // assertion failed: `assert_io_gt!(left, right)`
/// //   left: `1`,
/// //  right: `2`
/// # });
/// # let actual: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_io_gt!(left, right)`\n  left: `1`,\n right: `2`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_io_gt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val > right_val) {
                    ()
                } else {
                    panic!("assertion failed: `assert_io_gt!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right);
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val > right_val) {
                    ()
                } else {
                    panic!("{:?}", $($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_io_gt_x_arity_2_success() {
        let a = 2;
        let b = 1;
        let x = assert_io_gt!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_io_gt!(left, right)`\n  left: `1`,\n right: `2`")]
    fn test_assert_io_gt_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        assert_io_gt!(a, b);
    }

    #[test]
    fn test_assert_io_gt_x_arity_3_success() {
        let a = 2;
        let b = 1;
        let x = assert_io_gt!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_io_gt_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        assert_io_gt!(a, b, "message");
    }

}
