/// Assert a set is joint with another.
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
/// let a = [1, 2];
/// let b = [2, 3];
/// let x = assertable_set_joint!(&a, &b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let a = [1, 2];
/// let b = [3, 4];
/// let x = assertable_set_joint!(&a, &b);
/// //-> Err("…")
/// // assertable failed: `assertable_set_joint!(left, right)`
/// //   left: `[1, 2]`,
/// //  right: `[3, 4]`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_set_joint!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assertable_set_joint {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if !left_set.is_disjoint(&right_set) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_set_joint!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if !left_set.is_disjoint(&right_set) {
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
    fn test_assertable_set_joint_x_arity_2_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assertable_set_joint!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_set_joint_x_arity_2_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assertable_set_joint!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_set_joint!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`"
        );
    }

    #[test]
    fn test_assertable_set_joint_x_arity_3_success() {
        let a = [1, 2];
        let b = [2, 3];
        let x = assertable_set_joint!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_set_joint_x_arity_3_failure() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assertable_set_joint!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        )
    }

}
