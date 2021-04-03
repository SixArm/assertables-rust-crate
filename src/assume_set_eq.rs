/// Assume two sets are equal.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let a = [1, 2];
/// let b = [2, 1];
/// assume_set_eq!(&a, &b);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertable; fn main() {
/// let a = [1, 2];
/// let b = [3, 4];
/// assume_set_eq!(&a, &b);
/// //-> Err("assume_set_eq left:[1, 2] right:[3, 4]")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assume_set_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
                    Ok(true)
                } else {
                    Err(format!("assumption failed: `assume_set_eq(left, right)`\n  left: `{:?}`\n right: `{:?}`", $left, $right))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
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
    fn test_assume_set_eq_x_arity_2_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assume_set_eq!(&a, &b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_set_eq_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assume_set_eq!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_set_eq(left, right)`\n  left: `[1, 2]`\n right: `[3, 4]`"
        );
    }

    #[test]
    fn test_assume_set_eq_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assume_set_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            true
        )
    }

    #[test]
    fn test_assume_set_eq_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assume_set_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        )
    }

}
