/// Assert a std::io::Read read_to_string() value is greater than or equal to another.
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
/// # fn main() {
/// use std::io::Read;
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// assert_std_io_read_to_string_ge!(b, a);
/// //-> ()
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// # let result = panic::catch_unwind(|| {
/// use std::io::Read;
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// assert_std_io_read_to_string_ge!(a, b);
/// # });
/// # let err: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # assert_eq!(err, "assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`");
/// //-> panic!("assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_std_io_read_to_string_ge {
    ($left:expr, $right:expr $(,)?) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n  left read_to_string error: `{:?}`", err),
        };
        let _right_size = match $right.read_to_string(&mut right_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n right read_to_string error: `{:?}`", err),
        };
        if (left_buffer >= right_buffer) {
            ()
        } else {
            panic!("assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", left_buffer, right_buffer);
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        let _right_size = match $right.read_to_string(&mut right_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n right read_to_string error: `{:?}`", err),
        };
        if (left_buffer >= right_buffer) {
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
    fn test_assert_std_io_read_to_string_ge_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assert_std_io_read_to_string_ge!(b, a);
        assert_eq!(
            x, 
            ()
        );
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_std_io_read_to_string_ge!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`")]
    fn test_assert_std_io_read_to_string_ge_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let _x = assert_std_io_read_to_string_ge!(a, b);
    }

    #[test]
    fn test_assert_assert_std_io_read_to_string_ge_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assert_std_io_read_to_string_ge!(b, a, "message");
        assert_eq!(
            x, 
            ()
        );
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_assert_std_io_read_to_string_ge_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let _x = assert_std_io_read_to_string_ge!(a, b, "message");
    }

}
