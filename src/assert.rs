/// Assert a condition is true.
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
/// // Return Ok
/// let x = assert_as_result!(true);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let x = assert_as_result!(false);
/// //-> Err(…)
/// assert!(x.is_err());
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
    fn test_assert_x_success_as_result() {
        let a = true;
        let x = assert_as_result!(a);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

}

/// Assert a condition is true.
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
/// // Return Ok
/// assert!(true);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert!(false);
/// //-> panic!
/// });
/// assert!(result.is_err());
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
    fn test_assert_x_success() {
        let a = true;
        let x = assert!(a);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic]
    fn test_assert_x_failure() {
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
    #[should_panic]
    fn test_assert_x_arity_3_failure() {
        let a = false;
        let _x = assert!(a, "message");
    }

}
