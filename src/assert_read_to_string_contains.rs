/// Assert a std::io::Read read_to_string() contains a pattern.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # #[allow(unused_imports)]
/// use std::io::Read;
/// # fn main() {
/// let mut reader = "hello".as_bytes();
/// let containee = "ell";
/// let x = assert_read_to_string_contains_as_result!(reader, containee);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut reader = "hello".as_bytes();
/// let containee = "xyz";
/// let x = assert_read_to_string_contains_as_result!(reader, containee);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_contains!(left_reader, right_expr)`\n",
///     " left reader name: `reader`,\n",
///     "  right expr name: `containee`,\n",
///     " left reader size: `5`,\n",
///     " left reader data: `\"hello\"`,\n",
///     "       right expr: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_contains_as_result {
    ($reader:expr, $b_pattern:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(msg_with_left_reader_and_right_expr_and_err!(
                "assertion failed",
                "assert_read_to_string_contains!",
                stringify!($reader),
                stringify!($b_pattern),
                a_err
            ))
        } else {
            if a_string.contains($b_pattern) {
                Ok(())
            } else {
                let a_size = a_result.unwrap();
                Err(msg_with_left_reader_and_right_expr!(
                    "assertion failed",
                    "assert_read_to_string_contains!",
                    stringify!($reader),
                    stringify!($b_pattern),
                    a_size,
                    a_string,
                    $b_pattern
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
    fn test_assert_read_to_string_contains_as_result_x_arity_2_success() {
        let mut reader = "alpha".as_bytes();
        let containee = "lph";
        let x = assert_read_to_string_contains_as_result!(reader, containee);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_read_to_string_contains_as_result_x_arity_2_failure() {
        let mut reader = "alpha".as_bytes();
        let containee = "xyz";
        let x = assert_read_to_string_contains_as_result!(reader, containee);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_contains!(left_reader, right_expr)`\n",
                " left reader name: `reader`,\n",
                "  right expr name: `containee`,\n",
                " left reader size: `5`,\n",
                " left reader data: `\"alpha\"`,\n",
                "       right expr: `\"xyz\"`"
            )
        );
    }
}

/// Assert a a std::io::Read read_to_string() contains a pattern.
///
/// * When true, return `()`.
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
/// let mut reader = "hello".as_bytes();
/// let containee = "ell";
/// assert_read_to_string_contains!(reader, containee);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let containee = "xyz";
/// assert_read_to_string_contains!(reader, containee);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_contains!(left_reader, right_expr)`\n",
///     " left reader name: `reader`,\n",
///     "  right expr name: `containee`,\n",
///     " left reader size: `5`,\n",
///     " left reader data: `\"hello\"`,\n",
///     "       right expr: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_contains {
    ($a_reader:expr, $b_pattern:expr $(,)?) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b_pattern) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_pattern:expr, $($arg:tt)+) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b_pattern) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_contains_x_arity_2_success() {
        let mut reader = "alpha".as_bytes();
        let containee = "lph";
        let x = assert_read_to_string_contains!(reader, containee);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_contains!(left_reader, right_expr)`\n left reader name: `reader`,\n  right expr name: `containee`,\n left reader size: `5`,\n left reader data: `\"alpha\"`,\n       right expr: `\"xyz\"")]
    fn test_assert_read_to_string_contains_x_arity_2_failure() {
        let mut reader = "alpha".as_bytes();
        let containee = "xyz";
        let _x = assert_read_to_string_contains!(reader, containee);
    }

    #[test]
    fn test_assert_read_to_string_contains_x_arity_3_success() {
        let mut reader = "alpha".as_bytes();
        let containee = "lph";
        let x = assert_read_to_string_contains!(reader, containee, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_contains_x_arity_3_failure() {
        let mut reader = "alpha".as_bytes();
        let containee = "xyz";
        let _x = assert_read_to_string_contains!(reader, containee, "message");
    }

}
