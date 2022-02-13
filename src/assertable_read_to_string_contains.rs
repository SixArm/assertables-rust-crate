/// Assert a std::io::Read read_to_string() contains a pattern.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # #[allow(unused_imports)]
/// use std::io::Read;
/// # fn main() {
/// let mut readable = "hello".as_bytes();
/// let pattern = "hello";
/// let x = assertable_read_to_string_contains!(readable, pattern);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut readable = "hello".as_bytes();
/// let pattern = "world";
/// let x = assertable_read_to_string_contains!(readable, pattern);
/// //-> Err("…")
/// // assertable failed: `assertable_read_to_string_contains!(readable, pattern)`
/// //  readable: `\"hello\"`,
/// //   pattern: `\"world\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_read_to_string_contains!(readable, pattern)`\n readable: `\"hello\"`,\n  pattern: `\"world\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_read_to_string_contains {
    ($readable:expr, $pattern:expr $(,)?) => ({
        let mut readable_buffer = String::new();
        let readable_result = $readable.read_to_string(&mut readable_buffer);
        if let Err(err) = readable_result {
            Err(format!("assertable failed: `assertable_read_to_string_contains!(readable, pattern)`\n readable.read_to_string error: `{:?}`", err))
        } else {
            let _readable_size = readable_result.unwrap();
            if readable_buffer.contains($pattern) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_read_to_string_contains!(readable, pattern)`\n readable: `{:?}`,\n  pattern: `{:?}`", readable_buffer, $pattern))
            }
        }
    });
    ($readable:expr, $pattern:expr, $($arg:tt)+) => ({
        let mut readable_buffer = String::new();
        let readable_result = $readable.read_to_string(&mut readable_buffer);
        if let Err(_err) = readable_result {
            Err(format!("{}", $($arg)+))
        } else {
            let _readable_size = readable_result.unwrap();
            if readable_buffer.contains($pattern) {
                Ok(())
            } else {
                Err(format!("{}", $($arg)+))
            }
        }
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assertable_read_to_string_contains_x_arity_2_success() {
        let mut readable = "a".as_bytes();
        let pattern = "a";
        let x = assertable_read_to_string_contains!(readable, pattern);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_contains_x_arity_2_failure() {
        let mut readable = "a".as_bytes();
        let pattern = "b";
        let x = assertable_read_to_string_contains!(readable, pattern);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_read_to_string_contains!(readable, pattern)`\n readable: `\"a\"`,\n  pattern: `\"b\"`"
        );
    }

    #[test]
    fn test_assertable_read_to_string_contains_x_arity_3_success() {
        let mut readable = "a".as_bytes();
        let pattern = "a";
        let x = assertable_read_to_string_contains!(readable, pattern, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_contains_x_arity_3_failure() {
        let mut readable = "a".as_bytes();
        let pattern = "b";
        let x = assertable_read_to_string_contains!(readable, pattern, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
