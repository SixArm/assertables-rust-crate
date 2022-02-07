/// Assert a function err() is greater than another.
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
/// fn digit_string(i: isize) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// assert_f_err_string_gt!(digit_string, 20, 10);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// assert_f_err_string_gt!(digit_string, 10, 20);
/// //-> panic!("…")
/// // assertion failed: `assert_f_err_string_gt!(fn, left, right)`
/// //    left input: `10`,
/// //   right input: `20`,
/// //   left output: `\"10 is out of range\"`,
/// //  right output: `\"20 is out of range\"`
/// # });
/// # let actual: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_f_err_string_gt!(fn, left, right)`\n   left input: `10`,\n  right input: `20`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_f_err_string_gt {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            panic!("assertion failed: `assert_f_err_string_gt!(fn, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`\n  left output is_err(): `{:?}`,\n right output is_err(): `{:?}`", $left, $right, left.is_err(), right.is_err());
        } else {
            let left = left.unwrap_err().to_string();
            let right = right.unwrap_err().to_string();
            if (left > right) {
                ()
            } else {
                panic!("assertion failed: `assert_f_err_string_gt!(fn, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right);
            }
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if !left.is_err() || !right.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let left = left.unwrap_err();
            let right = right.unwrap_err();
            let left = left.to_string();
            let right = right.to_string();
            if (left > right) {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    // Replicate this function relevant tests in this crate.
    fn digit_string(i: isize) -> Result<String, String> {
        match i {
            0..=9 => Ok(format!("{}", i)),
            _ => Err(format!("{:?} is out of range", i)),
        }
    }

    #[test]
    fn test_assert_f_err_string_gt_x_arity_2_gt_success() {
        let a = 20;
        let b = 10;
        let x = assert_f_err_string_gt!(digit_string, a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_err_string_gt!(fn, left, right)`\n   left input: `10`,\n  right input: `10`,\n  left output: `\"10 is out of range\"`,\n right output: `\"10 is out of range\"`")]
    fn test_assert_f_err_string_gt_x_arity_2_eq_failure() {
        let a = 10;
        let b = 10;
        let _x = assert_f_err_string_gt!(digit_string, a, b);
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_f_err_string_gt!(fn, left, right)`\n   left input: `10`,\n  right input: `20`,\n  left output: `\"10 is out of range\"`,\n right output: `\"20 is out of range\"`")]
    fn test_assert_f_err_string_gt_x_arity_2_lt_failure() {
        let a = 10;
        let b = 20;
        let _x = assert_f_err_string_gt!(digit_string, a, b);
    }

    #[test]
    fn test_assert_f_err_string_gt_x_arity_3_gt_success() {
        let a = 20;
        let b = 10;
        let x = assert_f_err_string_gt!(digit_string, a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_err_string_gt_x_arity_3_eq_failure() {
        let a = 10;
        let b = 10;
        let _x = assert_f_err_string_gt!(digit_string, a, b, "message");
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_f_err_string_gt_x_arity_3_failure() {
        let a = 10;
        let b = 20;
        let _x = assert_f_err_string_gt!(digit_string, a, b, "message");
    }

}
