/// Assert a command stdout string contains a given string.
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
/// a.args(["%s", "hello"]);
/// let str = "ell";
/// let x = assertable_command_stdout_contains_str!(a, str);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let str = "xyz";
/// let x = assertable_command_stdout_contains_str!(a, str);
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stdout_contains_str!(command, str)`
/// //  command program: `\"printf\"`,
/// //  str: `\"xyz\"`
/// //  stdout: `\"hello\"`,
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"xyz\"`,\n stdout: `\"hello\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_contains_str {
    ($command:expr, $str:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stdout_contains_str!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n output: {:?}", $command.get_program(), $str, output))
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if actual.contains($str) {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stdout_contains_str!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n stdout: `{:?}`", $command.get_program(), $str, actual))
            }
        }
    });
    ($command:expr, $str:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            Err($($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stdout).unwrap();
            if actual.contains($str) {
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
    fn asserterable_command_stdout_contains_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let str = "lph";
        let x = assertable_command_stdout_contains_str!(a, str);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let str = "xyz";
        let x = assertable_command_stdout_contains_str!(a, str);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"xyz\"`,\n stdout: `\"alpha\"`");
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let str = "lph";
        let x = assertable_command_stdout_contains_str!(a, str, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let str = "xyz";
        let x = assertable_command_stdout_contains_str!(a, str, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
