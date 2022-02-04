/// Assume one value is greater than or equal to another value.
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
/// let x = assume_ge!(2, 1);
/// //-> Ok(true)
///
/// let x = assume_ge!(1, 2);
/// //-> Err("assumption failed: `assume_ge!(left, right)`\n  left: `1`,\n right: `2`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_ge {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok(true)
                } else {
                    Err(format!("assumption failed: `assume_ge!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok(true)
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
    fn test_assume_ge_x_arity_2_success() {
        let a = 2;
        let b = 1;
        let x = assume_ge!(a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_ge_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assume_ge!(a, b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_ge!(left, right)`\n  left: `1`,\n right: `2`"
        );
    }

    #[test]
    fn test_assume_ge_x_arity_3_success() {
        let a = 2;
        let b = 1;
        let x = assume_ge!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_ge_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assume_ge!(a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
