/// Assert a command stdout string is equal to another.
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
/// use std::process::Command;
///
/// # fn main() {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
/// assert_command_stdout_eq!(a, b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
/// assert_command_stdout_eq!(a, b);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stdout_eq!(left_command, right_command)`
/// //   left command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //   left stdout: `\"hello\"`,
/// //  right stdout: `\"world\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stdout: `\"hello\"`,\n right stdout: `\"world\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stdout_eq {
    ($left_command:expr, $right_command:expr $(,)?) => ({
        let left_output = $left_command.output();
        let right_output = $right_command.output();
        if left_output.is_err() || right_output.is_err() {
            panic!("assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left_command.get_program(), $right_command.get_program(), left_output, right_output)
        } else {
            let left_string = String::from_utf8(left_output.unwrap().stdout).unwrap();
            let right_string = String::from_utf8(right_output.unwrap().stdout).unwrap();
            if left_string == right_string {
                ()
            } else {
                panic!("assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left stdout: `{:?}`,\n right stdout: `{:?}`", $left_command.get_program(), $right_command.get_program(), left_string, right_string)
            }
        }
    });
    ($left_command:expr, $right_command:expr, $($arg:tt)+) => ({
        let left_output = $left_command.output();
        let right_output = $right_command.output();
        if left_output.is_err() || right_output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let left_string = String::from_utf8(left_output.unwrap().stdout).unwrap();
            let right_string = String::from_utf8(right_output.unwrap().stdout).unwrap();
            if left_string == right_string {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_eq_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assert_command_stdout_eq!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stdout: `\"alpha\"`,\n right stdout: `\"bravo\"`")]
    fn test_assert_command_stdout_eq_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let _x = assert_command_stdout_eq!(a, b);
    }

    #[test]
    fn test_assert_command_stdout_eq_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assert_command_stdout_eq!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stdout_eq_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let _x = assert_command_stdout_eq!(a, b, "message");
    }

}
