/// Assure two sets are equal.
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
/// let x = assure_set_eq!([1, 2], [2, 1]);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_set_eq!([1, 2], [3, 4]);
/// assert_eq!(x.unwrap(), false);
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
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_set_eq_x_arity_2_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_eq!(&a, &b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_set_eq_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_eq!(&a, &b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_set_eq_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            true
        )
    }

    #[test]
    fn test_assure_set_eq_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_eq!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            false
        )
    }

}
