/// Assert a set is joint with another.
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
/// let a = [1, 2];
/// let b = [2, 3];
/// assert_set_joint!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_set_joint!(&a, &b);
/// //-> panic!
/// // assertion failed: `assert_set_joint!(left, right)`
/// //   left: `[1, 2]`,
/// //  right: `[3, 4]`
/// # });
/// # let actual: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_set_joint!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assert_set_joint {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if !left_set.is_disjoint(&right_set) {
                    ()
                } else {
                    panic!("assertion failed: `assert_set_joint!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if !left_set.is_disjoint(&right_set) {
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
    fn test_assert_set_joint_x_arity_2_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_joint!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_joint!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`")]
    fn test_assert_set_joint_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_joint!(&a, &b);
    }

    #[test]
    fn test_assert_set_joint_x_arity_3_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_joint!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_joint_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_joint!(&a, &b, "message");
    }

}
