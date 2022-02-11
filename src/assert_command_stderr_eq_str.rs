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
/// assert_command_stderr_eq_str!(a, "usage: printf format [arguments ...]\n");
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// assert_command_stderr_eq_str!(a, "hello");
/// //-> panic!("…")
/// // assertion failed: `assert_command_stderr_eq_str!(command, str)`
/// //  command program: `\"printf\"`,
/// //  str: `\"hello\"`
/// //  stderr: `\"usage: printf format [arguments ...]\n"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stderr_eq_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"hello\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_eq_str {
    ($command:expr, $str:expr $(,)?) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stderr_eq!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n output: `{:?}`", $command.get_program(), $str, output)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual == $str {
                ()
            } else {
                panic!("assertion failed: `assert_command_stderr_eq_str!(command, str)`\n command program: `{:?}`,\n str: `{:?}`,\n stderr: `{:?}`", $command.get_program(), $str, actual)
            }
        }
    });
    ($command:expr, $str:expr, $($arg:tt)+) => ({
        let output = $command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let actual = String::from_utf8(output.unwrap().stderr).unwrap();
            if actual == $str {
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
    fn assert_command_stderr_eq_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        let str = "usage: printf format [arguments ...]\n";
        let x = assert_command_stderr_eq_str!(a, str);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_eq_str!(command, str)`\n command program: `\"printf\"`,\n str: `\"alpha\"`,\n stderr: `\"usage: printf format [arguments ...]\\n\"`")]
    fn assert_command_stderr_eq_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let str = "alpha";
        let _x = assert_command_stderr_eq_str!(a, str);
    }

    #[test]
    fn assert_command_stderr_eq_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        let str = "usage: printf format [arguments ...]\n";
        let x = assert_command_stderr_eq_str!(a, str, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stderr_eq_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let str = "alpha";
        let _x = assert_command_stderr_eq_str!(a, str, "message");
    }

}
