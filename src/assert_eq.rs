/// Assert a values is equal to another.
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
/// let x = assert_eq_as_result!(1, 1);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assert_eq_as_result!(1, 2);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_eq!(left, right)`\n",
///     "  left: `1`,\n",
///     " right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val == b_val {
                    Ok(())
                } else {
                    Err(msg_with_left_and_right!("assertion failed", "assert_eq!", $a, $b))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_eq_as_result_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq_as_result!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assert_eq_as_result_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x =  assert_eq_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_eq!(left, right)`\n",
                "  left: `1`,\n",
                " right: `2`"
            )
        );
    }
}

/// Assert a value is equal to another.
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
/// assert_eq!(1, 1);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// assert_eq!(1, 2);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `(left == right)`\n",
///     "  left: `1`,\n",
///     " right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```


// `assert_eq` macro is provided by Rust `std`.

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_eq_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`")]
    fn test_assert_eq_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        assert_eq!(a, b);
    }

    #[test]
    fn test_assert_eq_x_arity_3_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_eq_x_arity_3_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        assert_eq!(a, b, "message");
    }

}
