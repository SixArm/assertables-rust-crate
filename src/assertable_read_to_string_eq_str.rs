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
/// let mut a = "a".as_bytes();
/// let str = "a";
/// let x = assertable_read_to_string_eq_str!(a, str);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = "a".as_bytes();
/// let str = "b";
/// let x = assertable_read_to_string_eq_str!(a, str);
/// //-> Err("…")
/// // assertable failed: `assertable_read_to_string_eq_str!(left, str)`
/// //  left: `\"a\"`,
/// //   str: `\"b\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_read_to_string_eq_str!(left, str)`\n left: `\"a\"`,\n  str: `\"b\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_read_to_string_eq_str {
    ($left:expr, $str:expr $(,)?) => ({
        let mut left_buffer = String::new();
        let left_result = $left.read_to_string(&mut left_buffer);
        if let Err(err) = left_result {
            Err(format!("assertable failed: `assertable_read_to_string_eq_str!(left, str)`\n  left read_to_string error: `{:?}`", err))
        } else {
            let _left_size = left_result.unwrap();
            if (left_buffer == $str) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_read_to_string_eq_str!(left, str)`\n left: `{:?}`,\n  str: `{:?}`", left_buffer, $str))
            }
        }
    });
    ($left:expr, $str:expr, $($arg:tt)+) => ({
        let mut left_buffer = String::new();
        let left_result = $left.read_to_string(&mut left_buffer);
        if let Err(err) = left_result {
            Err(format!("assertable failed: `assertable_read_to_string_eq_str!(left, str)`\n  left read_to_string error: `{:?}`", err))
        } else {
            let _left_size = left_result.unwrap();
            if (left_buffer == $str) {
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
    fn test_assertable_read_to_string_eq_str_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let str = "a";
        let x = assertable_read_to_string_eq_str!(a, str);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_str_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let str = "b";
        let x = assertable_read_to_string_eq_str!(a, str);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_read_to_string_eq_str!(left, str)`\n left: `\"a\"`,\n  str: `\"b\"`"
        );
    }

    #[test]
    fn test_assertable_assertable_read_to_string_eq_str_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let str = "a";
        let x = assertable_read_to_string_eq_str!(a, str, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_assertable_read_to_string_eq_str_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let str = "b";
        let x = assertable_read_to_string_eq_str!(a, str, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
