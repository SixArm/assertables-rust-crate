/// Assert an expression is equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a = 1;
/// let b = 1;
/// let x = assert_eq_as_result!(a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// assert_eq!(x.unwrap(), ());
///
/// let a = 1;
/// let b = 2;
/// let x = assert_eq_as_result!(a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_eq!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `1`,\n",
///     " right label: `b`,\n",
///     " right debug: `2`,\n",
///     "        left: `1`,\n",
///     "       right: `2`",
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
/// 
/// # Related
/// 
/// * [`assert_eq`]
/// * [`assert_eq_as_result`]
/// * [`debug_assert_eq`]
///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => ({
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
                        stringify!($a), $a,
                        stringify!($b), $b,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_eq_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq_as_result!(a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_eq_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x =  assert_eq_as_result!(a, b);
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
/// # Related
/// 
/// * [`assert_eq`]
/// * [`assert_eq`]
/// * [`debug_assert_eq`]
/// 
#[macro_export]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_eq!($($arg)*);
        }
    };
}
