//! Assert an expression is not equal to another.
//!
//! Pseudocode:<br>
//! a ≠ b
//!
//! # Module macro
//!
//! * [`assert_ne_as_result`](macro@crate::assert_ne_as_result)
//!
//! # Rust standard macros
//!
//! * [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html)
//! * [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)

/// Assert an expression is not equal to another.
///
/// Pseudocode:<br>
/// a ≠ b
///
/// * If true, return Result `Ok(())`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macro
///
/// * [`assert_ne_as_result`](macro@crate::assert_ne_as_result)
///
/// # Rust standard macros
///
/// * [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html)
/// * [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)
///
#[macro_export]
macro_rules! assert_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if a != b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_ne!(a, b)`\n",
                            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`"
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_ne_as_result {

    #[test]
    fn ne() {
        let a: i32 = 1;
        let b: i32 = 2;
        let actual = assert_ne_as_result!(a, b);
        assert_eq!(actual.unwrap(), ());
    }

    #[test]
    fn eq() {
        let a: i32 = 1;
        let b: i32 = 1;
        let actual = assert_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_ne.html\n",
            " a label: `a`,\n",
            " a debug: `1`,\n",
            " b label: `b`,\n",
            " b debug: `1`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}
