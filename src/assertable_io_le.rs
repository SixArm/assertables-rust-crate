/// Assert one value is less than or equal to another value.
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
/// # use std::panic;
/// # fn main() {
/// let x = assertable_io_le!(1, 2);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assertable_io_le!(2, 1);
/// //-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "…")
/// // assertable failed: `assertable_io_le!(left, right)`
/// //   left: `2`,
/// //  right: `1`
/// assert_eq!(
///     x.unwrap_err().get_ref().unwrap().to_string(),
///     "assertable failed: `assertable_io_le!(left, right)`\n  left: `2`,\n right: `1`"
/// );
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_io_le {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
                    Ok(())
                } else {
                    Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("assertable failed: `assertable_io_le!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                        )
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
                    Ok(())
                } else {
                    Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            $($arg)+
                        )
                    )
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_io_le_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assertable_io_le!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_io_le_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        let x = assertable_io_le!(a, b);
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assertable failed: `assertable_io_le!(left, right)`\n  left: `2`,\n right: `1`"
        );
    }

    #[test]
    fn test_assertable_io_le_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assertable_io_le!(a, b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_io_le_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let x = assertable_io_le!(a, b, "message");
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
