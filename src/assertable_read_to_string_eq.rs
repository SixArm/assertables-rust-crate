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
/// let mut b = "a".as_bytes();
/// let x = assertable_read_to_string_eq!(a, b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// let x = assertable_read_to_string_eq!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_read_to_string_eq!(left, right)`
/// //   left: `\"a\"`,
/// //  right: `\"b\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_read_to_string_eq!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_read_to_string_eq {
    ($left:expr, $right:expr $(,)?) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let left_result = $left.read_to_string(&mut left_buffer);
        let right_result = $right.read_to_string(&mut right_buffer);
        if let Err(err) = left_result {
            Err(format!("assertable failed: `assertable_read_to_string_eq!(left, right)`\n  left read_to_string error: `{:?}`", err))
        } else {
            if let Err(err) = right_result {
                Err(format!("assertable failed: `assertable_read_to_string_eq!(left, right)`\n right read_to_string error: `{:?}`", err))
            } else {
                let _left_size = left_result.unwrap();
                let _right_size = right_result.unwrap();
                if (left_buffer == right_buffer) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_read_to_string_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", left_buffer, right_buffer))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let left_result = $left.read_to_string(&mut left_buffer);
        let right_result = $right.read_to_string(&mut right_buffer);
        if let Err(err) = left_result {
            Err(format!("assertable failed: `assertable_read_to_string_eq!(left, right)`\n  left read_to_string error: `{:?}`", err))
        } else {
            if let Err(err) = right_result {
                Err(format!("assertable failed: `assertable_read_to_string_eq!(left, right)`\n right read_to_string error: `{:?}`", err))
            } else {
                let _left_size = left_result.unwrap();
                let _right_size = right_result.unwrap();
                if (left_buffer == right_buffer) {
                    Ok(())
                } else {
                    Err(format!("{}", $($arg)+))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assertable_read_to_string_eq_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let mut b = "a".as_bytes();
        let x = assertable_read_to_string_eq!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_eq!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_read_to_string_eq!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`"
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let mut b = "a".as_bytes();
        let x = assertable_read_to_string_eq!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_eq_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_eq!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
