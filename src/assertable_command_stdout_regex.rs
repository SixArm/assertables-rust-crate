/// Assert a command stdout string is a match to a given regex.
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
/// a.args(["%s", "hello"]);
/// let regex = Regex::new(r"el").unwrap();
/// let x = assertable_command_stdout_regex!(a, regex);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let regex = Regex::new(r"xyz").unwrap();
/// let x = assertable_command_stdout_regex!(a, regex);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stdout_regex!(command, regex)`
/// //  command program: `\"printf\"`,
/// //  regex: `xyz`
/// //  stdout: `\"hello\"`,
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stdout: `\"hello\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_regex {
    ($command:expr, $regex:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stdout_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n output: `{:?}`", $command.get_program(), $regex, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if $regex.is_match(&actual) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stdout_regex!(command, regex)`\n command program: `{:?}`,\n regex: `{:?}`,\n stdout: `{:?}`", $command.get_program(), $regex, actual))
            }
        }
    });
    ($command:expr, $regex:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
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
    fn assertable_command_stdout_regex_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let regex = Regex::new(r"lph").unwrap();
        let x = assertable_command_stdout_regex!(a, regex);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stdout_regex_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let regex = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stdout_regex!(a, regex);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_regex!(command, regex)`\n command program: `\"printf\"`,\n regex: `xyz`,\n stdout: `\"alpha\"`");
    }

    #[test]
    fn assertable_command_stdout_regex_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let regex = Regex::new(r"lph").unwrap();
        let x = assertable_command_stdout_regex!(a, regex, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stdout_regex_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let regex = Regex::new(r"xyz").unwrap();
        let x = assertable_command_stdout_regex!(a, regex, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
