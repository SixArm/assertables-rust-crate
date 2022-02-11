/// Assert a function ok() is not equal to another.
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
/// fn example_digit_to_string(i: isize) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// assert_f_ok_ne!(example_digit_to_string, 1, 2);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_ok_ne!(example_digit_to_string, 1, 1);
/// //-> panic!
/// // assertion failed: `assert_f_ok_ne!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `1`,
/// //   right input: `1`,
/// //   left output: `\"1\"`,
/// //  right output: `\"1\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_ok_ne!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_ok_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            panic!("assertion failed: `assert_f_ok_ne!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is ok: `{:?}`,\n right output is ok: `{:?}`", stringify!($function), $left, $right, left.is_ok(), right.is_ok());
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left != right) {
                ()
            } else {
                panic!("assertion failed: `assert_f_ok_ne!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $left, $right, left, right);
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_ok() || !right.is_ok() {
            panic!("{:?}", $($arg)+)
        } else {
            let left = left.unwrap();
            let right = right.unwrap();
            if (left != right) {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    fn example_digit_to_string(i: isize) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_f_ok_ne_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assert_f_ok_ne!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_ok_ne!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `1`,\n  right input: `1`,\n  left output: `\"1\"`,\n right output: `\"1\"`")]
    fn test_assert_f_ok_ne_x_arity_2_failure() {
        let a = 1;
        let b = 1;
        let _x = assert_f_ok_ne!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_f_ok_ne_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assert_f_ok_ne!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_ok_ne_x_arity_3_failure() {
        let a = 1;
        let b = 1;
        let _x = assert_f_ok_ne!(example_digit_to_string, a, b, "message");
    }

}
