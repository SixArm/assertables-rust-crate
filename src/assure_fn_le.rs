/// Assure one function output is less than or equal to another function output.
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
/// let x = assure_fn_le!(i32::abs, 1 as i32, -2 as i32);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_fn_le!(i32::abs, -2 as i32, 1 as i32);
/// assert!(x.is_err());
/// assert_eq!(x.unwrap_err(), "assurance failed: `assure_fn_le!(fn, left, right)`\n  left input: `-2`,\n right input: `1`,\n  left output: `2`,\n right output: `1`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_le {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left <= right) {
            Ok(())
        } else {
            Err(format!("assurance failed: `assure_fn_le!(fn, left, right)`\n  left input: `{:?}`,\n right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left <= right) {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_fn_le_x_arity_2_lt_success() {
        let a = 1;
        let b = -2;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_le_x_arity_2_eq_success() {
        let a = 1;
        let b = -1;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_le_x_arity_2_gt_failure() {
        let a = -2;
        let b = 1;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `assure_fn_le!(fn, left, right)`\n  left input: `-2`,\n right input: `1`,\n  left output: `2`,\n right output: `1`"
        );
    }

    #[test]
    fn test_assure_fn_le_x_arity_3_lt_success() {
        let a = 1;
        let b = -2;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_le_x_arity_3_eq_success() {
        let a = 1;
        let b = -1;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_le_x_arity_3_gt_failure() {
        let a = -2;
        let b = 1;
        let x = assure_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
