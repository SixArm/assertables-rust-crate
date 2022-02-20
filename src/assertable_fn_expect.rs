/// Assert a function output is equal to a given.
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
/// let x = assertable_fn_expect!(i32::abs, -1 as i32, 1 as i32);
/// //-> Ok(())
/// assert!(x.is_ok());
///
/// let x = assertable_fn_expect!(i32::abs, -1 as i32, 2 as i32);
/// //-> Err
/// // assertable failed: `assertable_fn_expect!(function, input, expect)`
/// //  function: `\"i32::abs\"`,
/// //  input: `-1`,
/// //  actual: `1`,
/// //  expect: `2`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_fn_expect!(function, input, expect)`\n function: `\"i32::abs\"`,\n input: `-1`,\n actual: `1`,\n expect: `2`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_fn_expect {
    ($function:path, $a:expr, $expect:expr $(,)?) => ({
        let a_output = $function($a);
        if (a_output == $expect) {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_fn_expect!(function, input, expect)`\n function: `{:?}`,\n input: `{:?}`,\n actual: `{:?}`,\n expect: `{:?}`", stringify!($function), $a, a_output, $expect))
        }
    });
    ($function:path, $a:expr, $expect:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        if (a_output == $expect) {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_fn_expect_x_arity_2_success() {
        let a = -1;
        let b = 1;
        let x = assertable_fn_expect!(i32::abs, a as i32, b as i32);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_fn_expect_x_arity_2_failure() {
        let a = -1;
        let b = 2;
        let x = assertable_fn_expect!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_fn_expect!(function, input, expect)`\n function: `\"i32::abs\"`,\n input: `-1`,\n actual: `1`,\n expect: `2`"
        );
    }

    #[test]
    fn test_assertable_fn_expect_x_arity_3_success() {
        let a = -1;
        let b = 1;
        let x = assertable_fn_expect!(i32::abs, a as i32, b as i32, "message");
        assert!(x.is_ok());
    }

    #[test]
    fn test_assertable_fn_expect_x_arity_3_failure() {
        let a = -1;
        let b = 2;
        let x = assertable_fn_expect!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
