/// Assert a command stderr string is equal to a given string.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::process::Command;
///
/// # fn main() {
/// let mut a = Command::new("printf");
/// let string = "usage: printf format [arguments ...]\n";
/// let x = assertable_command_stderr_eq_string!(a, string);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let string = "xyz";
/// let x = assertable_command_stderr_eq_string!(a, string);
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stderr_eq_string!(command, string)`
/// //  command program: `\"printf\"`,
/// //  string: `\"xyz\"`
/// //  stderr: `\"usage: printf format [arguments ...]\n\"`,
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq_string!(command, string)`\n command program: `\"printf\"`,\n string: `\"xyz\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_eq_string {
    ($command:expr, $string:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_eq_string!(command, string)`\n command program: `{:?}`,\n string: `{:?}`,\n output: {:?}", $command.get_program(), $string, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            let expect = String::from($string);
            if actual == expect {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_eq_string!(command, string)`\n command program: `{:?}`,\n string: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $string, actual))
            }
        }
    });
    ($command:expr, $string:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            Err($($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            let expect = String::from($string);
            if actual == expect {
                Ok(())
            } else {
                Err($($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_asserterable_command_stderr_eq_string_x_arity_2_success() {
        let mut a = Command::new("printf");
        let string = "usage: printf format [arguments ...]\n";
        let x = assertable_command_stderr_eq_string!(a, string);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stderr_eq_string_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let string = "bravo";
        let x = assertable_command_stderr_eq_string!(a, string);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq_string!(command, string)`\n command program: `\"printf\"`,\n string: `\"bravo\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn test_asserterable_command_stderr_eq_string_x_arity_3_success() {
        let mut a = Command::new("printf");
        let string = "usage: printf format [arguments ...]\n";
        let x = assertable_command_stderr_eq_string!(a, string, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stderr_eq_string_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let string = "bravo";
        let x = assertable_command_stderr_eq_string!(a, string, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
