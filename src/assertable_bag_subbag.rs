/// Assert a bag is a subbag of another.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
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
/// let x = assertable_bag_subbag!(&a, &b);
/// //-> Ok(())
/// # assert_eq!(x.unwrap(), ());
///
/// let a = [1, 1];
/// let b = [2, 2];
/// let x = assertable_bag_subbag!(&a, &b);
/// //-> Err("…")
/// // assertion failed: `assertable_bag_subbag!(left, right)`
/// //   left: `{1: 2}`,
/// //  right: `{2: 2}`
/// # let expect = concat!(
/// #     "assertion failed: `assertable_bag_subbag!(left, right)`\n",
/// #     "  left: `{1: 2}`,\n",
/// #     " right: `{2: 2}`"
/// # );
/// # assert_eq!(x.unwrap_err(), expect);
///
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// let x = assertable_bag_subbag!(&a, &b);
/// //-> Err("…")
/// // assertion failed: `assertable_bag_subbag!(left, right)`
/// //   left: `{1: 3}`,
/// //  right: `{1: 2}`
/// # let expect = concat!(
/// #     "assertion failed: `assertable_bag_subbag!(left, right)`\n",
/// #     "  left: `{1: 3}`,\n",
/// #     " right: `{1: 2}`"
/// # );
/// # assert_eq!(x.unwrap_err(), expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`BTreeMap`] to count items and sort them.
#[macro_export]
macro_rules! assertable_bag_subbag {
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
                if a_val.into_iter().all(|key| {
                    a_bag.contains_key(&key) && b_bag.contains_key(&key) &&
                    a_bag.get_key_value(&key) <= b_bag.get_key_value(&key)
                }) {
                    Ok(())
                } else {
                    Err(msg_key_left_right!("assertion failed", "assertable_bag_subbag!", &a_bag, &b_bag))
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
                if a_bag.into_iter().all(|(&a_key, a_val)| {
                    match b_bag.get(&a_key) {
                        Some(&b_value) => {
                            a_val <= b_value
                        },
                        None => false,
                    }
                }) {
                    Ok(())
                } else {
                    Err($($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_bag_subbag_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_subbag!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_subbag_x_arity_2_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assertable_bag_subbag!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertion failed: `assertable_bag_subbag!(left, right)`\n  left: `{1: 2}`,\n right: `{2: 2}`"
        );
    }

    #[test]
    fn test_assertable_bag_subbag_x_arity_2_failure_because_val_count_is_excessive() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assertable_bag_subbag!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertion failed: `assertable_bag_subbag!(left, right)`\n  left: `{1: 3}`,\n right: `{1: 2}`"
        );
    }

    #[test]
    fn test_assertable_bag_subbag_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_subbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_subbag_x_arity_3_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assertable_bag_subbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assertable_bag_subbag_x_arity_3_failure_because_val_count_is_excessive() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assertable_bag_subbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
