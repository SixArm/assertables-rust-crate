/// Assert a bag is a superbag of another.
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
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// let x = assertable_bag_superbag!(&a, &b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let a = [1, 1];
/// let b = [2, 2];
/// let x = assertable_bag_superbag!(&a, &b);
/// //-> Err("…")
/// // assertable failed: `assertable_bag_superbag!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[2, 2]`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_bag_superbag!(left, right)`\n  left: `[1, 1]`,\n right: `[2, 2]`".to_string());
///
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// let x = assertable_bag_superbag!(&a, &b);
/// //-> Err("…")
/// // assertable failed: `assertable_bag_superbag!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[1, 1, 1]`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_bag_superbag!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1, 1]`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assertable_bag_superbag {
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
                if right_bag.into_iter().all(|(&right_key, right_val)| {
                    match left_bag.get(&right_key) {
                        Some(&left_val) => {
                            left_val >= right_val
                        },
                        None => false,
                    }
                }) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_bag_superbag!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right))
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
                if right_bag.into_iter().all(|(&right_key, right_val)| {
                    match left_bag.get(&right_key) {
                        Some(&left_val) => {
                            left_val >= right_val
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
    fn test_assertable_bag_superbag_x_arity_2_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assertable_bag_superbag!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_superbag_x_arity_2_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assertable_bag_superbag!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_bag_superbag!(left, right)`\n  left: `[1, 1]`,\n right: `[2, 2]`"
        );
    }

    #[test]
    fn test_assertable_bag_superbag_x_arity_2_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_superbag!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_bag_superbag!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1, 1]`"
        );
    }

    #[test]
    fn test_assertable_bag_superbag_x_arity_3_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assertable_bag_superbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_bag_superbag_x_arity_3_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assertable_bag_superbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assertable_bag_superbag_x_arity_3_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_superbag!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
