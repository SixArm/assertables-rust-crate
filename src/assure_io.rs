/// Assure a condition is true.
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
/// let x = assure_io!(true);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_io!(false); //-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "…");
/// assert!(x.is_err());
/// assert_eq!(
///     x.unwrap_err().get_ref().unwrap().to_string(),
///     "assurance failed: `assure_io(condition) condition: `false`"
/// );
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_io {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok(())
        } else {
            Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("assurance failed: `assure_io(condition) condition: `{:?}`", $x)
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
    fn test_assure_io_x_arity_2_success() {
        let a = true;
        let x = assure_io!(a);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_io_x_arity_2_failure() {
        let a = false;
        let x = assure_io!(a);
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assurance failed: `assure_io(condition) condition: `false`"
        );
    }

    #[test]
    fn test_assure_io_x_arity_3_success() {
        let a = true;
        let x = assure_io!(a, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_io_x_arity_3_failure() {
        let a = false;
        let x = assure_io!(a, "message");
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
