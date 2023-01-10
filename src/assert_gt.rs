/// Assert an expression is greater than another.
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
/// let a = 2;
/// let b = 1;
/// let x = assert_gt_as_result!(a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = 1;
/// let b = 2;
/// let x = assert_gt_as_result!(a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_gt!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `1`,\n",
///     " right label: `b`,\n",
///     " right debug: `2`,\n",
///     "        left: `1`,\n",
///     "       right: `2`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val > b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_gt!(left, right)`\n",
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
    fn test_assert_gt_as_result_x_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_gt_as_result!(a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_gt_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_gt_as_result!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_gt!(left, right)`\n",
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

/// Assert a value is greater than another.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a = 2;
/// let b = 1;
/// assert_gt!(a, b);
/// //-> ()
///
/// let a = 1;
/// let b = 2;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_gt!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_gt!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `1`,\n",
///     " right label: `b`,\n",
///     " right debug: `2`,\n",
///     "        left: `1`,\n",
///     "       right: `2`"
/// );
/// assert_eq!(actual, expect);
/// 
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_gt!(a, b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
