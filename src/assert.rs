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
/// # fn main() {
/// assert!(true);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// # let result = panic::catch_unwind(|| {
/// assert!(false);
/// # });
/// # let err: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # assert_eq!(err, "assertion failed: false");
/// //-> panic!("assertion failed: false");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.

// `assert_eq` macro is provided by Rust `std`.

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_x_arity_2_success() {
        let a = true;
        let x = assert!(a);
        assert_eq!(
            x, 
            ()
        );
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
        assert_eq!(
            x, 
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_x_arity_3_failure() {
        let a = false;
        let _x = assert!(a, "message");
    }

}
