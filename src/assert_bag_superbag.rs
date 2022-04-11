/// Assert a bag is a superbag of another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// let x = assert_bag_superbag_as_result!(&a, &b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 1];
/// let b = [2, 2];
/// let x = assert_bag_superbag_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[2, 2]`,\n",
///     "            left: `{1: 2}`,\n",
///     "           right: `{2: 2}`"
/// );
/// assert_eq!(actual, expect);

///
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// let x = assert_bag_superbag_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[1, 1, 1]`,\n",
///     "            left: `{1: 2}`,\n",
///     "           right: `{1: 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeMap`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_bag_superbag_as_result {
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
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
                            "  left_bag label: `{}`,\n",
                            "  left_bag debug: `{:?}`,\n",
                            " right_bag label: `{}`,\n",
                            " right_bag debug: `{:?}`,\n",
                            "            left: `{:?}`,\n",
                            "           right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        &a_bag,
                        &b_bag
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_bag_superbag_as_result_x_arity_2_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assert_bag_superbag_as_result!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_bag_superbag_as_result_x_arity_2_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assert_bag_superbag_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
                "  left_bag label: `&a`,\n",
                "  left_bag debug: `[1, 1]`,\n",
                " right_bag label: `&b`,\n",
                " right_bag debug: `[2, 2]`,\n",
                "            left: `{1: 2}`,\n",
                "           right: `{2: 2}`"
            )
        );
    }

    #[test]
    fn test_assert_bag_superbag_as_result_x_arity_2_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_superbag_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
                "  left_bag label: `&a`,\n",
                "  left_bag debug: `[1, 1]`,\n",
                " right_bag label: `&b`,\n",
                " right_bag debug: `[1, 1, 1]`,\n",
                "            left: `{1: 2}`,\n",
                "           right: `{1: 3}`"
            )
        );
    }
}

/// Assert a bag is a superbag of another.
///
/// * If true, return `()`.
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
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_superbag!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[2, 2]`,\n",
///     "            left: `{1: 2}`,\n",
///     "           right: `{2: 2}`"
/// );
/// 
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_superbag!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[1, 1, 1]`,\n",
///     "            left: `{1: 2}`,\n",
///     "           right: `{1: 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }

/// ```

///
/// This implementation uses [`BTreeMap`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_bag_superbag {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_bag_superbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_bag_superbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
