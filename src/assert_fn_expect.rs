/// Assert a function output is equal to a given.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// assert_fn_expect!(i32::abs, -1 as i32, 1);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_fn_expect!(i32::abs, -1 as i32, 2);
/// //-> panic!("…")
/// // assertion failed: `assert_fn_expect!(function, input, expect)`
/// //  function: `\"i32::abs\"`,
/// //  input: `-1`,
/// //  actual: `1`,
/// //  expect: `2`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_fn_expect!(function, input, expect)`\n function: `\"i32::abs\"`,\n input: `-1`,\n actual: `1`,\n expect: `2`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_expect {
    ($function:path, $a:expr, $expect:expr $(,)?) => ({
        let a_output = $function($a);
        if (a_output == $expect) {
            ()
        } else {
            panic!("assertion failed: `assert_fn_expect!(function, input, expect)`\n function: `{:?}`,\n input: `{:?}`,\n actual: `{:?}`,\n expect: `{:?}`", stringify!($function), $a, a_output, $expect);
        }
    });
    ($function:path, $a:expr, $expect:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        if (a_output == $expect) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_fn_expect_x_arity_2_eq_success() {
        let a = -1;
        let b = 1;
        let x = assert_fn_expect!(i32::abs, a as i32, b as i32);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_expect!(function, input, expect)`\n function: `\"i32::abs\"`,\n input: `-1`,\n actual: `1`,\n expect: `2`")]
    fn test_assert_fn_expect_x_arity_2_ne_failure() {
        let a = -1;
        let b = 2;
        let _x = assert_fn_expect!(i32::abs, a as i32, b as i32);
    }

    #[test]
    fn test_assert_fn_expect_x_arity_3_eq_success() {
        let a = -1;
        let b = 1;
        let x = assert_fn_expect!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_expect_x_arity_3_ne_failure() {
        let a = -1;
        let b = 2;
        let _x = assert_fn_expect!(i32::abs, a as i32, b as i32, "message");
    }

}
