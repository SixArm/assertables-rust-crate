/// Assert a command stdout string contains a given pattern.
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
/// a.args(["%s", "hello"]);
/// let pattern = "ell";
/// let x = assertable_command_stdout_contains!(a, pattern);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let pattern = "xyz";
/// let x = assertable_command_stdout_contains!(a, pattern);
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stdout_contains!(command, pattern)`
/// //  command program: `\"printf\"`,
/// //  pattern: `\"xyz\"`
/// //  stdout: `\"hello\"`,
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stdout: `\"hello\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_contains {
    ($command:expr, $pattern:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stdout_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n output: {:?}", $command.get_program(), $pattern, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if actual.contains($pattern) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stdout_contains!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n stdout: `{:?}`", $command.get_program(), $pattern, actual))
            }
        }
    });
    ($command:expr, $pattern:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            Err($($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
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
    fn test_asserterable_command_stdout_contains_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "lph";
        let x = assertable_command_stdout_contains!(a, pattern);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "xyz";
        let x = assertable_command_stdout_contains!(a, pattern);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains!(command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"xyz\"`,\n stdout: `\"alpha\"`");
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "lph";
        let x = assertable_command_stdout_contains!(a, pattern, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = "xyz";
        let x = assertable_command_stdout_contains!(a, pattern, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
