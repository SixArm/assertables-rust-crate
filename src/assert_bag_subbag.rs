/// Assert a bag is equal to another.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] in order to print the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_subbag!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_subbag!(&a, &b);
/// //-> panic!("…")
/// // assertion failed: `assert_bag_subbag!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[2, 2]`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_bag_subbag!(left, right)`\n  left: `[1, 1]`,\n right: `[2, 2]`";
/// # assert_eq!(actual, expect);
/// 
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// assert_bag_subbag!(&a, &b);
/// //-> panic!("…")
/// // assertion failed: `assert_bag_subbag!(left, right)`
/// //   left: `[1, 1, 1]`,
/// //  right: `[1, 1]`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_bag_subbag!(left, right)`\n  left: `[1, 1, 1]`,\n right: `[1, 1]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assert_bag_subbag {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let mut left_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut right_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in left_val.into_iter() {
                    let n = left_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in right_val.into_iter() {
                    let n = right_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if left_bag.into_iter().all(|(&left_key, left_val)| {
                    match right_bag.get(&left_key) {
                        Some(&right_val) => {
                            left_val <= right_val
                        },
                        None => false,
                    }
                }) {
                    ()
                } else {
                    panic!("assertion failed: `assert_bag_subbag!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let mut left_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut right_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in left_val.into_iter() {
                    let n = left_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in right_val.into_iter() {
                    let n = right_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if left_bag.into_iter().all(|(&left_key, left_val)| {
                    match right_bag.get(&left_key) {
                        Some(&right_val) => {
                            left_val <= right_val
                        },
                        None => false,
                    }
                }) {
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
    fn test_assert_bag_subbag_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x= assert_bag_subbag!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_subbag!(left, right)`\n  left: `[1, 1]`,\n right: `[2, 2]`")]
    fn test_assert_bag_subbag_x_arity_2_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let _x = assert_bag_subbag!(&a, &b);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_subbag!(left, right)`\n  left: `[1, 1, 1]`,\n right: `[1, 1]`")]
    fn test_assert_bag_subbag_x_arity_2_failure_because_val_count_is_excessive() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let _x = assert_bag_subbag!(&a, &b);
    }

    #[test]
    fn test_assert_bag_subbag_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_subbag!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_subbag_x_arity_3_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let _x = assert_bag_subbag!(&a, &b, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_subbag_x_arity_3_failure_because_val_count_is_excessive() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let _x = assert_bag_subbag!(&a, &b, "message");
    }

}
