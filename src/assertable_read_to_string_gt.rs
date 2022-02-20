/// Assert one std::io::Read read_to_string() value is greater than another.
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
/// let mut b = "b".as_bytes();
/// let x = assertable_read_to_string_gt!(b, a);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = "a".as_bytes();
/// let mut b = "b".as_bytes();
/// let x = assertable_read_to_string_gt!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_read_to_string_gt!(left, right)`
/// //   left: `\"a\"`,
/// //  right: `\"b\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_read_to_string_gt!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_read_to_string_gt {
    ($a:expr, $b:expr $(,)?) => ({
        let mut a_buffer = String::new();
        let mut b_buffer = String::new();
        let a_result = $a.read_to_string(&mut a_buffer);
        let b_result = $b.read_to_string(&mut b_buffer);
        if let Err(err) = a_result {
            Err(format!("assertable failed: `assertable_read_to_string_gt!(left, right)`\n  left read_to_string error: `{:?}`", err))
        } else {
            if let Err(err) = b_result {
                Err(format!("assertable failed: `assertable_read_to_string_gt!(left, right)`\n right read_to_string error: `{:?}`", err))
            } else {
                let _a_size = a_result.unwrap();
                let _b_size = b_result.unwrap();
                if (a_buffer > b_buffer) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_read_to_string_gt!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", a_buffer, b_buffer))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        let mut a_buffer = String::new();
        let mut b_buffer = String::new();
        let a_result = $a.read_to_string(&mut a_buffer);
        let b_result = $b.read_to_string(&mut b_buffer);
        if let Err(err) = a_result {
            Err(format!("assertable failed: `assertable_read_to_string_gt!(left, right)`\n  left read_to_string error: `{:?}`", err))
        } else {
            if let Err(err) = b_result {
                Err(format!("assertable failed: `assertable_read_to_string_gt!(left, right)`\n right read_to_string error: `{:?}`", err))
            } else {
                let _a_size = a_result.unwrap();
                let _b_size = b_result.unwrap();
                if (a_buffer > b_buffer) {
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
    fn test_assertable_read_to_string_gt_x_arity_2_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_gt!(b, a);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_gt_x_arity_2_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_gt!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_read_to_string_gt!(left, right)`\n  left: `\"a\"`,\n right: `\"b\"`"
        );
    }

    #[test]
    fn test_assertable_read_to_string_gt_x_arity_3_success() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_gt!(b, a, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_read_to_string_gt_x_arity_3_failure() {
        let mut a = "a".as_bytes();
        let mut b = "b".as_bytes();
        let x = assertable_read_to_string_gt!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
