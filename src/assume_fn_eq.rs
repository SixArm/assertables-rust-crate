/// Assume one function output is greater than another function output.
///
/// * When true, return `Ok(true)`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// assume_fn_eq!(i32::abs, 1 as i32, -1 as i32);
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// assume_fn_eq!(i32::abs, 1 as i32, -2 as i32);
/// //-> Err("assumption failed: `assume_fn_eq(left, right)`\n  left input: `1`\n right input: `2`\n  left output: `1`\n right output: `2`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_fn_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left == right) {
            Ok(true)
        } else {
            Err(format!("assumption failed: `assume_fn_eq(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output: `{:?}`\n right output: `{:?}`", $left, $right, left, right))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left == right) {
            Ok(true)
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assume_fn_eq_x_arity_2_success() {
        let a = 1;
        let b = -1;
        let x = assume_fn_eq!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_eq_x_arity_2_failure() {
        let a = 1;
        let b = -2;
        let x = assume_fn_eq!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_fn_eq(fn, left, right)`\n  left input: `1`\n right input: `-2`\n  left output: `1`\n right output: `2`"
        );
    }

    #[test]
    fn test_assume_fn_eq_x_arity_3_success() {
        let a = 1;
        let b = -1;
        let x = assume_fn_eq!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_eq_x_arity_3_failure() {
        let a = 1;
        let b = -2;
        let x = assume_fn_eq!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
