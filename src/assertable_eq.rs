/// Assert a values is equal to another.
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
/// let x = assertable_eq!(1, 1);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assertable_eq!(1, 2);
/// //-> Err("…")
/// // assertable failed: `assertable_eq!(left, right)`
/// //   left: `1`,
/// //  right: `2`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_eq!(left, right)`\n  left: `1`,\n right: `2`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_eq {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if (a_val == b_val) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                if (a_val == b_val) {
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
    fn test_assertable_eq_x_arity_2_success() {
        let a = 1;
        let b = 1;
        let x = assertable_eq!(a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_eq_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x =  assertable_eq!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_eq!(left, right)`\n  left: `1`,\n right: `2`"
        );
    }

    #[test]
    fn test_assertable_eq_x_arity_3_success() {
        let a = 1;
        let b = 1;
        let x = assertable_eq!(a, b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_eq_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assertable_eq!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
