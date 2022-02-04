/// Assure two bags are not equal.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let x = assure_bag_ne!([1, 1], [1, 1, 1]);
/// //-> Ok(true)
///
/// let x = assure_bag_ne!([1, 1], [1, 1]);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assure_bag_ne {
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
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
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
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_bag_ne_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assure_bag_ne!(&a, &b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_bag_ne_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assure_bag_ne!(&a, &b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_bag_ne_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assure_bag_ne!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            true
        )
    }

    #[test]
    fn test_assure_bag_ne_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assure_bag_ne!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
