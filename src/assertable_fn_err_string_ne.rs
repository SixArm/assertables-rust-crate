/// Assert one function ok() is not equal to another function ok().
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
/// let x = assertable_fn_err_string_ne!(f, 1, 2);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_fn_err_string_ne!(f, 1, 1);
/// //-> Err("…")
/// // assertable failed: `assertable_fn_err_string_ne!(fn, left, right)`
/// //    left input: `1`,
/// //   right input: `1`,
/// //   left output: `\"1\"`,
/// //  right output: `\"1\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_err_string_ne!(fn, left, right)`\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_err_string_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            Err(format!("assertable failed: `assertable_fn_err_string_ne!(fn, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is_err(): `{:?}`,\n right output is_err(): `{:?}`", $left, $right, left.is_err(), right.is_err()))
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left != right) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_fn_err_string_ne!(fn, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right))
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
            if (left != right) {
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
    fn test_assertable_fn_err_string_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assertable_fn_err_string_ne!(f, a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        let x = assertable_fn_err_string_ne!(f, a, b);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_err_string_ne!(fn, left, right)`\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`"
        );
    }

    #[test]
    fn test_assertable_fn_err_string_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assertable_fn_err_string_ne!(f, a, b, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_err_string_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        let x = assertable_fn_err_string_ne!(f, a, b, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
