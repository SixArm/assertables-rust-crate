/// Assert a condition is true.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// assert!(true);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // assert!(false);
/// //-> panic!("assertion failed: false")
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
        let _ = assert!(a);
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
        let _ = assert!(a, "message");
    }

}
