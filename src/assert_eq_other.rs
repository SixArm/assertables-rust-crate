/// Assert a values is equal to another.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let x = assert_eq_as_result!(1, 1);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assert_eq_as_result!(1, 2);
/// //-> Err(…)
/// assert_eq!(actual, expect);

/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_eq!(left, right)`

 left: `1`,\n right: `2`".to_string());
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val == b_val {
                    Ok(())
                } else {
                    Err(format!("assertion failed: `assert_eq!(left, right)`

 left: `{:?}`,\n right: `{:?}`", $a, $b))
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
            "assertion failed: `assert_eq!(left, right)`

 left: `1`,\n right: `2`"
        );
    }
}
