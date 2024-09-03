/// Assert an expression is equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_eq`](macro.assert_eq.html)
/// * [`assert_eq_as_result`](macro.assert_eq_as_result.html)
/// * [`debug_assert_eq`](macro.debug_assert_eq.html)
///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val == b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_eq!(left, right)`\n",
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
    fn test_assert_eq_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq_as_result!(a, b);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_eq_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_eq_as_result!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_eq!(left, right)`\n",
                "  left label: `a`,\n",
                "  left debug: `1`,\n",
                " right label: `b`,\n",
                " right debug: `2`,\n",
                "        left: `1`,\n",
                "       right: `2`"
            )
        );
    }
}

/// Assert an expression is equal to an expression.
///
/// This macro provides the same statements as [`assert_eq`],
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_eq`](macro.assert_eq.html)
/// * [`assert_eq`](macro.assert_eq.html)
/// * [`debug_assert_eq`](macro.debug_assert_eq.html)
///
#[macro_export]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_eq!($($arg)*);
        }
    };
}
