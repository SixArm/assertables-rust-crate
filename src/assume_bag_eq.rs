/// Assume two bags are equal.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let a = [1, 1];
/// let b = [1, 1];
/// let x = assume_bag_eq!(&a, &b);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let a = [1, 1];
/// let b = [3, 4];
/// let x = assume_bag_eq!(&a, &b);
/// //-> Err("assume_bag_eq left:[1, 1] right:[3, 4]")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assume_bag_eq {
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
                if left_bag == right_bag {
                    Ok(true)
                } else {
                    Err(format!("assumption failed: `assume_bag_eq(left, right)`\n  left: `{:?}`\n right: `{:?}`", $left, $right))
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
                if left_bag == right_bag {
                    Ok(true)
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
    fn test_assume_bag_eq_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assume_bag_eq!(&a, &b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_bag_eq_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assume_bag_eq!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_bag_eq(left, right)`\n  left: `[1, 1]`\n right: `[1, 1, 1]`"
        );
    }

    #[test]
    fn test_assume_bag_eq_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assume_bag_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_bag_eq_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assume_bag_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
