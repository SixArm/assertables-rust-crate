//! Assert a condition is true.
//!
//! Pseudocode:<br>
//! condition
//!
//! * If true, return Result `Ok(())`.
//!
//! * Otherwise, return Result `Err(message)`.
//!
//! This macro provides the same statements as [`assert`](macro@assert),
//! except this macro returns a Result, rather than doing a panic.
//!
//! This macro is useful for runtime checks, such as checking parameters,
//! or sanitizing inputs, or handling different results in different ways.
//!
//! # Module macros
//!
//! * [`assert_as_result`](macro.assert_as_result.html)

/// Assert a condition is true.
///
/// Pseudocode:<br>
/// condition
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert`](macro@assert),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_as_result`](macro.assert_as_result.html)
///
#[macro_export]
macro_rules! assert_as_result {
    ($a:expr $(,)?) => {{
        match ($a) {
            a => {
                if a {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert!(condition)`\n",
                            "https://docs.rs/assertables/9.2.0/assertables/macro.assert.html\n",
                            " condition label: `{}`,\n",
                            " condition debug: `{:?}`,\n",
                        ),
                        stringify!($a),
                        a,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_as_result_x_success() {
        let a = true;
        let result = assert_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_as_result_x_failure() {
        let a = false;
        let result = assert_as_result!(a);
        assert_eq!(result.is_err(), true);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert!(condition)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert.html\n",
            " condition label: `a`,\n",
            " condition debug: `false`,\n",
        );
        assert_eq!(actual, expect);
    }
}
