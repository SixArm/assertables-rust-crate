//! Assert an expression is equal to another.
//!
//! Pseudocode:<br>
//! a = b
//!
//! # Module macro
//!
//! * [`assert_eq_as_result`](macro@crate::assert_eq_as_result)
//!
//! # Rust standard macros
//!
//! * [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html)
//! * [`debug_assert_eq`](https://doc.rust-lang.org/std/macro.debug_assert_eq.html)

/// Assert an expression is equal to another.
///
/// Pseudocode:<br>
/// a = b
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_eq_as_result`](macro@crate::assert_eq_as_result)
///
/// # Rust standard macros
///
/// * [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html)
/// * [`debug_assert_eq`](https://doc.rust-lang.org/std/macro.debug_assert_eq.html)
///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if a == b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_eq!(a, b)`\n",
                            "https://docs.rs/assertables/9.5.5/assertables/macro.assert_eq.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
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
mod test_assert_eq_as_result {

    #[test]
    fn success() {
        let a: i8 = 1;
        let b: i8 = 1;
        let actual = assert_eq_as_result!(a, b);
        assert_eq!(actual.unwrap(), ());
    }

    #[test]
    fn failure() {
        let a: i8 = 1;
        let b: i8 = 2;
        let actual = assert_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_eq!(a, b)`\n",
            "https://docs.rs/assertables/9.5.5/assertables/macro.assert_eq.html\n",
            " a label: `a`,\n",
            " a debug: `1`,\n",
            " b label: `b`,\n",
            " b debug: `2`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}
