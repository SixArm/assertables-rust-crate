/// Assert one value is less than another value.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// assert_io_lt!(1, 2);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// // assert_io_lt!(2, 1);
/// //-> panic!("assertion failed: `assert_io_lt(left, right)`\n  left: `2`,\n right: `1`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_io_lt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    ()
                } else {
                    panic!("assertion failed: `assert_io_lt(left, right)`\n  left: `{:?}`\n right: `{:?}`", $left, $right);
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val < right_val) {
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
    fn test_assert_io_lt_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assert_io_lt!(a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_io_lt(left, right)`\n  left: `2`\n right: `1`")]
    fn test_assert_io_lt_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        assert_io_lt!(a, b);
    }

    #[test]
    fn test_assert_io_lt_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assert_io_lt!(a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_io_lt_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        assert_io_lt!(a, b, "message");
    }

}
