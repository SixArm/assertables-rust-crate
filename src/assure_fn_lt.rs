/// Assure one function output is less than to another function output.
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
/// let x: Result<bool, &str> = assure_fn_lt!(i32::abs, 1 as i32, -2 as i32);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// let x: Result<bool, &str> = assure_fn_lt!(i32::abs, -2 as i32, 1 as i32);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_lt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left < right) {
            Ok(true)
        } else {
            Ok(false)
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left < right) {
            Ok(true)
        } else {
            Ok(false)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_fn_lt_x_arity_2_lt_success() {
        let a = 1;
        let b = -2;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_lt_x_arity_2_eq_failure() {
        let a = 1;
        let b = -1;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_lt_x_arity_2_gt_failure() {
        let a = -2;
        let b = 1;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_lt_x_arity_3_lt_success() {
        let a = 1;
        let b = -2;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_lt_x_arity_3_eq_failure() {
        let a = 1;
        let b = -1;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_lt_x_arity_3_gt_failure() {
        let a = -2;
        let b = 1;
        let x: Result<bool, &str> = assure_fn_lt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
