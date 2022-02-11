/// Assert a command stderr string is a match to a given pattern.
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
/// let x = assertable_command_stderr_match!(a, b);
/// //-> Ok(())
/// assert_match!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
/// let x = assertable_command_stderr_match!(a, b);
/// //-> Err("…")
/// // assertable failed: `assertable_command_stderr_match!(command, right_command)`
/// //   command program: `\"printf\"`,
/// //  right command program: `\"printf\"`,
/// //   left stderr: `\"hello\"`,
/// //  right stderr: `\"world\"`
/// assert_match!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_match!(left, right)`\n  command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stderr: `\"hello\"`,\n right stderr: `\"world\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stderr_match {
    ($command:expr, $right_command:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            Err(format!("assertable failed: `assertable_command_stderr_match!(left, right)`\n  command program: `{:?}`,\n right command program: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $command.get_program(), $right_command.get_program(), output, right_output))
        } else {
            let left_actual = String::from_utf8(output.unwrap().stderr).unwrap();
            let right_actual = String::from_utf8(right_output.unwrap().stderr).unwrap();
            if left_actual == right_actual {
                Ok(())
            } else {
                Err(format!("assertable failed: `assertable_command_stderr_match!(left, right)`\n  command program: `{:?}`,\n right command program: `{:?}`,\n  left stderr: `{:?}`,\n right stderr: `{:?}`", $command.get_program(), $right_command.get_program(), left_actual, right_actual))
            }
        }
    });
    ($command:expr, $right_command:expr, $($arg:tt)+) => ({
        let output = $command.output();
        let right_output = $right_command.output();
        if output.is_err() {
            Err(format!("{}", $($arg)+))
        } else {
            let left_actual = String::from_utf8(output.unwrap().stderr).unwrap();
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
    fn assertable_command_stderr_match_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assertable_command_stderr_match!(a, b);
        assert_match!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stderr_match_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let x = assertable_command_stderr_match!(a, b);
        assert_match!(x.unwrap_err(), "assertable failed: `assertable_command_stderr_match!(left, right)`\n  command program: `\"printf\"`,\n right command program: `\"printf\"`,\n  left stderr: `\"alpha\"`,\n right stderr: `\"bravo\"`");
    }

    #[test]
    fn assertable_command_stderr_match_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assertable_command_stderr_match!(a, b, "message");
        assert_match!(x.unwrap(), ());
    }

    #[test]
    fn assertable_command_stderr_match_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let x = assertable_command_stderr_match!(a, b, "message");
        assert_match!(x.unwrap_err(), "message");
    }

}
