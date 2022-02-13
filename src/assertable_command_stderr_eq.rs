/// Assert a command stderr string is equal to another.
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
/// let mut b = Command::new("printf");
/// let x = assertable_command_stderr_eq!(a, b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let mut b = Command::new("printf");
/// b.arg("-v");
/// let x = assertable_command_stderr_eq!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stderr_eq!(left_command, right_command)`
/// //   left command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //   left stderr: `\"usage: printf format [arguments ...]\\n\"`,
/// //  right stderr: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq!(left_command, right_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stderr: `\"usage: printf format [arguments ...]\\n\"`,\n right stderr: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_eq {
    ($left_command:expr, $right_command:expr $(,)?) => ({
        let left_output = $left_command.output();
        let right_output = $right_command.output();
        if left_output.is_err() || right_output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_eq!(left_command, right_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left_command.get_program(), $right_command.get_program(), left_output, right_output))
        } else {
            let left_actual = String::from_utf8(left_output.unwrap().stderr).unwrap();
            let right_actual = String::from_utf8(right_output.unwrap().stderr).unwrap();
            if left_actual == right_actual {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_eq!(left_command, right_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left stderr: `{:?}`,\n right stderr: `{:?}`", $left_command.get_program(), $right_command.get_program(), left_actual, right_actual))
            }
        }
    });
    ($left_command:expr, $right_command:expr, $($arg:tt)+) => ({
        let left_output = $left_command.output();
        let right_output = $right_command.output();
        if left_output.is_err() || right_output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let left_actual = String::from_utf8(left_output.unwrap().stderr).unwrap();
            let right_actual = String::from_utf8(right_output.unwrap().stderr).unwrap();
            if left_actual == right_actual {
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
    fn test_assertable_command_stderr_eq_x_arity_2_success() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        let x = assertable_command_stderr_eq!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stderr_eq_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        b.arg("-v");
        let x = assertable_command_stderr_eq!(a, b);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq!(left_command, right_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stderr: `\"usage: printf format [arguments ...]\\n\"`,\n right stderr: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn test_assertable_command_stderr_eq_x_arity_3_success() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        let x = assertable_command_stderr_eq!(a, b, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stderr_eq_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        b.arg("-v");
        let x = assertable_command_stderr_eq!(a, b, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
