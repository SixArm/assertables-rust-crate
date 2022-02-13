/// Assert a read_to_string() value is equal to a string.
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
/// # fn main() {
/// use std::io::Read;
/// let mut a = "a".as_bytes();
/// let str = "a";
/// assert_read_to_string_eq_str!(a, str);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// use std::io::Read;
/// let mut a = "a".as_bytes();
/// let str = "b";
/// assert_read_to_string_eq_str!(a, str);
/// //-> panic!
/// // assertion failed: `assert_read_to_string_eq_str!(left, str)`
/// //  left: `\"a\"`,
/// //   str: `\"b\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_read_to_string_eq_str!(left, str)`\n left: `\"a\"`,\n  str: `\"b\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_read_to_string_eq_str {
    ($left:expr, $str:expr $(,)?) => ({
        let mut left_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_eq_str!(left, str)`\n left read_to_string error: `{:?}`", err),
        };
        if (left_buffer == $str) {
            ()
        } else {
            panic!("assertion failed: `assert_read_to_string_eq_str!(left, str)`\n left: `{:?}`,\n  str: `{:?}`", left_buffer, $str);
        }
    });
    ($left:expr, $str:expr, $($arg:tt)+) => ({
        let mut left_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        if (left_buffer == $str) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_eq_str_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let str = "a";
        let x = assert_read_to_string_eq_str!(a, str);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_eq_str!(left, str)`\n left: `\"a\"`,\n  str: `\"b\"`")]
    fn test_assert_read_to_string_eq_str_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let str = "b";
        let _x = assert_read_to_string_eq_str!(a, str);
    }

    #[test]
    fn test_assert_assert_read_to_string_eq_str_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let str = "a";
        let x = assert_read_to_string_eq_str!(a, str, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_assert_read_to_string_eq_str_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let str = "b";
        let _x = assert_read_to_string_eq_str!(a, str, "message");
    }

}
