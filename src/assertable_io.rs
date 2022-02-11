/// Assert a condition is true.
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
/// let x = assertable_io!(true);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assertable_io!(false);
/// //-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "…");
/// // assertable failed: `assertable_io(condition) condition: `false`
/// assert_eq!(
///     x.unwrap_err().get_ref().unwrap().to_string(),
///     "assertable failed: `assertable_io(condition) condition: `false`"
/// );
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_io {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok(())
        } else {
            Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("assertable failed: `assertable_io(condition) condition: `{:?}`", $x)
                )
            )
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok(())
        } else {
            Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    $($arg)+
                )
            )
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_io_x_arity_2_success() {
        let a = true;
        let x = assertable_io!(a);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_io_x_arity_2_failure() {
        let a = false;
        let x = assertable_io!(a);
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assertable failed: `assertable_io(condition) condition: `false`"
        );
    }

    #[test]
    fn test_assertable_io_x_arity_3_success() {
        let a = true;
        let x = assertable_io!(a, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_io_x_arity_3_failure() {
        let a = false;
        let x = assertable_io!(a, "message");
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
