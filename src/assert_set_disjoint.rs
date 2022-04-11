/// Assert a set is disjoint with another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = [1, 2];
/// let b = [3, 4];
/// let x = assert_set_disjoint_as_result!(&a, &b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 2];
/// let b = [2, 3];
/// let x = assert_set_disjoint_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_set_disjoint!(left_set, right_set)`\n",
///     "  left_set label: `&a`,\n",
///     "  left_set debug: `[1, 2]`,\n",
///     " right_set label: `&b`,\n",
///     " right_set debug: `[2, 3]`,\n",
///     "            left: `{1, 2}`,\n",
///     "           right: `{2, 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_disjoint_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set.is_disjoint(&b_set) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_set_disjoint!(left_set, right_set)`\n",
                            "  left_set label: `{}`,\n",
                            "  left_set debug: `{:?}`,\n",
                            " right_set label: `{}`,\n",
                            " right_set debug: `{:?}`,\n",
                            "            left: `{:?}`,\n",
                            "           right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        &a_set, 
                        &b_set
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_set_disjoint_as_result_x_arity_2_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_disjoint_as_result!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_set_disjoint_as_result_x_arity_2_failure() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_disjoint_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_disjoint!(left_set, right_set)`\n",
                "  left_set label: `&a`,\n",
                "  left_set debug: `[1, 2]`,\n",
                " right_set label: `&b`,\n",
                " right_set debug: `[2, 3]`,\n",
                "            left: `{1, 2}`,\n",
                "           right: `{2, 3}`"
            )
        );
    }
}

/// Assert a set is disjoint with another.
///
/// * If true, return `()`.
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
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_set_disjoint!(&a, &b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [2, 3];
/// assert_set_disjoint!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_set_disjoint!(left_set, right_set)`\n",
///     "  left_set label: `&a`,\n",
///     "  left_set debug: `[1, 2]`,\n",
///     " right_set label: `&b`,\n",
///     " right_set debug: `[2, 3]`,\n",
///     "            left: `{1, 2}`,\n",
///     "           right: `{2, 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_disjoint {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_set_disjoint_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_set_disjoint_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
