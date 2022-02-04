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
/// # use std::panic;
/// # fn main() {
/// let x = assume!(true);
/// //-> Ok(true)
///
/// let x = assume!(false);
/// //-> Err("assumption failed: `false`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume {
    ($x:expr $(,)?) => ({
        if $x {
            Ok(true)
        } else {
            Err(format!("assumption failed: `{:?}`", $x))
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if $x {
            Ok(true)
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assume_x_arity_2_success() {
        let a = true;
        let x = assume!(a);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_x_arity_2_failure() {
        let a = false;
        let x = assume!(a);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `false`"
        );
    }

    #[test]
    fn test_assume_x_arity_3_success() {
        let a = true;
        let x = assume!(a, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_x_arity_3_failure() {
        let a = false;
        let x = assume!(a, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
