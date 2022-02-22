/// Assert a set is equal to another.
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
/// let b = [2, 1];
/// assert_set_eq!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_set_eq!(&a, &b);
/// //-> panic!
/// // assertion failed: `assert_set_eq!(left, right)`
/// //   left: `{1, 2}`,
/// //  right: `{3, 4}`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_set_eq!(left, right)`\n",
/// #     "  left: `{1, 2}`,\n",
/// #     " right: `{3, 4}`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`BTreeSet`] to count items and sort them.
#[macro_export]
macro_rules! assert_set_eq {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set == b_set {
                    ()
                } else {
                    panic!("{}", msg_key_left_right!("assertion failed", "assert_set_eq!", a_set, b_set))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set == b_set {
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
    fn test_assert_set_eq_x_arity_2_success() {
        let a = [1, 2];
        let b = [2, 1];
        let x = assert_set_eq!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_eq!(left, right)`\n  left: `{1, 2}`,\n right: `{3, 4}`")]
    fn test_assert_set_eq_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_eq!(&a, &b);
    }

    #[test]
    fn test_assert_set_eq_x_arity_3_success() {
        let a = [1, 2];
        let b = [2, 1];
        let x = assert_set_eq!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_eq_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_eq!(&a, &b, "message");
    }

}
