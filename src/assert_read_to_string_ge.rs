/// Assert one std::io::Read read_to_string() value is greater than or equal to another.
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
/// let mut reader = "bravo".as_bytes();
/// let value = String::from("alpha");
/// let x = assert_read_to_string_ge_as_result!(reader, &value);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut reader = "alpha".as_bytes();
/// let value = String::from("bravo");
/// let x = assert_read_to_string_ge_as_result!(reader, &value);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_ge!(left_reader, right_expr)`\n",
///     " left_reader label: `reader`,\n",
///     " left_reader debug: `[]`,\n",
///     "  right_expr label: `&value`,\n",
///     "  right_expr debug: `\"bravo\"`,\n",
///     "              left: `\"alpha\"`,\n",
///     "             right: `\"bravo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_ge_as_result {
    ($a_reader:expr, $b_expr:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_ge!(left_reader, right_expr)`\n",
                    " left_reader label: `{}`,\n",
                    " left_reader debug: `{:?}`,\n",
                    "  right_expr label: `{}`,\n",
                    "  right_expr debug: `{:?}`,\n",
                    "          left err: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_expr), $b_expr,
                a_err
            ))
        } else {
            let _a_size = a_result.unwrap();
            let b_string = String::from($b_expr);
            if a_string >= b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_ge!(left_reader, right_expr)`\n",
                        " left_reader label: `{}`,\n",
                        " left_reader debug: `{:?}`,\n",
                        "  right_expr label: `{}`,\n",
                        "  right_expr debug: `{:?}`,\n",
                        "              left: `{:?}`,\n",
                        "             right: `{:?}`",
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_expr), $b_expr,
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
    fn test_assert_read_to_string_ge_as_result_x_arity_2_success() {
        let mut reader = "bravo".as_bytes();
        let value = String::from("alpha");
        let x = assert_read_to_string_ge_as_result!(reader, &value);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_read_to_string_ge_as_result_x_arity_2_failure() {
        let mut reader = "alpha".as_bytes();
        let value = String::from("bravo");
        let x = assert_read_to_string_ge_as_result!(reader, &value);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_ge!(left_reader, right_expr)`\n",
                " left_reader label: `reader`,\n",
                " left_reader debug: `[]`,\n",
                "  right_expr label: `&value`,\n",
                "  right_expr debug: `\"bravo\"`,\n",
                "              left: `\"alpha\"`,\n",
                "             right: `\"bravo\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() value is greater than or equal to another.
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
/// let mut reader = "bravo".as_bytes();
/// let value = String::from("alpha");
/// assert_read_to_string_ge!(reader, &value);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut reader = "alpha".as_bytes();
/// let value = String::from("bravo");
/// assert_read_to_string_ge!(reader, &value);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_ge!(left_reader, right_expr)`\n",
///     " left_reader label: `reader`,\n",
///     " left_reader debug: `[]`,\n",
///     "  right_expr label: `&value`,\n",
///     "  right_expr debug: `\"bravo\"`,\n",
///     "              left: `\"alpha\"`,\n",
///     "             right: `\"bravo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_ge {
    ($a_reader:expr,  $b_expr:expr $(,)?) => ({
        match assert_read_to_string_ge_as_result!($a_reader, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_read_to_string_ge_as_result!($a_reader, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
