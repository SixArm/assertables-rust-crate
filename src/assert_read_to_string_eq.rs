/// Assert a read_to_string() value is equal to another.
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
/// let mut a = "a".as_bytes();
/// let mut b = "a".as_bytes();
/// assert_read_to_string_eq!(a, b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// assert_read_to_string_eq!(a, b);
/// //-> panic!
/// // assertion failed: `assert_read_to_string_eq!(left, right)`
/// //   left: `\"a\"`,
/// //  right: `\"b\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_read_to_string_eq!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_read_to_string_eq {
    ($a:expr, $b:expr $(,)?) => ({
        let mut a_buffer = String::new();
        let mut b_buffer = String::new();
        let _a_size = match $a.read_to_string(&mut a_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_eq!(left, right)`\n  left read_to_string error: `{:?}`", err),
        };
        let _b_size = match $b.read_to_string(&mut b_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_eq!(left, right)`\n right read_to_string error: `{:?}`", err),
        };
        if (a_buffer == b_buffer) {
            ()
        } else {
            panic!("assertion failed: `assert_read_to_string_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", a_buffer, b_buffer);
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        let mut a_buffer = String::new();
        let mut b_buffer = String::new();
        let _a_size = match $a.read_to_string(&mut a_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        let _b_size = match $b.read_to_string(&mut b_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_eq!(left, right)`\n right read_to_string error: `{:?}`", err),
        };
        if (a_buffer == b_buffer) {
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
    fn test_assert_read_to_string_eq_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let mut b = "a".as_bytes();
        let x = assert_read_to_string_eq!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_eq!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`")]
    fn test_assert_read_to_string_eq_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let _x = assert_read_to_string_eq!(a, b);
    }

    #[test]
    fn test_assert_read_to_string_eq_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let mut b = "a".as_bytes();
        let x = assert_read_to_string_eq!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_eq_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let _x = assert_read_to_string_eq!(a, b, "message");
    }

}
