/// Assert a bag is equal to another.
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
/// let b = [1, 1];
/// let x = assertable_bag_eq!(&a, &b);
/// //-> Ok(())
/// # assert_eq!(x.unwrap(), ());
///
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// let x = assertable_bag_eq!(&a, &b);
/// //-> Err("…")
/// // assertion failed: `assertable_bag_eq!(left, right)`
/// //   left: `{1: 2}`,
/// //  right: `{1: 3}`
/// # let expect = concat!(
/// #     "assertion failed: `assertable_bag_eq!(left, right)`\n",
/// #     "  left: `{1: 2}`,\n",
/// #     " right: `{1: 3}`"
/// # );
/// # assert_eq!(x.unwrap_err(), expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`BTreeMap`] to count items and sort them.
#[macro_export]
macro_rules! assertable_bag_eq {
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
                if a_bag == b_bag {
                    Ok(())
                } else {
                    Err(msg_key_left_right!("assertion failed", "assertable_bag_eq!", &a_bag, &b_bag))
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
                if a_bag == b_bag {
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
    fn test_assertable_bag_eq_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assertable_bag_eq!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_eq_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_eq!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertion failed: `assertable_bag_eq!(left, right)`\n  left: `{1: 2}`,\n right: `{1: 3}`"
        );
    }

    #[test]
    fn test_assertable_bag_eq_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assertable_bag_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_eq_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
