/// Assert a function output is not equal to another.
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
/// assert_fn_ne!(i32::abs, 1 as i32, -2 as i32);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_fn_ne!(i32::abs, 1 as i32, -1 as i32);
/// //-> panic!("…")
/// // assertion failed: `assert_fn_ne!(function, left, right)`
/// //      function: `\"i32::abs\"`,
/// //    left input: `1`,
/// //   right input: `-1`,
/// //   left output: `1`,
/// //  right output: `1`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_fn_ne!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `1`,\n  right input: `-1`,\n  left output: `1`,\n right output: `1`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_ne {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        if (a_output != b_output) {
            ()
        } else {
            panic!("assertion failed: `assert_fn_ne!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_output, b_output);
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        if (a_output != b_output) {
            ()
        } else {
            panic!("{:?}", $($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_fn_ne_x_arity_2_success() {
        let a = 1;
        let b = -2;
        let x = assert_fn_ne!(i32::abs, a as i32, b as i32);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_ne!(function, left, right)`\n     function: `\"i32::abs\"`,\n   left input: `1`,\n  right input: `-1`,\n  left output: `1`,\n right output: `1`")]
    fn test_assert_fn_ne_x_arity_2_failure() {
        let a = 1;
        let b = -1;
        let _x = assert_fn_ne!(i32::abs, a as i32, b as i32);
    }

    #[test]
    fn test_assert_fn_ne_x_arity_3_success() {
        let a = 1;
        let b = -2;
        let x = assert_fn_ne!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_ne_x_arity_3_failure() {
        let a = 1;
        let b = -1;
        let _x = assert_fn_ne!(i32::abs, a as i32, b as i32, "message");
    }

}
