/// Assume a condition is true.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assume_io!(true);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assume_io!(false); //-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "…");
/// assert_eq!(
///     x.unwrap_err().get_ref().unwrap().to_string(),
///     "assumption failed: `assume_io(condition) condition: `false`"
/// );
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_io {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok(true)
        } else {
            Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("assumption failed: `assume_io(condition) condition: `{:?}`", $x)
                )
            )
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok(true)
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
    fn test_assume_io_x_arity_2_success() {
        let a = true;
        let x = assume_io!(a);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_io_x_arity_2_failure() {
        let a = false;
        let x = assume_io!(a);
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assumption failed: `assume_io(condition) condition: `false`"
        );
    }

    #[test]
    fn test_assume_io_x_arity_3_success() {
        let a = true;
        let x = assume_io!(a, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_io_x_arity_3_failure() {
        let a = false;
        let x = assume_io!(a, "message");
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
