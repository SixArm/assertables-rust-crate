/// Assert a set is a subset of another.
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
/// let b = [1, 2, 3];
/// let x = assertable_set_subset!(&a, &b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let a = [1, 2, 3];
/// let b = [1, 2];
/// let x = assertable_set_subset!(&a, &b);
/// //-> Err("…")
/// // assertable failed: `assertable_set_subset!(left, right)`
/// //   left: `[1, 2, 3]`,
/// //  right: `[1, 2]`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_set_subset!(left, right)`\n  left: `[1, 2, 3]`,\n right: `[1, 2]`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
///
/// This implementation uses [`HashSet`] to count items.
#[macro_export]
macro_rules! assertable_set_subset {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::HashSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::HashSet<_> = b_val.into_iter().collect();
                if a_set.is_subset(&b_set) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_set_subset!(left, right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b))
                }
            }
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match (&($a), &($b)) {
            (a_val, b_val) => {
                let a_set: ::std::collections::HashSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::HashSet<_> = b_val.into_iter().collect();
                if a_set.is_subset(&b_set) {
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
    fn test_assertable_set_subset_x_arity_2_success() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let x = assertable_set_subset!(&a, &b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_set_subset_x_arity_2_failure() {
        let a = [1, 2, 3];
        let b = [1, 2];
        let x = assertable_set_subset!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_set_subset!(left, right)`\n  left: `[1, 2, 3]`,\n right: `[1, 2]`"
        );
    }

    #[test]
    fn test_assertable_set_subset_x_arity_3_success() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let x = assertable_set_subset!(&a, &b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_set_subset_x_arity_3_failure() {
        let a = [1, 2, 3];
        let b = [3, 2];
        let x = assertable_set_subset!(&a, &b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        )
    }

}
