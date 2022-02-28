/// Assert one value is greater than another.
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
/// let x = assert_gt_as_result!(2, 1);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let x = assert_gt_as_result!(1, 2);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_gt!(left, right)`\n",
///     "  left: `1`,\n",
///     " right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val > b_val {
                    Ok(())
                } else {
                    Err(msg_with_left_and_right!("assertion failed", "assert_gt!", a_val, b_val))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_gt_as_result_x_arity_2_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_gt_as_result!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_gt_as_result_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_gt_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_gt!(left, right)`\n",
                "  left: `1`,\n",
                " right: `2`"
            )
        );
    }
}

/// Assert a value is greater than another.
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
/// assert_gt!(2, 1);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// assert_gt!(1, 2);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_gt!(left, right)`\n",
///     "  left: `1`,\n",
///     " right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val > b_val {
                    ()
                } else {
                    panic!("{}", msg_with_left_and_right!("assertion failed", "assert_gt!", $a, $b))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                if a_val > b_val {
                    ()
                } else {
                    panic!("{:?}", $($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_gt_x_arity_2_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_gt!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_gt!(left, right)`\n  left: `1`,\n right: `2`")]
    fn test_assert_gt_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let _x = assert_gt!(a, b);
    }

    #[test]
    fn test_assert_gt_x_arity_3_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_gt!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_gt_x_arity_3_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let _x = assert_gt!(a, b, "message");
    }

}
