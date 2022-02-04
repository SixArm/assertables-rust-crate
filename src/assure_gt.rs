/// Assure one value is greater than anoter.
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
/// # fn main() {
/// let x = assure_gt!(2, 1);
/// //-> Ok(true)
///
/// let x = assure_gt!(1, 2);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_gt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val > right_val {
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
                if left_val > right_val {
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
    fn test_assure_gt_x_arity_2_success() {
        let a = 2;
        let b = 1;
        let x = assure_gt!(a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_gt_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assure_gt!(a, b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_gt_x_arity_3_success() {
        let a = 2;
        let b = 1;
        let x = assure_gt!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_gt_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assure_gt!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
