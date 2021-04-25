/// Assert two values are equal.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// assert_eq!(1, 1);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// // assert_eq!(1, 2);
/// //-> panic!("assertion failed: `(left == right)`\n  left: `1`,\n right: `2`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.

// `assert_eq` macro is provided by Rust `std`.

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_eq_x_arity_2_success() {
        let a = 1;
        let b = 1;
        let x = assert_eq!(a, b);
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`")]
    fn test_assert_eq_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        assert_eq!(a, b);
    }

    #[test]
    fn test_assert_eq_x_arity_3_success() {
        let a = 1;
        let b = 1;
        let x = assert_eq!(a, b, "message");
        assert_eq!(
            x,
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_eq_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        assert_eq!(a, b, "message");
    }

}
