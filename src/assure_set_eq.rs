/// Assure a set is equal to another.
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
/// let a = [1, 2];
/// let b = [2, 1];
/// let x = assure_set_eq!(&a, &b);
/// assert!(x.is_ok());
///
/// let a = [1, 2];
/// let b = [3, 4];
/// let x = assure_set_eq!(&a, &b);
/// assert!(x.is_err());
/// assert_eq!(x.unwrap_err(), "assurance failed: `assure_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assure_set_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
                    Ok(())
                } else {
                    Err(format!("assurance failed: `assure_set_eq!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right))
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
    fn test_assure_set_eq_x_arity_2_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_eq!(&a, &b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_set_eq_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_eq!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `assure_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`"
        );
    }

    #[test]
    fn test_assure_set_eq_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_eq!(&a, &b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_set_eq_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        )
    }

}
