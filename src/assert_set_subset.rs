/// Assert a set is a subset of another.
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
/// let a = [1, 2];
/// let b = [1, 2, 3];
/// assert_set_subset!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 2, 3];
/// let b = [1, 2];
/// assert_set_subset!(&a, &b);
/// //-> panic!
/// // assertion failed: `assert_set_subset!(left, right)`
/// //   left: `[1, 2, 3]`,
/// //  right: `[1, 2]`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_set_subset!(left, right)`\n  left: `[1, 2, 3]`,\n right: `[1, 2]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assert_set_subset {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::HashSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::HashSet<_> = b_val.into_iter().collect();
                if a_set.is_subset(&b_set) {
                    ()
                } else {
                    panic!("assertion failed: `assert_set_subset!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b)
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                let a_set: ::std::collections::HashSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::HashSet<_> = b_val.into_iter().collect();
                if a_set.is_subset(&b_set) {
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
    fn test_assert_set_subset_x_arity_2_success() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let x = assert_set_subset!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_subset!(left, right)`\n  left: `[1, 2, 3]`,\n right: `[1, 2]`")]
    fn test_assert_set_subset_x_arity_2_failure() {
        let a = [1, 2, 3];
        let b = [1, 2];
        assert_set_subset!(&a, &b);
    }

    #[test]
    fn test_assert_set_subset_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let x = assert_set_subset!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_subset_x_arity_3_failure() {
        let a = [1, 2, 3];
        let b = [1, 2];
        assert_set_subset!(&a, &b, "message");
    }

}
