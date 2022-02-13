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
/// let regex = Regex::new(r"usage").unwrap();
/// assert_command_stderr_regex!(a, regex);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// let regex = Regex::new(r"xyz").unwrap();
/// assert_command_stderr_regex!(a, regex);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stderr_regex!(command, regex)`
/// //  command program: `\"printf\"`,
/// //  regex: `xyz`,
/// //  stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stderr_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_regex {
    ($command:expr, $regex:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stderr_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n output: `{:?}`", $command.get_program(), $regex, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $regex.is_match(&actual) {
                ()
            } else {
                panic!("assertion failed: `assert_command_stderr_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $regex, actual)
            }
        }
    });
    ($command:expr, $regex:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $regex.is_match(&actual) {
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
    fn assert_command_stderr_regex_x_arity_2_success() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_regex!(a, regex);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`")]
    fn assert_command_stderr_regex_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_regex!(a, regex);
    }

    #[test]
    fn assert_command_stderr_regex_x_arity_3_success() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_regex!(a, regex, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stderr_regex_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_regex!(a, regex, "message");
    }

}
