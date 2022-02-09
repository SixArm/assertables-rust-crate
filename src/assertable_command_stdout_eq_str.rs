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
/// let x = assertable_command_stdout_eq_str!(a, "hello");
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let x = assertable_command_stdout_eq_str!(a, "world");
/// //-> Err!("…")
/// // assertable failed: `assertable_command_stdout_eq_str!(command, expect)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  expect: `\"world\"`
/// # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_eq_str!(command, expect)`\n command program: `\"printf\"`,\n stdout: `\"hello\"`,\n expect: `\"world\"`");
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_command_stdout_eq_str {
    ($command:expr, $expect:expr $(,)?) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual == $expect {
                    Ok(())
                } else {
                    Err(format!("assertable failed: `assertable_command_stdout_eq_str!(command, expect)`\n command program: `{:?}`,\n stdout: `{:?}`,\n expect: `{:?}`", $command.get_program(), actual, $expect))
                }
            }
            Err(err) => {
                Err(format!("assertable failed: `assertable_command_stdout_eq_str!(command, expect)`\n command program: `{:?}`,\n expect: `{:?}`,\n err: {:?}", $command.get_program(), $expect, err))
            }
        }
    });
    ($command:expr, $expect:expr, $($arg:tt)+) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual == $expect {
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
    fn asserterable_command_stdout_eq_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "alpha";
        let x = assertable_command_stdout_eq_str!(a, expect);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_eq_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "bravo";
        let x = assertable_command_stdout_eq_str!(a, expect);
        assert_eq!(x.unwrap_err(), "assertable failed: `assertable_command_stdout_eq_str!(command, expect)`\n command program: `\"printf\"`,\n stdout: `\"alpha\"`,\n expect: `\"bravo\"`");
    }

    #[test]
    fn asserterable_command_stdout_eq_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "alpha";
        let x = assertable_command_stdout_eq_str!(a, expect, "message");
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn asserterable_command_stdout_eq_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "bravo";
        let x = assertable_command_stdout_eq_str!(a, expect, "message");
        assert_eq!(x.unwrap_err(), "message");
    }

}
