/// Assert one function output is less than or equal to another function output.
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
/// # use std::panic;
/// # fn main() {
/// let x = assertable_fn_le!(i32::abs, 1 as i32, -2 as i32);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_fn_le!(i32::abs, -2 as i32, 1 as i32);
/// //-> Err("…")
/// // assertable failed: `assertable_fn_le!(function, left, right)`
/// //      function: `\"i32::abs\"`,
/// //    left input: `-2`,
/// //   right input: `1`,
/// //   left output: `2`,
/// //  right output: `1`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_le!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_le {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        if (a_output <= b_output) {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_fn_le!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_output, b_output))
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        if (a_output <= b_output) {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_fn_le_x_arity_2_lt_success() {
        let a = 1;
        let b = -2;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_le_x_arity_2_eq_success() {
        let a = 1;
        let b = -1;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_le_x_arity_2_gt_failure() {
        let a = -2;
        let b = 1;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_le!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`"
        );
    }

    #[test]
    fn test_assertable_fn_le_x_arity_3_lt_success() {
        let a = 1;
        let b = -2;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_le_x_arity_3_eq_success() {
        let a = 1;
        let b = -1;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_fn_le_x_arity_3_gt_failure() {
        let a = -2;
        let b = 1;
        let x = assertable_fn_le!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
