/// Assert two values are equal.
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
/// assert_io_ne!(1, 2);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// // assert_io_ne!(1, 1);
/// //-> panic!("assertion failed: `assert_io_ne(left, right)`\n  left: `1`,\n right: `2`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_io_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val != right_val) {
                    ()
                } else {
                    panic!("assertion failed: `assert_io_ne(left, right)`\n  left: `{:?}`\n right: `{:?}`", $left, $right);
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val != right_val) {
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
    fn test_assert_io_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assert_io_ne!(a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_io_ne(left, right)`\n  left: `1`\n right: `1`")]
    fn test_assert_io_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        assert_io_ne!(a, b);
    }

    #[test]
    fn test_assert_io_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assert_io_ne!(a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_io_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        assert_io_ne!(a, b, "message");
    }

}
