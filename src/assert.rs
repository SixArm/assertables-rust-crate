/// Assert a condition is true.
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
/// let x = assert_as_result!(true);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let x = assert_as_result!(false);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
/// #    "assertion failed: `assertable!(condition)`\n",
/// #    " condition: `false`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_as_result {
    ($x:expr $(,)?) => ({
        if $x {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assertable!(condition)`\n",
                    " condition: `{:?}`"
                ),
                $x
            ))
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_x_arity_2_success_as_result() {
        let a = true;
        let x = assert_as_result!(a);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

}

/// Assert a condition is true.
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
/// assert!(true);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// assert!(false);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: false"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```


// `assert_eq` macro is provided by Rust `std`.

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_x_arity_2_success() {
        let a = true;
        let x = assert!(a);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: a")]
    fn test_assert_x_arity_2_failure() {
        let a = false;
        let _x = assert!(a);
    }

    #[test]
    fn test_assert_x_arity_3_success() {
        let a = true;
        let x = assert!(a, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_x_arity_3_failure() {
        let a = false;
        let _x = assert!(a, "message");
    }

}
