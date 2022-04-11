/// Assert a std::io::Read read_to_string() contains a pattern.
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
///     "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
///     "     left_reader label: `reader`,\n",
///     "     left_reader debug: `[]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_contains_as_result {
    ($a_reader:expr, $b_containee:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                    "     left_reader label: `{}`,\n",
                    "     left_reader debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "              left err: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_containee), $b_containee,
                a_err
            ))
        } else {
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                let _a_size = a_result.unwrap();
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                        "     left_reader label: `{}`,\n",
                        "     left_reader debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`",
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_containee), $b_containee,
                    a_string,
                    $b_containee
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
                "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                "     left_reader label: `reader`,\n",
                "     left_reader debug: `[]`,\n",
                " right_containee label: `containee`,\n",
                " right_containee debug: `\"xyz\"`,\n",
                "                  left: `\"alpha\"`,\n",
                "                 right: `\"xyz\"`"
            )
        );
    }
}

/// Assert a a std::io::Read read_to_string() contains a pattern.
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
///     "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
///     "     left_reader label: `reader`,\n",
///     "     left_reader debug: `[]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_contains {
    ($a_reader:expr, $b:expr $(,)?) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b:expr, $($arg:tt)+) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
