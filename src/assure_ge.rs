/// Assure one value is greater than or equal to another value.
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
/// let x = assure_ge!(2, 1);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_ge!(1, 2);
/// assert!(x.is_err());
/// assert_eq!(x.unwrap_err(), "assurance failed: `assure_ge!(left, right)`\n  left: `1`,\n right: `2`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_ge {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok(())
                } else {
                    Err(format!("assurance failed: `assure_ge!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok(())
                } else {
                    Err($($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_ge_x_arity_2_success() {
        let a = 2;
        let b = 1;
        let x = assure_ge!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_ge_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assure_ge!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `assure_ge!(left, right)`\n  left: `1`,\n right: `2`"
        );
    }

    #[test]
    fn test_assure_ge_x_arity_3_success() {
        let a = 2;
        let b = 1;
        let x = assure_ge!(a, b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_ge_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assure_ge!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
