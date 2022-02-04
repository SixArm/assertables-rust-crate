/// Assert a set is not equal to another.
///
/// * When true, return `Ok(())`.
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
/// assert_set_ne!([1, 2], [3, 4]);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_set_ne!([1, 2], [1, 2]);
/// //-> panic!
/// // assertion failed: `assert_set_ne!(left, right)`
/// //   left: `[1, 2]`,
/// //  right: `[1, 2]`
/// # });
/// # let actual: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_set_ne!(left, right)`\n  left: `[1, 2]`,\n right: `[1, 2]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assert_set_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
                    ()
                } else {
                    panic!("assertion failed: `assert_set_ne!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
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
    fn test_assert_set_ne_x_arity_2_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_ne!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_ne!(left, right)`\n  left: `[1, 2]`,\n right: `[1, 2]`")]
    fn test_assert_set_ne_x_arity_2_failure() {
        let a = [1, 2];
        let b = [1, 2];
        assert_set_ne!(&a, &b);
    }

    #[test]
    fn test_assert_set_ne_x_arity_3_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_ne!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_ne_x_arity_3_failure() {
        let a = [1, 2];
        let b = [1, 2];
        assert_set_ne!(&a, &b, "message");
    }

}
