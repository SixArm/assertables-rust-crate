/// Assert a set is disjoint with another.
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
/// let b = [3, 4];
/// assert_set_disjoint!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [2, 3];
/// assert_set_disjoint!(&a, &b);
/// //-> panic!
/// // assertion failed: `assert_set_disjoint!(left, right)`
/// //   left: `[1, 2]`,
/// //  right: `[2, 3]`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_set_disjoint!(left, right)`\n  left: `[1, 2]`,\n right: `[2, 3]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assert_set_disjoint {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set.is_disjoint(&right_set) {
                    ()
                } else {
                    panic!("assertion failed: `assert_set_disjoint!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set.is_disjoint(&right_set) {
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
    fn test_assert_set_disjoint_x_arity_2_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_disjoint!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_disjoint!(left, right)`\n  left: `[1, 2]`,\n right: `[2, 3]`")]
    fn test_assert_set_disjoint_x_arity_2_failure() {
        let a = [1, 2];
        let b = [2, 3];
        assert_set_disjoint!(&a, &b);
    }

    #[test]
    fn test_assert_set_disjoint_x_arity_3_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_disjoint!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_disjoint_x_arity_3_failure() {
        let a = [1, 2];
        let b = [2, 3];
        assert_set_disjoint!(&a, &b, "message");
    }

}
