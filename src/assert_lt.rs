/// Assert one value is less than another.
///
/// * If true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = 1;
/// let b = 2;
/// let x = assert_lt_as_result!(a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = 2;
/// let b = 1;
/// let x = assert_lt_as_result!(a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_lt!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `2`,\n",
///     " right label: `b`,\n",
///     " right debug: `1`,\n",
///     "        left: `2`,\n",
///     "       right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_lt_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val < b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_lt!(left, right)`\n",
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
mod test_assert_x_result {

    #[test]
    fn test_assert_lt_as_result_x_arity_2_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_lt_as_result!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_lt_as_result_x_arity_2_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_lt_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_lt!(left, right)`\n",
                "  left label: `a`,\n",
                "  left debug: `2`,\n",
                " right label: `b`,\n",
                " right debug: `1`,\n",
                "        left: `2`,\n",
                "       right: `1`"
            )
        );
    }
}

/// Assert a value is less than another.
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
/// let a = 1;
/// let b = 2;
/// assert_lt!(a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = 2;
/// let b = 1;
/// assert_lt!(a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_lt!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `2`,\n",
///     " right label: `b`,\n",
///     " right debug: `1`,\n",
///     "        left: `2`,\n",
///     "       right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_lt {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_lt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_lt_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
