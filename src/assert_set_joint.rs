/// Assert a set is joint with another.
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
/// let b = [2, 3];
/// let x = assert_set_joint_as_result!(&a, &b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 2];
/// let b = [3, 4];
/// let x = assert_set_joint_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_set_joint!(left, right)`\n",
///     "  left: `{1, 2}`,\n",
///     " right: `{3, 4}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_joint_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if !a_set.is_disjoint(&b_set) {
                    Ok(())
                } else {
                    Err(msg_with_left_and_right!("assertion failed", "assert_set_joint!", &a_set, &b_set))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_set_joint_as_result_x_arity_2_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_joint_as_result!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_set_joint_as_result_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assert_set_joint_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_joint!(left, right)`\n",
                "  left: `{1, 2}`,\n",
                " right: `{3, 4}`"
            )
        );
    }

}

/// Assert a set is joint with another.
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
/// let b = [2, 3];
/// assert_set_joint!(&a, &b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_set_joint!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_set_joint!(left, right)`\n",
///     "  left: `{1, 2}`,\n",
///     " right: `{3, 4}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_set_joint {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_set_joint_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_set_joint_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_set_joint_x_arity_2_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_joint!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_set_joint!(left, right)`\n  left: `{1, 2}`,\n right: `{3, 4}`")]
    fn test_assert_set_joint_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_joint!(&a, &b);
    }

    #[test]
    fn test_assert_set_joint_x_arity_3_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assert_set_joint!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_set_joint_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        assert_set_joint!(&a, &b, "message");
    }

}
