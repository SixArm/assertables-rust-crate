/// Assure one value is equal to another value.
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
/// # fn main() {
/// let x = assure_io_eq!(1, 1);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_io_eq!(1, 2); //-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "…");
/// assert!(x.is_err());
/// assert_eq!(
///     x.unwrap_err().get_ref().unwrap().to_string(),
///     "assurance failed: `assure_io_eq!(left, right)`\n  left: `1`,\n right: `2`"
/// );
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_io_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val == right_val) {
                    Ok(())
                } else {
                    Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("assurance failed: `assure_io_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
                        )
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val == right_val) {
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
    fn test_assure_io_eq_x_arity_2_success() {
        let a = 1;
        let b = 1;
        let x = assure_io_eq!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_io_eq_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assure_io_eq!(a, b);
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assurance failed: `assure_io_eq!(left, right)`\n  left: `1`,\n right: `2`"
        );
    }

    #[test]
    fn test_assure_io_eq_x_arity_3_success() {
        let a = 1;
        let b = 1;
        let x = assure_io_eq!(a, b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_io_eq_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assure_io_eq!(a, b, "message");
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
