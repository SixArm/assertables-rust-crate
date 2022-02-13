/// Assert a std::io::Read read_to_string() is equal to another.
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
/// let string = "hello";
/// let x = assertable_read_to_string_eq_string!(readable, string);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut readable = "hello".as_bytes();
/// let string = "world";
/// let x = assertable_read_to_string_eq_string!(readable, string);
/// //-> Err("…")
/// // assertable failed: `assertable_read_to_string_eq_string!(readable, string)`
/// //  readable: `\"hello\"`,
/// //    string: `\"world\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_read_to_string_eq_string!(readable, string)`\n readable: `\"hello\"`,\n   string: `\"world\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_read_to_string_eq_string {
    ($readable:expr, $string:expr $(,)?) => ({
        let mut readable_buffer = String::new();
        let readable_result = $readable.read_to_string(&mut readable_buffer);
        if let Err(err) = readable_result {
            Err(format!("assertable failed: `assertable_read_to_string_eq_string!(readable, string)`\n read_to_string error: `{:?}`", err))
        } else {
            let _readable_size = readable_result.unwrap();
            let expect = String::from($string);
            if (readable_buffer == expect) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_read_to_string_eq_string!(readable, string)`\n readable: `{:?}`,\n   string: `{:?}`", readable_buffer, $string))
            }
        }
    });
    ($readable:expr, $string:expr, $($arg:tt)+) => ({
        let mut readable_buffer = String::new();
        let readable_result = $readable.read_to_string(&mut readable_buffer);
        if let Err(_err) = readable_result {
            Err(format!("{}", $($arg)+))
        } else {
            let _readable_size = readable_result.unwrap();
            let expect = String::from($string);
            if (readable_buffer == expect) {
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
    fn test_assertable_read_to_string_eq_string_x_arity_2_success() {
        let mut readable = "a".as_bytes();
        let string = "a";
        let x = assertable_read_to_string_eq_string!(readable, string);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_string_x_arity_2_failure() {
        let mut readable = "a".as_bytes();
        let string = "b";
        let x = assertable_read_to_string_eq_string!(readable, string);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_read_to_string_eq_string!(readable, string)`\n readable: `\"a\"`,\n   string: `\"b\"`"
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_string_x_arity_3_success() {
        let mut readable = "a".as_bytes();
        let string = "a";
        let x = assertable_read_to_string_eq_string!(readable, string, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_string_x_arity_3_failure() {
        let mut readable = "a".as_bytes();
        let string = "b";
        let x = assertable_read_to_string_eq_string!(readable, string, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
