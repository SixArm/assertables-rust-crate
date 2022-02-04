/// Assure one function ok() is less than or equal to another function ok().
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// 
/// # fn main() {
/// let x: Result<bool, String> = assure_fn_err_string_le!(f, 1, 2);
/// //-> Ok(true)
///
/// let x: Result<bool, String> = assure_fn_err_string_le!(f, 2, 1);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_err_string_le {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Ok(false)
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left <= right) {
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
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left <= right) {
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
    fn test_assure_fn_err_string_le_x_arity_2_lt_success() {
        let a = 1;
        let b = 2;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_le_x_arity_2_eq_success() {
        let a = 1;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_le_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_err_string_le_x_arity_3_lt_success() {
        let a = 1;
        let b = 2;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_le_x_arity_3_eq_success() {
        let a = 1;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_err_string_le_x_arity_3_gt_failure() {
        let a = 2;
        let b = 1;
        let x: Result<bool, String> = assure_fn_err_string_le!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
