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
/// let x = assertable_command_stderr_eq_str!(a, "usage: printf format [arguments ...]\n");
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// let x = assertable_command_stderr_eq_str!(a, "hello");
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stderr_eq_str!(command, str)`
/// //  command program: `\"printf\"`,
/// //  str: `\"hello\"`
/// //  stderr: `\"usage: printf format [arguments ...]\n\"`,
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"hello\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_eq_str {
    ($command:expr, $str:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_eq_str!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n output: {:?}", $command.get_program(), $str, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual == $str {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_eq_str!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $str, actual))
            }
        }
    });
    ($command:expr, $str:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            Err($($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual == $str {
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
    fn asserterable_command_stderr_eq_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        let str = "usage: printf format [arguments ...]\n";
        let x = assertable_command_stderr_eq_str!(a, str);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stderr_eq_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let str = "bravo";
        let x = assertable_command_stderr_eq_str!(a, str);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_eq_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"bravo\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`");
    }

    #[test]
    fn asserterable_command_stderr_eq_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        let str = "usage: printf format [arguments ...]\n";
        let x = assertable_command_stderr_eq_str!(a, str, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stderr_eq_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let str = "bravo";
        let x = assertable_command_stderr_eq_str!(a, str, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
