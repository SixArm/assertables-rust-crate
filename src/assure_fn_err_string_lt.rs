/// Assure one function ok() is less than anoter.
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
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// # fn main() {
/// let x = assure_fn_err_string_lt!(f, 1, 2);
/// assert!(x.is_ok());
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// # fn main() {
/// let x = assure_fn_err_string_lt!(f, 2, 1);
/// assert!(x.is_err());
/// assert_eq!(x.unwrap_err(), "assurance failed: `assure_fn_err_string_lt!(fn, left, right)`\n  left input: `2`,\n right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_err_string_lt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Err(format!("assurance failed: `assure_fn_err_string_lt!(fn, left, right)`\n  left input: `{:?}`,\n right input: `{:?}`\n  left output is_err(): `{:?}`,\n right output is_err(): `{:?}`", $left, $right, left.is_err(), right.is_err()))
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left < right) {
                Ok(())
            } else {
                Err(format!("assurance failed: `assure_fn_err_string_lt!(fn, left, right)`\n  left input: `{:?}`,\n right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right))
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Err($($arg)+)
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left < right) {
                Ok(())
            } else {
                Err($($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_2_lt_success() {
        let a = 1;
        let b = 2;
        let x = assure_fn_err_string_lt!(f, a, b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_2_eq_failure() {
        let a = 1;
        let b = 1;
        let x = assure_fn_err_string_lt!(f, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `assure_fn_err_string_lt!(fn, left, right)`\n  left input: `1`,\n right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`"
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_2_gt_failure() {
        let a = 2;
        let b = 1;
        let x = assure_fn_err_string_lt!(f, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assurance failed: `assure_fn_err_string_lt!(fn, left, right)`\n  left input: `2`,\n right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`"
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x = assure_fn_err_string_lt!(f, a, b, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_eq_failure() {
        let a = 1;
        let b = 1;
        let x = assure_fn_err_string_lt!(f, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_gt_failure() {
        let a = 2;
        let b = 1;
        let x = assure_fn_err_string_lt!(f, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
