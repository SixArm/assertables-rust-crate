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
/// let regex = Regex::new(r"usage").unwrap();
/// let x = assertable_command_stderr_regex!(a, regex);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let regex = Regex::new(r"xyz").unwrap();
/// let x = assertable_command_stderr_regex!(a, regex);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stderr_regex!(command, regex)`
/// //   command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //  regex: `xyz`,
/// //  stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_regex {
    ($command:expr, $regex:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n otput: `{:?}`", $command.get_program(), $regex, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $regex.is_match(&actual) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $regex, actual))
            }
        }
    });
    ($command:expr, $regex:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if $regex.is_match(&actual) {
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
    fn assertable_command_stderr_regex_x_arity_2_success() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"usage").unwrap();
        let x = assertable_command_stderr_regex!(a, regex);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stderr_regex_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stderr_regex!(a, regex);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn assertable_command_stderr_regex_x_arity_3_success() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"usage").unwrap();
        let x = assertable_command_stderr_regex!(a, regex, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stderr_regex_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let regex = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stderr_regex!(a, regex, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
