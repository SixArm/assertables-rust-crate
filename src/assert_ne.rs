/// Assert two values are not equal.
///
/// * When true, return `Ok(true)`.
///
/// * When false, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// assert_ne!(1, 2);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // assert_ne!(1, 1);
/// //-> panic!("assertion failed: `(left != right)`\n  left: `1`,\n right: `1`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.

// `assert_ne` macro is provided by Rust `std`.

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assert_ne!(a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `(left != right)`\n  left: `1`,\n right: `1`")]
    fn test_assert_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        assert_ne!(a, b);
    }

    #[test]
    fn test_assert_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assert_ne!(a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        assert_ne!(a, b, "message");
    }

}
