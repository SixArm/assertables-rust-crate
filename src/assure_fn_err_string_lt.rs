/// Assure one function ok() is less than to another function ok().
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// let x: Result<bool, String> = assure_fn_err_string_lt!(f, 1, 2);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// let x: Result<bool, String> = assure_fn_err_string_lt!(f, 2, 1);
/// //-> Ok(false)
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
            Ok(false)
        } else {
            let left = left.unwrap_err().to_string();
            let right = right.unwrap_err().to_string();
            if (left < right) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Ok(false)
        } else {
            let left = left.unwrap_err().to_string();
            let right = right.unwrap_err().to_string();
            if (left < right) {
                Ok(true)
            } else {
                Ok(false)
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
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_2_eq_failure() {
        let a = 1;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_2_gt_failure() {
        let a = 2;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_eq_failure() {
        let a = 1;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_err_string_lt_x_arity_3_gt_failure() {
        let a = 2;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_lt!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
