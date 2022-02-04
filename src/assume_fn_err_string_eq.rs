/// Assume one function ok() is equal to another function ok().
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// assume_fn_err_string_eq!(f, 1, 1);
/// //-> Ok(true)
///
/// fn f(i: i32) -> Result<bool, String> { Err(format!("{:?}", i)) }
/// assume_fn_err_string_eq!(f, 1, 2);
/// //-> Err("assumption failed: `assume_fn_err_string_eq!(left, right)`\n  left input: `1`,\n right input: `2`,\n  left output: `\"1\"`,\n right output: `\"2\"`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_fn_err_string_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Err(format!("assumption failed: `assume_fn_err_string_eq!(fn, left, right)`\n  left input: `{:?}`,\n right input: `{:?}`\n  left output is_err(): `{:?}`,\n right output is_err(): `{:?}`", $left, $right, left.is_err(), right.is_err()))
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left == right) {
                Ok(true)
            } else {
                Err(format!("assumption failed: `assume_fn_err_string_eq!(fn, left, right)`\n  left input: `{:?}`,\n right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right))
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
            if (left == right) {
                Ok(true)
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
    fn test_assume_fn_err_string_eq_x_arity_2_success() {
        let a = 1;
        let b = 1;
        let x = assume_fn_err_string_eq!(f, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_err_string_eq_x_arity_2_failure() {
        let a = 1;
        let b = 2;
        let x = assume_fn_err_string_eq!(f, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_fn_err_string_eq!(fn, left, right)`\n  left input: `1`,\n right input: `2`,\n  left output: `\"1\"`,\n right output: `\"2\"`"
        );
    }

    #[test]
    fn test_assume_fn_err_string_eq_x_arity_3_success() {
        let a = 1;
        let b = 1;
        let x = assume_fn_err_string_eq!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_err_string_eq_x_arity_3_failure() {
        let a = 1;
        let b = 2;
        let x = assume_fn_err_string_eq!(f, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
