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
