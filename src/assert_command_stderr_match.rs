/// Assert a command stderr string is a match to a given pattern.
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
/// let pattern = 'el';
/// assert_command_stderr_match!(a, pattern);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let pattern = 'xy';
/// assert_command_stderr_match!(a, pattern);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stderr_match!(command, pattern)`
/// //  command program: `\"printf\"`,
/// //  match pattern: `\"printf\"`,
/// //  stderr: `\"hello\"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stderr_match!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"el\"`,\n stderr: `\"hello\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_match {
    ($command:expr, $pattern:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stderr_match!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n output: `{:?}`", $command.get_program(), $pattern, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual match $pattern {
                ()
            } else {
                panic!("assertion failed: `assert_command_stderr_match!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $pattern, actual)
            }
        }
    });
    ($command:expr, $pattern:expr, $($arg:tt)+) => ({
        let output = $command.output();
        let right_output = $pattern.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual match $pattern {
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
    fn assert_command_stderr_match_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'lp';
        let x = assert_command_stderr_match!(a, pattern);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_match!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"printf\"`,\n  left stderr: `\"alpha\"`,\n right stderr: `\"bravo\"`")]
    fn assert_command_stderr_match_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha", ">&2"]);
        let pattern = 'xy';
        let _x = assert_command_stderr_match!(a, pattern);
    }

    #[test]
    fn assert_command_stderr_match_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha", ">&2"]);
        let pattern = 'lp';
        let x = assert_command_stderr_match!(a, pattern, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stderr_match_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha", ">&2"]);
        let pattern = 'xy';
        let _x = assert_command_stderr_match!(a, pattern, "message");
    }

}
