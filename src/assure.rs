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
/// let x = assure!(true);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure!(false);
/// assert!(x.is_err());
/// assert_eq!(x.unwrap_err(), "assurance failed: `false`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure {
    ($x:expr $(,)?) => ({
        if $x {
            Ok(())
        } else {
            Err(format!("assurance failed: `{:?}`", $x))
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if $x {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_x_arity_2_success() {
        let a = true;
        let x = assure!(a);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_x_arity_2_failure() {
        let a = false;
        let x = assure!(a);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `false`"
        );
    }

    #[test]
    fn test_assure_x_arity_3_success() {
        let a = true;
        let x = assure!(a, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_x_arity_3_failure() {
        let a = false;
        let x = assure!(a, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
