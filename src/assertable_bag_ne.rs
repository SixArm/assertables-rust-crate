/// Assert a bag is not equal to another.
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
/// let x = assertable_bag_ne!(&a, &b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let a = [1, 1];
/// let b = [1, 1];
/// let x = assertable_bag_ne!(&a, &b);
/// //-> Err("…")
/// // assertable failed: `assertable_bag_ne!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[1, 1]`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_bag_ne!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1]`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assertable_bag_ne {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut b_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if a_bag != b_bag {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_bag_ne!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut b_bag: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if a_bag != b_bag {
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
    fn test_assertable_bag_ne_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_ne!(&a, &b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_bag_ne_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assertable_bag_ne!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_bag_ne!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1]`"
        );
    }

    #[test]
    fn test_assertable_bag_ne_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assertable_bag_ne!(&a, &b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_bag_ne_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assertable_bag_ne!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
