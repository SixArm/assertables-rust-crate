/// Assert a command stderr string is a match to a given regex.
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
/// use regex::Regex;
///
/// # fn main() {
/// let mut a = Command::new("printf");
/// let matchable = Regex::new(r"usage").unwrap();
/// assert_command_stderr_matches!(a, matchable);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// let matchable = Regex::new(r"xyz").unwrap();
/// assert_command_stderr_matches!(a, matchable);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stderr_matches!(command, regex)`
/// //  command program: `\"printf\"`,
/// //  matchable: `xyz`,
/// //  stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stderr_matches!(command, regex)`\n command program: `\"printf\"`,\n matchable: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_matches {
    ($command:expr, $matchable:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stderr_matches!(command, regex)`\n command program: `{:?}`,\n matchable: `{:?}`,\n output: `{:?}`", $command.get_program(), $matchable, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $matchable.is_match(&actual) {
                ()
            } else {
                panic!("assertion failed: `assert_command_stderr_matches!(command, regex)`\n command program: `{:?}`,\n matchable: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $matchable, actual)
            }
        }
    });
    ($command:expr, $matchable:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $matchable.is_match(&actual) {
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
    use regex::Regex;

    #[test]
    fn test_assert_command_stderr_matches_x_arity_2_success() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches!(a, matchable);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_matches!(command, regex)`\n command program: `\"printf\"`,\n matchable: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`")]
    fn test_assert_command_stderr_matches_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_matches!(a, matchable);
    }

    #[test]
    fn test_assert_command_stderr_matches_x_arity_3_success() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches!(a, matchable, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stderr_matches_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_matches!(a, matchable, "message");
    }

}
