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
/// let x = assertable!(true);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable!(false);
/// //-> Err("…")
/// // assertable failed: `assertable!(condition)`
/// //  condition: `false`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable!(condition)`\n condition: `false`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable {
    ($x:expr $(,)?) => ({
        if $x {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable!(condition)`\n condition: `{:?}`", $x))
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
    fn test_assertable_x_arity_2_success() {
        let a = true;
        let x = assertable!(a);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_x_arity_2_failure() {
        let a = false;
        let x = assertable!(a);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable!(condition)`\n condition: `false`"
        );
    }

    #[test]
    fn test_assertable_x_arity_3_success() {
        let a = true;
        let x = assertable!(a, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_x_arity_3_failure() {
        let a = false;
        let x = assertable!(a, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
