/// Assert a command stdout string is equal to another.
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
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
/// let x = assertable_command_stdout_eq!(a, b);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
/// let x = assertable_command_stdout_eq!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stdout_eq!(a_command, b_command)`
/// //   left command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //   left stdout: `\"hello\"`,
/// //  right stdout: `\"world\"`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_eq!(a_command, b_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stdout: `\"hello\"`,\n right stdout: `\"world\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_eq {
    ($a_command:expr, $b_command:expr $(,)?) => ({
        let a_output = $a_command.output();
        let b_output = $b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(format!("assertable failed: `assertable_command_stdout_eq!(a_command, b_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $a_command.get_program(), $b_command.get_program(), a_output, b_output))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            let b_string = String::from_utf8(b_output.unwrap().stdout).unwrap();
            if a_string == b_string {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stdout_eq!(a_command, b_command)`\n  left command program: `{:?}`,\n right command program: `{:?}`,\n  left stdout: `{:?}`,\n right stdout: `{:?}`", $a_command.get_program(), $b_command.get_program(), a_string, b_string))
            }
        }
    });
    ($a_command:expr, $b_command:expr, $($arg:tt)+) => ({
        let a_output = $a_command.output();
        let b_output = $b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            let b_string = String::from_utf8(b_output.unwrap().stdout).unwrap();
            if a_string == b_string {
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
    fn test_assertable_command_stdout_eq_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assertable_command_stdout_eq!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stdout_eq_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let x = assertable_command_stdout_eq!(a, b);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_eq!(a_command, b_command)`\n  left command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stdout: `\"alpha\"`,\n right stdout: `\"bravo\"`");
    }

    #[test]
    fn test_assertable_command_stdout_eq_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assertable_command_stdout_eq!(a, b, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assertable_command_stdout_eq_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let x = assertable_command_stdout_eq!(a, b, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
