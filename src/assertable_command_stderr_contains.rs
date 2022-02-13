/// Assert a command stderr string contains a given pattern.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
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
/// use std::process::Command;
///
/// # fn main() {
/// let mut a = Command::new("printf");
/// let x = assertable_command_stderr_contains!(a, "usage");
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let x = assertable_command_stderr_contains!(a, "xyz");
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stderr_contains!(command, pattern)`
/// //  command program: `\"printf\"`,
/// //  pattern: `\"xyz\"`
/// //  stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_contains {
    ($command:expr, $pattern:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n output: {:?}", $command.get_program(), $pattern, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual.contains($pattern) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $pattern, actual))
            }
        }
    });
    ($command:expr, $pattern:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            Err($($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual.contains($pattern) {
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
    fn test_asserterable_read_to_string_contains_x_arity_2_success() {
        let mut a = Command::new("printf");
        let pattern = "usage";
        let x = assertable_command_stderr_contains!(a, pattern);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_read_to_string_contains_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let pattern = "xyz";
        let x = assertable_command_stderr_contains!(a, pattern);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn test_asserterable_read_to_string_contains_x_arity_3_success() {
        let mut a = Command::new("printf");
        let pattern = "usage";
        let x = assertable_command_stderr_contains!(a, pattern, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_read_to_string_contains_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let pattern = "xyz";
        let x = assertable_command_stderr_contains!(a, pattern, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
