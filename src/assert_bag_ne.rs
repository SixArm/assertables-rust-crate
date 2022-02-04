/// Assert a bag is not equal to another.
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
/// assert_bag_ne!([1, 1], [1, 1, 1]);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_bag_ne!([1, 1], [1, 1]);
/// //-> panic!("…")
/// // assertion failed: `assert_bag_ne!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[1, 1]`
/// # });
/// # let actual: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_bag_ne!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assert_bag_ne {
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
                if left_bag != right_bag {
                    ()
                } else {
                    panic!("assertion failed: `assert_bag_ne!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
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
                if left_bag != right_bag {
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
    fn test_assert_bag_ne_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_ne!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_ne!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1]`")]
    fn test_assert_bag_ne_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let _x = assert_bag_ne!(&a, &b);
    }

    #[test]
    fn test_assert_bag_ne_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_ne!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_ne_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let _x = assert_bag_ne!(&a, &b, "message");
    }

}
