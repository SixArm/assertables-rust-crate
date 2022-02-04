/// Assure one function output is greater than anoter.
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
/// let x: Result<bool, &str> = assure_fn_gt!(i32::abs, -2 as i32, 1 as i32);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x: Result<bool, &str> = assure_fn_gt!(i32::abs, 1 as i32, -2 as i32);
/// assert_eq!(x.unwrap(), false);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_gt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left > right) {
            Ok(true)
        } else {
            Ok(false)
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left > right) {
            Ok(true)
        } else {
            Ok(false)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_fn_gt_x_arity_2_gt_success() {
        let a = -2;
        let b = 1;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_gt_x_arity_2_eq_failure() {
        let a = 1;
        let b = -1;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_gt_x_arity_2_lt_failure() {
        let a = 1;
        let b = -2;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_gt_x_arity_3_gt_success() {
        let a = -2;
        let b = 1;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_gt_x_arity_3_eq_failure() {
        let a = 1;
        let b = -1;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_gt_x_arity_3_lt_failure() {
        let a = 1;
        let b = -2;
        let x: Result<bool, &str> = assure_fn_gt!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
