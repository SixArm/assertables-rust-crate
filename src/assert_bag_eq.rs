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
/// let b = [1, 1];
/// assert_bag_eq!(&a, &b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_eq!(&a, &b);
/// //-> panic!("…")
/// // assertion failed: `assert_bag_eq!(left, right)`
/// //   left: `[1, 1]`,
/// //  right: `[1, 1, 1]`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_bag_eq!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1, 1]`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashMap`] to count items.
#[macro_export]
macro_rules! assert_bag_eq {
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
                if a_bag == b_bag {
                    ()
                } else {
                    panic!("assertion failed: `assert_bag_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b)
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
                if a_bag == b_bag {
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
    fn test_assert_bag_eq_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x= assert_bag_eq!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_eq!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1, 1]`")]
    fn test_assert_bag_eq_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let _x = assert_bag_eq!(&a, &b);
    }

    #[test]
    fn test_assert_bag_eq_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assert_bag_eq!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_eq_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let _x = assert_bag_eq!(&a, &b, "message");
    }

}
