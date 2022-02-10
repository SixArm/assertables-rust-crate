/// Assert a command stdout string is equal to a target string.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
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
/// assert_command_stdout_contains_str!(a, "hel");
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// assert_command_stdout_contains_str!(a, "xy");
/// //-> panic!("…")
/// // assertion failed: `assert_command_stdout_contains_str!(command, substr)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  substr: `\"xy\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stdout_contains_str!(command, substr)`\n command program: `\"printf\"`,\n stdout: `\"hello\"`,\n substr: `\"xy\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stdout_contains_str {
    ($command:expr, $substr:expr $(,)?) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual.contains($substr) {
                    ()
                } else {
                    panic!("assertion failed: `assert_command_stdout_contains_str!(command, substr)`\n command program: `{:?}`,\n stdout: `{:?}`,\n substr: `{:?}`", $command.get_program(), actual, $substr)
                }
            }
            Err(err) => {
                panic!("assertion failed: `assert_command_stdout_contains_str!(command, substr)`\n command program: `{:?}`,\n substr: `{:?}`,\n err: {:?}", $command.get_program(), $substr, err)
            }
        }
    });
    ($command:expr, $expect:expr, $($arg:tt)+) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual.contains($expect) {
                    ()
                } else {
                    panic!("{:?}", $($arg)+)
                }
            }
            Err(_) => {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn assert_command_stdout_contains_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "a";
        let x = assert_command_stdout_contains_str!(a, substr);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stdout_contains_str!(command, substr)`\n command program: `\"printf\"`,\n stdout: `\"alpha\"`,\n substr: `\"z\"`")]
    fn assert_command_stdout_contains_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "z";
        let _x = assert_command_stdout_contains_str!(a, substr);
    }

    #[test]
    fn assert_command_stdout_contains_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "a";
        let x = assert_command_stdout_contains_str!(a, substr, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stdout_contains_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let substr = "z";
        let _x = assert_command_stdout_contains_str!(a, substr, "message");
    }

}
