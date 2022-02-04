/// Assert a set is equal to another.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// assert_set_eq!([1, 2], [2, 1]);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// # let result = panic::catch_unwind(|| {
/// assert_set_eq!([1, 2], [3, 4]);
/// # });
/// # let err: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # assert_eq!(err, "assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`");
/// //-> panic!("assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assert_set_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
                    ()
                } else {
                    panic!("assertion failed: `assert_set_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
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
        let b = [1, 2];
        let x = assert_set_eq!(&a, &b);
        assert_eq!(
            x, 
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`")]
    fn test_assert_set_eq_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_eq!(&a, &b);
    }

    #[test]
    fn test_assert_set_eq_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assert_set_eq!(&a, &b, "message");
        assert_eq!(
            x,
            ()
        )
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_eq_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_eq!(&a, &b, "message");
    }

}
