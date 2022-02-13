/// Assert a command stdout string contains a given pattern.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`std::String`] method `contains`.
/// 
/// * The pattern can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character matches.
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
/// assert_command_stdout_contains!(a, "ell");
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// assert_command_stdout_contains!(a, "xyz");
/// //-> panic!("…")
/// // assertion failed: `assert_command_stdout_contains!(command, pattern)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  substring: `\"xyz\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stdout_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stdout: `\"hello\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stdout_contains {
    ($command:expr, $pattern:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stdout_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n output: {:?}", $command.get_program(), $pattern, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if actual.contains($pattern) {
                ()
            } else {
                panic!("assertion failed: `assert_command_stdout_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n stdout: `{:?}`", $command.get_program(), $pattern, actual)
            }
        }
    });
    ($command:expr, $pattern:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if actual.contains($pattern) {
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
    fn test_assert_command_stdout_contains_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "lph";
        let x = assert_command_stdout_contains!(a, pattern);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stdout_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stdout: `\"alpha\"`")]
    fn test_assert_command_stdout_contains_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "xyz";
        let _x = assert_command_stdout_contains!(a, pattern);
    }

    #[test]
    fn test_assert_command_stdout_contains_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "lph";
        let x = assert_command_stdout_contains!(a, pattern, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stdout_contains_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "xyz";
        let _x = assert_command_stdout_contains!(a, pattern, "message");
    }

}
