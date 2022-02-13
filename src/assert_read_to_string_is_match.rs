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
/// use regex::Regex;
/// 
/// # fn main() {
/// use std::io::Read;
/// let mut readable = "hello".as_bytes();
/// let matchable = Regex::new(r"ell").unwrap();
/// assert_read_to_string_is_match!(readable, matchable);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// use std::io::Read;
/// let mut readable = "hello".as_bytes();
/// let matchable = Regex::new(r"xyz").unwrap();
/// assert_read_to_string_is_match!(readable, matchable);
/// //-> panic!
/// // assertion failed: `assert_read_to_string_is_match!(readable, matchable)`
/// //  readable: `\"hello\"`,
/// //  matchable: `\"xyx\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_read_to_string_is_match!(readable, matchable)`\n readable: `\"hello\"`,\n matchable: `xyz`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_read_to_string_is_match {
    ($readable:expr, $matchable:expr $(,)?) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(err) => panic!("assertion failed: `assert_read_to_string_is_match!(readable, matchable)`\n readable.read_to_string error: `{:?}`", err),
        };
        if $matchable.is_match(readable_buffer.as_str()) {
            ()
        } else {
            panic!("assertion failed: `assert_read_to_string_is_match!(readable, matchable)`\n readable: `{:?}`,\n matchable: `{:?}`", readable_buffer, $matchable);
        }
    });
    ($readable:expr, $matchable:expr, $($arg:tt)+) => ({
        let mut readable_buffer = String::new();
        let _readable_size = match $readable.read_to_string(&mut readable_buffer) {
            Ok(size) => size,
            Err(_err) => panic!("{:?}", $($arg)+)
        };
        if $matchable.is_match(readable_buffer.as_str()) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use regex::Regex;

    #[test]
    fn test_assert_read_to_string_is_match_x_arity_2_success() {
        let mut readable = "alpha".as_bytes();
        let matchable = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_is_match!(readable, matchable);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_is_match!(readable, matchable)`\n readable: `\"alpha\"`,\n matchable: `xyz`")]
    fn test_assert_read_to_string_is_match_x_arity_2_failure() {
        let mut readable = "alpha".as_bytes();
        let matchable = Regex::new(r"xyz").unwrap();
        let _x = assert_read_to_string_is_match!(readable, matchable);
    }

    #[test]
    fn test_assert_read_to_string_is_match_x_arity_3_success() {
        let mut readable = "alpha".as_bytes();
        let matchable = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_is_match!(readable, matchable, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_is_match_x_arity_3_failure() {
        let mut readable = "alpha".as_bytes();
        let matchable = Regex::new(r"xyz").unwrap();
        let _x = assert_read_to_string_is_match!(readable, matchable, "message");
    }

}
