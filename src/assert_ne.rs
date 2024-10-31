//! Assert an expression is not equal to another expression.
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

/// Assert an expression is not equal to another expression.
///
/// Pseudocode:<br>
/// a ≠ b
///
/// * If true, return Result `Ok(())`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
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
                            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_ne.html\n",
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
mod tests {

    #[test]
    fn test_assert_ne_as_result_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let result = assert_ne_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_ne_as_result_failure() {
        let a: i32 = 1;
        let b: i32 = 1;
        let result = assert_ne_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_ne!(a, b)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_ne.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
            )
        );
    }
}
