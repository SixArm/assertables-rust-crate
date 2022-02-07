/// Assert a value is less than another.
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
/// assert_lt!(1, 2);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_lt!(2, 1);
/// //-> panic!
/// // assertion failed: `assert_lt!(left, right)`
/// //   left: `2`,
/// //  right: `1`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    ()
                } else {
                    panic!("assertion failed: `assert_lt!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right);
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
    fn test_assert_lt_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assert_lt!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`")]
    fn test_assert_lt_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        let _x = assert_lt!(a, b);
    }

    #[test]
    fn test_assert_lt_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assert_lt!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_lt_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let _x = assert_lt!(a, b, "message");
    }

}
