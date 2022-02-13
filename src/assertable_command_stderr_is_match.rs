/// Assert a command stderr string is a match to a given regex.
///
/// * When true, return `()`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
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
/// let x = assertable_command_stderr_is_match!(a, matchable);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let matchable = Regex::new(r"xyz").unwrap();
/// let x = assertable_command_stderr_is_match!(a, matchable);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stderr_is_match!(command, regex)`
/// //   command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //  matchable: `xyz`,
/// //  stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_is_match!(command, regex)`\n command program: `\"printf\"`,\n matchable: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_is_match {
    ($command:expr, $matchable:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_is_match!(command, regex)`\n command program: `{:?}`,\n matchable: `{:?}`,\n otput: `{:?}`", $command.get_program(), $matchable, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $matchable.is_match(&actual) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_is_match!(command, regex)`\n command program: `{:?}`,\n matchable: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $matchable, actual))
            }
        }
    });
    ($command:expr, $matchable:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $matchable.is_match(&actual) {
                Ok(())
            } else {
                Err(format!("{}", $($arg)+))
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;
    use regex::Regex;

    #[test]
    fn test_assertable_command_stderr_is_match_x_arity_2_success() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"usage").unwrap();
        let x = assertable_command_stderr_is_match!(a, matchable);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stderr_is_match_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stderr_is_match!(a, matchable);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_is_match!(command, regex)`\n command program: `\"printf\"`,\n matchable: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn test_assertable_command_stderr_is_match_x_arity_3_success() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"usage").unwrap();
        let x = assertable_command_stderr_is_match!(a, matchable, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stderr_is_match_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let matchable = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stderr_is_match!(a, matchable, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}