//! Assert an expression is not equal to an expression.
//!
//! * If true, return Result `Ok(())`.
//!
//! * When false, return [`Err`] with a message and the values of the
//!   expressions with their debug representations.
//!
//! This macro provides the same statements as [`assert_`](macro.assert_.html),
//! except this macro returns a Result, rather than doing a panic.
//!
//! This macro is useful for runtime checks, such as checking parameters,
//! or sanitizing inputs, or handling different results in different ways.
//!
//! # Module macro
//!
//! * [`assert_ne_as_result`](macro@crate::assert_ne_as_result)
//!
//! # Rust standard macros
//!
//! * [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html)
//! * [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)

/// Assert an expression is not equal to an expression.
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
            (a_val, b_val) => {
                if a_val != b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_ne!(left, right)`\n",
                            "  left label: `{}`,\n",
                            "  left debug: `{:?}`,\n",
                            " right label: `{}`,\n",
                            " right debug: `{:?}`,\n",
                            "        left: `{:?}`,\n",
                            "       right: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_ne_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_ne_as_result!(a, b);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_ne_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_ne_as_result!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_ne!(left, right)`\n",
                "  left label: `a`,\n",
                "  left debug: `1`,\n",
                " right label: `b`,\n",
                " right debug: `1`,\n",
                "        left: `1`,\n",
                "       right: `1`"
            )
        );
    }
}
