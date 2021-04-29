/// Assume one function ok() is equal to another function ok().
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
/// # use std::str::FromStr;
/// assume_fn_ok_eq!(i32::from_str, "1", "1");
/// //-> Ok(true)
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables; fn main() {
/// # use std::str::FromStr;
/// assume_fn_ok_eq!(i32::from_str, "1", "2");
/// //-> Err("assumption failed: `assume_fn_ok_eq(left, right)`\n  left input: `\"1\"`\n right input: `\"2\"`\n  left output: `1`\n right output: `2`")
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assume_fn_ok_eq {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Err(format!("assumption failed: `assume_fn_ok_eq(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output is_ok(): `{:?}`\n right output is_ok(): `{:?}`", $left, $right, left.is_ok(), right.is_ok()))
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left == right) {
                Ok(true)
            } else {
                Err(format!("assumption failed: `assume_fn_ok_eq(fn, left, right)`\n  left input: `{:?}`\n right input: `{:?}`\n  left output: `{:?}`\n right output: `{:?}`", $left, $right, left, right))
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            Err($($arg)+)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
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
    use std::str::FromStr;

    #[test]
    fn test_assume_fn_ok_eq_x_arity_2_success() {
        let a = "1";
        let b = "1";
        let x = assume_fn_ok_eq!(i32::from_str, a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_ok_eq_x_arity_2_failure() {
        let a = "1";
        let b = "2";
        let x = assume_fn_ok_eq!(i32::from_str, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assumption failed: `assume_fn_ok_eq(fn, left, right)`\n  left input: `\"1\"`\n right input: `\"2\"`\n  left output: `1`\n right output: `2`"
        );
    }

    #[test]
    fn test_assume_fn_ok_eq_x_arity_3_success() {
        let a = "1";
        let b = "1";
        let x = assume_fn_ok_eq!(i32::from_str, a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assume_fn_ok_eq_x_arity_3_failure() {
        let a = "1";
        let b = "2";
        let x = assume_fn_ok_eq!(i32::from_str, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
