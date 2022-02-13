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
/// # fn main() {
/// use std::io::Read;
/// let mut readable = "hello".as_bytes();
/// let pattern = "ell";
/// assert_read_to_string_contains!(readable, pattern);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// use std::io::Read;
/// let mut readable = "hello".as_bytes();
/// let pattern = "xyz";
/// assert_read_to_string_contains!(readable, pattern);
/// //-> panic!
/// // assertion failed: `assert_read_to_string_contains!(readable, pattern)`
/// //  readable: `\"hello\"`,
/// //   pattern: `\"xyx\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_read_to_string_contains!(readable, pattern)`\n readable: `\"hello\"`,\n  pattern: `\"xyz\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_read_to_string_contains {
    ($readable:expr, $pattern:expr $(,)?) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_contains!(readable, pattern)`\n readable.read_to_string error: `{:?}`", err),
        };
        if (readable_buffer.contains($pattern)) {
            ()
        } else {
            panic!("assertion failed: `assert_read_to_string_contains!(readable, pattern)`\n readable: `{:?}`,\n  pattern: `{:?}`", readable_buffer, $pattern);
        }
    });
    ($readable:expr, $pattern:expr, $($arg:tt)+) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        if (readable_buffer.contains($pattern)) {
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
    fn test_assert_read_to_string_contains_x_arity_2_success() {
        let mut readable = "alpha".as_bytes();
        let pattern = "lph";
        let x = assert_read_to_string_contains!(readable, pattern);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_contains!(readable, pattern)`\n readable: `\"alpha\"`,\n  pattern: `\"xyz\"`")]
    fn test_assert_read_to_string_contains_x_arity_2_failure() {
        let mut readable = "alpha".as_bytes();
        let pattern = "xyz";
        let _x = assert_read_to_string_contains!(readable, pattern);
    }

    #[test]
    fn test_assert_read_to_string_contains_x_arity_3_success() {
        let mut readable = "alpha".as_bytes();
        let pattern = "lph";
        let x = assert_read_to_string_contains!(readable, pattern, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_contains_x_arity_3_failure() {
        let mut readable = "alpha".as_bytes();
        let pattern = "xyz";
        let _x = assert_read_to_string_contains!(readable, pattern, "message");
    }

}
