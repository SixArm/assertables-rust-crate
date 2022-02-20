/// Assert a function err() is equal to another.
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
/// assert_fn_err_eq!(example_digit_to_string, 10, 10);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_fn_err_eq!(example_digit_to_string, 10, 20);
/// //-> panic!("…")
/// // assertion failed: `assert_fn_err_eq!(function, left, right)`
/// //      function: `\"example_digit_to_string\"`,
/// //    left input: `10`,
/// //   right input: `20`,
/// //   left is err: `true`,
/// //  right is err: `true`,
/// //   left output: `\"10 is out of range\"`,
/// //  right output: `\"20 is out of range\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_fn_err_eq!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_fn_err_eq {
    ($function:path, $a:expr, $b:expr $(,)?) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        if !a_is_err || !b_is_err {
            panic!("assertion failed: `assert_fn_err_eq!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`", stringify!($function), $a, $b, a_is_err, b_is_err);
        } else {
            let a_err = a_output.unwrap_err();
            let b_err = b_output.unwrap_err();
            if a_err == b_err {
                ()
            } else {
                panic!("assertion failed: `assert_fn_err_eq!(function, left, right)`\n     function: `{:?}`,\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left is err: `{:?}`,\n right is err: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", stringify!($function), $a, $b, a_is_err, b_is_err, a_err, b_err);
            }
        }
    });
    ($function:path, $a:expr, $b:expr, $($arg:tt)+) => ({
        let a_output = $function($a);
        let b_output = $function($b);
        let a_is_err = a_output.is_err();
        let b_is_err = b_output.is_err();
        if !a_is_err || !b_is_err {
            panic!("{:?}", $($arg)+)
        } else {
            let a_err = a_output.unwrap_err();
            let b_err = b_output.unwrap_err();
            if a_err == b_err {
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
    fn test_assert_fn_err_eq_x_arity_2_eq_success() {
        let a = 10;
        let b = 10;
        let x = assert_fn_err_eq!(example_digit_to_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_fn_err_eq!(function, left, right)`\n     function: `\"example_digit_to_string\"`,\n   left input: `10`,\n  right input: `20`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`")]
    fn test_assert_fn_err_eq_x_arity_2_ne_failure() {
        let a = 10;
        let b = 20;
        let _x = assert_fn_err_eq!(example_digit_to_string, a, b);
    }

    #[test]
    fn test_assert_fn_err_eq_x_arity_3_eq_success() {
        let a = 10;
        let b = 10;
        let x = assert_fn_err_eq!(example_digit_to_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_fn_err_eq_x_arity_3_ne_failure() {
        let a = 10;
        let b = 20;
        let _x = assert_fn_err_eq!(example_digit_to_string, a, b, "message");
    }

}
