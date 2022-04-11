/// Assert one std::io::Read read_to_string() value is greater than another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # #[allow(unused_imports)]
/// use std::io::Read;
/// # fn main() {
/// let mut a = "bravo".as_bytes();
/// let mut b = "alpha".as_bytes();
/// let x = assert_read_to_string_gt_other_as_result!(a,b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// let x = assert_read_to_string_gt_other_as_result!(a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_gt_other!(left_reader, right_reader)`\n",
///     "  left_reader label: `a`,\n",
///     "  left_reader debug: `[]`,\n",
///     " right_reader label: `b`,\n",
///     " right_reader debug: `[]`,\n",
///     "               left: `\"alpha\"`,\n",
///     "              right: `\"bravo\"`",
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_gt_other_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        let mut a_string = String::new();
        let mut b_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        let b_result = $b_reader.read_to_string(&mut b_string);
        if a_result.is_err() || b_result.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_gt_other!(left_reader, right_reader)`\n",
                    "  left_reader label: `{}`,\n",
                    "  left_reader debug: `{:?}`,\n",
                    " right_reader label: `{}`,\n",
                    " right_reader debug: `{:?}`,\n",
                    "        left result: `{:?}`,\n",
                    "       right result: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_reader), $b_reader,
                a_result,
                b_result
            ))
        } else {
            let _a_size = a_result.unwrap();
            let _b_size = b_result.unwrap();
            if a_string > b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_gt_other!(left_reader, right_reader)`\n",
                        "  left_reader label: `{}`,\n",
                        "  left_reader debug: `{:?}`,\n",
                        " right_reader label: `{}`,\n",
                        " right_reader debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_reader), $b_reader,
                    a_string,
                    b_string
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_gt_other_as_result_x_arity_2_success() {
        let mut a = "bravo".as_bytes();
        let mut b = "alpha".as_bytes();
        let x = assert_read_to_string_gt_other_as_result!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_read_to_string_gt_other_as_result_x_arity_2_failure() {
        let mut a = "alpha".as_bytes();
        let mut b = "bravo".as_bytes();
        let x = assert_read_to_string_gt_other_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_gt_other!(left_reader, right_reader)`\n",
                "  left_reader label: `a`,\n",
                "  left_reader debug: `[]`,\n",
                " right_reader label: `b`,\n",
                " right_reader debug: `[]`,\n",
                "               left: `\"alpha\"`,\n",
                "              right: `\"bravo\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() value is greater than another.
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
/// use std::io::Read;
///
/// # fn main() {
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// assert_read_to_string_gt_other!(b, a);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// assert_read_to_string_gt_other!(a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_gt_other!(left_reader, right_reader)`\n",
///     "  left_reader label: `a`,\n",
///     "  left_reader debug: `[]`,\n",
///     " right_reader label: `b`,\n",
///     " right_reader debug: `[]`,\n",
///     "               left: `\"alpha\"`,\n",
///     "              right: `\"bravo\"`",
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_gt_other {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        match assert_read_to_string_gt_other_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_reader:expr, $($arg:tt)+) => ({
        match assert_read_to_string_gt_other_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
