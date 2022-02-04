/// Assure a std::io::Read read_to_string() value is less than anoter.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::io::Read;
/// # fn main() {
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// let x = assure_std_io_read_to_string_lt!(a, b);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::io::Read;
/// # fn main() {
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// let x = assure_std_io_read_to_string_lt!(b, a);
/// assert_eq!(x.unwrap(), false);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_std_io_read_to_string_lt {
    ($left:expr, $right:expr $(,)?) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(_err) => 0,
        };
        let _right_size = match $right.read_to_string(&mut right_buffer) {
            Ok(size) => size,
            Err(_err) => 0,
        };
        if (left_buffer < right_buffer) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, String>);
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let _left_size = match $left.read_to_string(&mut left_buffer) {
            Ok(size) => size,
            Err(_err) => 0,
        };
        let _right_size = match $right.read_to_string(&mut right_buffer) {
            Ok(size) => size,
            Err(_err) => 0,
        };
        if (left_buffer < right_buffer) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn test_assure_std_io_read_to_string_lt_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assure_std_io_read_to_string_lt!(a, b);
        assert_eq!(
            x,
            Ok(true)
        );
    }

    #[test]
    fn test_assure_std_io_read_to_string_lt_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assure_std_io_read_to_string_lt!(b, a);
        assert_eq!(
            x,
            Ok(false)
        );
    }

    #[test]
    fn test_assert_assure_std_io_read_to_string_lt_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assure_std_io_read_to_string_lt!(a, b, "message");
        assert_eq!(
            x,
            Ok(true)
        );
    }

    #[test]
    fn test_assert_assure_std_io_read_to_string_lt_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assure_std_io_read_to_string_lt!(b, a, "message");
        assert_eq!(
            x,
            Ok(false)
        );
    }

}
