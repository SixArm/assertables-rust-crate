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
/// use std::io::Read;
/// 
/// # fn main() {
/// let mut readable = "hello".as_bytes();
/// let string = "hello";
/// assert_read_to_string_eq_string!(readable, string);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut readable = "hello".as_bytes();
/// let string = "world";
/// assert_read_to_string_eq_string!(readable, string);
/// //-> panic!
/// // assertion failed: `assert_read_to_string_eq_string!(readable, string)`
/// //  readable: `\"hello\"`,
/// //    string: `\"world\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_read_to_string_eq_string!(readable, string)`\n readable: `\"hello\"`,\n   string: `\"world\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_read_to_string_eq_string {
    ($readable:expr, $string:expr $(,)?) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_eq_string!(readable, string)`\n readable.read_to_string error: `{:?}`", err),
        };
        let expect = String::from($string);
        if (readable_buffer == expect) {
            ()
        } else {
            panic!("assertion failed: `assert_read_to_string_eq_string!(readable, string)`\n readable: `{:?}`,\n   string: `{:?}`", readable_buffer, $string);
        }
    });
    ($readable:expr, $string:expr, $($arg:tt)+) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        let expect = String::from($string);
        if (readable_buffer == expect) {
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
    fn test_assert_read_to_string_eq_string_x_arity_2_success() {
        let mut readable = "a".as_bytes();
        let string = "a";
        let x = assert_read_to_string_eq_string!(readable, string);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_eq_string!(readable, string)`\n readable: `\"a\"`,\n   string: `\"b\"`")]
    fn test_assert_read_to_string_eq_string_x_arity_2_failure() {
        let mut readable = "a".as_bytes();
        let string = "b";
        let _x = assert_read_to_string_eq_string!(readable, string);
    }

    #[test]
    fn test_assert_read_to_string_eq_string_x_arity_3_success() {
        let mut readable = "a".as_bytes();
        let string = "a";
        let x = assert_read_to_string_eq_string!(readable, string, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_eq_string_x_arity_3_failure() {
        let mut readable = "a".as_bytes();
        let string = "b";
        let _x = assert_read_to_string_eq_string!(readable, string, "message");
    }

}
