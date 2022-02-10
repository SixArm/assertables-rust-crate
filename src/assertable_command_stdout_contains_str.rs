/// Assert a command stdout string is equal to a target string.
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
/// let x = assertable_command_stdout_contains_str!(a, "he");
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let x = assertable_command_stdout_contains_str!(a, "xy");
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stdout_contains_str!(command, substr)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  substr: `\"xy\"`
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains_str!(command, substr)`\n command program: `\"printf\"`,\n stdout: `\"hello\"`,\n substr: `\"xy\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_contains_str {
    ($command:expr, $substr:expr $(,)?) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual.contains($substr) {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_command_stdout_contains_str!(command, substr)`\n command program: `{:?}`,\n stdout: `{:?}`,\n substr: `{:?}`", $command.get_program(), actual, $substr))
                }
            }
            Err(err) => {
                Err(format!("assertable failed: `assertable_command_stdout_contains_str!(command, substr)`\n command program: `{:?}`,\n substr: `{:?}`,\n err: {:?}", $command.get_program(), $substr, err))
            }
        }
    });
    ($command:expr, $substr:expr, $($arg:tt)+) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual.contains($substr) {
                    Ok(())
                } else {
                    Err($($arg)+)
                }
            }
            Err(_) => {
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
        let substr = "a";
        let x = assertable_command_stdout_contains_str!(a, substr);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "z";
        let x = assertable_command_stdout_contains_str!(a, substr);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_contains_str!(command, substr)`\n command program: `\"printf\"`,\n stdout: `\"alpha\"`,\n substr: `\"z\"`");
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "a";
        let x = assertable_command_stdout_contains_str!(a, substr, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_contains_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "z";
        let x = assertable_command_stdout_contains_str!(a, substr, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
