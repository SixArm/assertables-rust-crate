/// Assert a set is not equal to another.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = [1, 2];
/// let b = [3, 4];
/// let x = assert_set_ne_as_result!(&a, &b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 2];
/// let b = [1, 2];
/// let x = assert_set_ne_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
/// #    "assertion failed: `assert_set_ne!(left, right)`\n",
/// #    "  left: `{1, 2}`,\n",
/// #    " right: `{1, 2}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_ne_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set != b_set {
                    Ok(())
                } else {
                    Err(format!("assertion failed: `assert_set_ne!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", &a_set, &b_set))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_set_ne_as_result_x_arity_2_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_ne_as_result!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_set_ne_as_result_x_arity_2_failure() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assert_set_ne_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_ne!(left, right)`\n",
                "  left: `{1, 2}`,\n",
                " right: `{1, 2}`"
            )
        );
    }

}

/// Assert a set is not equal to another.
///
/// * When true, return `()`.
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
/// assert_set_ne!(&a, &b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [2, 1];
/// assert_set_ne!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_set_ne!(left, right)`\n",
///     "  left: `{1, 2}`,\n",
///     " right: `{1, 2}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_ne {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_set_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_set_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_set_ne_x_arity_2_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_ne!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_ne!(left, right)`\n  left: `{1, 2}`,\n right: `{1, 2}`")]
    fn test_assert_set_ne_x_arity_2_failure() {
        let a = [1, 2];
        let b = [2, 1];
        assert_set_ne!(&a, &b);
    }

    #[test]
    fn test_assert_set_ne_x_arity_3_success() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_ne!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_ne_x_arity_3_failure() {
        let a = [1, 2];
        let b = [2, 1];
        assert_set_ne!(&a, &b, "message");
    }

}
