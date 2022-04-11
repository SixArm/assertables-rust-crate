/// Assert a values is equal to another.
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
/// let a = 1;
/// let b = 1;
/// let x = assert_eq_as_result!(a, b);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let a = 1;
/// let b = 2;
/// let x = assert_eq_as_result!(a, b);
/// //-> Err(…)
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
    fn test_assert_eq_as_result_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let x = assert_eq_as_result!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assert_eq_as_result_x_arity_2_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x =  assert_eq_as_result!(a, b);
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
