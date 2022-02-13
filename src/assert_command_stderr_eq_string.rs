/// Assert a command stderr string is equal to a given string.
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
/// let string = "usage: printf format [arguments ...]\n";
/// assert_command_stderr_eq_string!(a, string);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// let string = "xyz";
/// assert_command_stderr_eq_string!(a, string);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stderr_eq_string!(command, string)`
/// //  command program: `\"printf\"`,
/// //  string: `\"xyz\"`
/// //  stderr: `\"usage: printf format [arguments ...]\n"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stderr_eq_string!(command, string)`\n command program: `\"printf\"`,\n string: `\"xyz\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_eq_string {
    ($command:expr, $string:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stderr_eq!(command, string)`\n command program: `{:?}`,\n string: `{:?}`,\n output: `{:?}`", $command.get_program(), $string, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            let expect = String::from($string);
            if actual == expect {
                ()
            } else {
                panic!("assertion failed: `assert_command_stderr_eq_string!(command, string)`\n command program: `{:?}`,\n string: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $string, actual)
            }
        }
    });
    ($command:expr, $string:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            let expect = String::from($string);
            if actual == expect {
                ()
            } else {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stderr_eq_string_x_arity_2_success() {
        let mut a = Command::new("printf");
        let string = "usage: printf format [arguments ...]\n";
        let x = assert_command_stderr_eq_string!(a, string);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_eq_string!(command, string)`\n command program: `\"printf\"`,\n string: `\"alpha\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`")]
    fn test_assert_command_stderr_eq_string_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let string = "alpha";
        let _x = assert_command_stderr_eq_string!(a, string);
    }

    #[test]
    fn test_assert_command_stderr_eq_string_x_arity_3_success() {
        let mut a = Command::new("printf");
        let string = "usage: printf format [arguments ...]\n";
        let x = assert_command_stderr_eq_string!(a, string, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stderr_eq_string_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let string = "alpha";
        let _x = assert_command_stderr_eq_string!(a, string, "message");
    }

}
