/// Assert a bag is a superbag of another.
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
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// assert_bag_superbag!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_superbag!(&a, &b);
/// //-> panic!("…")
/// // assertion failed: `assert_bag_superbag!(left, right)`
/// //   left: `{1: 2}`,
/// //  right: `{2: 2}`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_bag_superbag!(left, right)`\n",
/// #     "  left: `{1: 2}`,\n",
/// #     " right: `{2: 2}`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`BTreeMap`] to count items and sort them.
#[macro_export]
macro_rules! assert_bag_superbag {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                let mut b_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if b_val.into_iter().all(|key| {
                    a_bag.contains_key(&key) && b_bag.contains_key(&key) &&
                    a_bag.get_key_value(&key) >= b_bag.get_key_value(&key)
                }) {
                    ()
                } else {
                    panic!("{}", msg_key_left_right!("assertion failed", "assert_bag_superbag!", &a_bag, &b_bag))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                let mut b_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if b_bag.into_iter().all(|(&b_key, b_val)| {
                    match a_bag.get(&b_key) {
                        Some(&a_val) => {
                            a_val >= b_val
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
    fn test_assert_bag_superbag_x_arity_2_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x= assert_bag_superbag!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_superbag!(left, right)`\n  left: `{1: 2}`,\n right: `{2: 2}`")]
    fn test_assert_bag_superbag_x_arity_2_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let _x = assert_bag_superbag!(&a, &b);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_superbag!(left, right)`\n  left: `{1: 2}`,\n right: `{1: 3}`")]
    fn test_assert_bag_superbag_x_arity_2_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let _x = assert_bag_superbag!(&a, &b);
    }

    #[test]
    fn test_assert_bag_superbag_x_arity_3_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assert_bag_superbag!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_superbag_x_arity_3_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let _x = assert_bag_superbag!(&a, &b, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_superbag_x_arity_3_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let _x = assert_bag_superbag!(&a, &b, "message");
    }

}
