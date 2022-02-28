/// Assert a a std::io::Read read_to_string() is a match to a given regex.
///
/// * When true, return `()`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_read_to_string_matches_as_result!(reader, matcher);
/// //-> ()
///
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"xyz").unwrap();
/// let x = assert_read_to_string_matches_as_result!(reader, matcher);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_matches!(left_reader, right_expr)`\n",
///     " left reader name: `reader`,\n",
///     "  right expr name: `matcher`,\n",
///     " left reader size: `5`,\n",
///     " left reader data: `\"hello\"`,\n",
///     "       right expr: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_matches_as_result {
    ($reader:expr, $matcher:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(msg_with_left_reader_and_right_expr_and_err!(
                "assertion failed",
                "assert_read_to_string_matches!",
                stringify!($reader),
                stringify!($matcher),
                a_err
            ))
        } else {
            let a_size = a_result.unwrap();
            if $matcher.is_match(a_string.as_str()) {
                Ok(())
            } else {
                Err(msg_with_left_reader_and_right_expr!(
                    "assertion failed",
                    "assert_read_to_string_matches!",
                    stringify!($reader),
                    stringify!($matcher),
                    a_size,
                    a_string,
                    $matcher
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {
    use std::io::Read;
    use regex::Regex;

    #[test]
    fn test_assert_read_to_string_matches_as_result_x_arity_2_success() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_matches_as_result!(reader, matcher);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_read_to_string_matches_as_result_x_arity_2_failure() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"xyz").unwrap();
        let x = assert_read_to_string_matches_as_result!(reader, matcher);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_matches!(left_reader, right_expr)`\n",
                " left reader name: `reader`,\n",
                "  right expr name: `matcher`,\n",
                " left reader size: `5`,\n",
                " left reader data: `\"alpha\"`,\n",
                "       right expr: `xyz`"
            )
        );
    }
}

/// Assert a a std::io::Read read_to_string() is a match to a given regex.
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
/// use regex::Regex;
///
/// # fn main() {
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_read_to_string_matches!(reader, matcher);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"xyz").unwrap();
/// assert_read_to_string_matches!(reader, matcher);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_matches!(left_reader, right_expr)`\n",
///     " left reader name: `reader`,\n",
///     "  right expr name: `matcher`,\n",
///     " left reader size: `5`,\n",
///     " left reader data: `\"hello\"`,\n",
///     "       right expr: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_matches {
    ($a_reader:expr, $b_matcher:expr $(,)?) => ({
        match assert_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_matcher:expr, $($arg:tt)+) => ({
        match assert_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {
    use std::io::Read;
    use regex::Regex;

    #[test]
    fn test_assert_read_to_string_matches_x_arity_2_success() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_matches!(reader, matcher);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_matches!(left_reader, right_expr)`\n left reader name: `reader`,\n  right expr name: `matcher`,\n left reader size: `5`,\n left reader data: `\"alpha\"`,\n       right expr: `xyz`")]
    fn test_assert_read_to_string_matches_x_arity_2_failure() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"xyz").unwrap();
        let _x = assert_read_to_string_matches!(reader, matcher);
    }

    #[test]
    fn test_assert_read_to_string_matches_x_arity_3_success() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_matches!(reader, matcher, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_matches_x_arity_3_failure() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"xyz").unwrap();
        let _x = assert_read_to_string_matches!(reader, matcher, "message");
    }

}
