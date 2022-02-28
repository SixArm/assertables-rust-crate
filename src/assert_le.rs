/// Assert one value is less than or equal to another value.
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
/// let x = assert_le_as_result!(1, 2);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let x = assert_le_as_result!(2, 1);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_le!(left, right)`\n",
///     "  left: `2`,\n",
///     " right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_le_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val <= b_val {
                    Ok(())
                } else {
                    Err(msg_with_left_and_right!("assertion failed", "assert_le!", a_val, b_val))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_le_as_result_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_le_as_result!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_le_as_result_x_arity_2_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_le_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_le!(left, right)`\n",
                "  left: `2`,\n",
                " right: `1`"
            )
        );
    }
}

/// Assert a value is less than or equal to another.
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
/// assert_le!(1, 2);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// assert_le!(2, 1);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_le!(left, right)`\n",
///     "  left: `2`,\n",
///     " right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_le {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_le_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_le_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_le_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_le!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_le!(left, right)`\n  left: `2`,\n right: `1`")]
    fn test_assert_le_x_arity_2_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let _x = assert_le!(a, b);
    }

    #[test]
    fn test_assert_le_x_arity_3_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_le!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_le_x_arity_3_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let _x = assert_le!(a, b, "message");
    }

}
