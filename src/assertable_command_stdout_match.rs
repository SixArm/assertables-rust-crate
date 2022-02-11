/// Assert a command stdout string is a match to a given pattern.
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
///
/// # fn main() {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let pattern = 'el';
/// let x = assertable_command_stdout_match!(a, pattern);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let pattern = 'xy';
/// let x = assertable_command_stdout_match!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stdout_match!(command, pattern)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  pattern: `\"abc\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_match!(command, pattern)`\n command program: `\"printf\"`,\n pattner: `\"abc\"`,\n stdout: `\"hello\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_match {
    ($command:expr, $pattern:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stdout_match!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n output: `{:?}`", $command.get_program(), $pattern, output))
        } else {
            let string = String::from_utf8(left_output.unwrap().stdout).unwrap();
            if string match $pattern {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stdout_match!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n  stdout: `{:?}`", $command.get_program(), $pattern, string))
            }
        }
    });
    ($command:expr, $right_command:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let string = String::from_utf8(left_output.unwrap().stdout).unwrap();
            if string match $pattern {
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

    #[test]
    fn assertable_command_stdout_match_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'lp';
        let x = assertable_command_stdout_match!(a, pattern);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stdout_match_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'xy';
        let x = assertable_command_stdout_match!(a, b);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_match!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"abc\"`,\n stdout: `\"alpha\"`");
    }

    #[test]
    fn assertable_command_stdout_match_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'lp';
        let x = assertable_command_stdout_match!(a, pattern, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stdout_match_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'xy';
        let x = assertable_command_stdout_match!(a, pattern, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
