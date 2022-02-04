/// Assure a condition is true.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure!(true);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure!(false);
/// assert_eq!(x.unwrap(), false);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, String>);
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_x_arity_2_success() {
        let a = true;
        let x = assure!(a);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_x_arity_2_failure() {
        let a = false;
        let x = assure!(a);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_x_arity_3_success() {
        let a = true;
        let x = assure!(a, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_x_arity_3_failure() {
        let a = false;
        let x = assure!(a, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
