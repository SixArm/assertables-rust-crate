/// Assure one function ok() is equal to another function ok().
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::str::FromStr;
/// # fn main() {
/// let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, "1", "1");
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::str::FromStr;
/// # fn main() {
/// let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, "1", "2");
/// assert_eq!(x.unwrap(), false);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_fn_ok_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Ok(false)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left == right) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Ok(false)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left == right) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_assure_fn_ok_eq_x_arity_2_success() {
        let a = "1";
        let b = "1";
        let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_ok_eq_x_arity_2_failure() {
        let a = "1";
        let b = "2";
        let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, a, b);
        assert_eq!(
            x.unwrap(),
            false
        );
    }

    #[test]
    fn test_assure_fn_ok_eq_x_arity_3_success() {
        let a = "1";
        let b = "1";
        let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_fn_ok_eq_x_arity_3_failure() {
        let a = "1";
        let b = "2";
        let x: Result<bool, &str> = assure_fn_ok_eq!(i32::from_str, a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
        );
    }

}
