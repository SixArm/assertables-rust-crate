/// Assert a command stdout string is a match to a given pattern.
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
/// let pattern = 'el';
/// assert_command_stdout_eq!(a, b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let pattern = 'xy';
/// assert_command_stdout_eq!(a, b);
/// //-> panic!("…")
/// // assertion failed: `assert_command_stdout_eq!( command, pattern)`
/// //  command program: `\"printf\"`,
/// //  pattern: `\"abc\"`,
/// //  stdout: `\"hello\"`,
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stdout_eq!( command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"abc\"`,\n stdout: `\"hello\"``";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stdout_eq {
    ($ command:expr, $pattern:expr $(,)?) => ({
        let output = $ command.output();
        if output.is_err() {
            panic!("assertion failed: `assert_command_stdout_eq!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n  output: `{:?}`", $ command.get_program(), $pattern, output)
        } else {
            let string = String::from_utf8(output.unwrap().stdout).unwrap();
            if string match pattern {
                ()
            } else {
                panic!("assertion failed: `assert_command_stdout_eq!(command, pattern)`\n command program: `{:?}`,\n pattern: `{:?}`,\n stdout: `{:?}`,\n right stdout: `{:?}`", $ command.get_program(), $pattern, string)
            }
        }
    });
    ($ command:expr, $pattern:expr, $($arg:tt)+) => ({
        let output = $ command.output();
        if output.is_err() {
            panic!("{:?}", $($arg)+)
        } else {
            let string = String::from_utf8(output.unwrap().stdout).unwrap();
            if string match pattern {
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
    fn assert_command_stdout_eq_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'el';
        let x = assert_command_stdout_eq!(a, pattern);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stdout_eq!( command, pattern)`\n command program: `\"printf\"`,\n pattern: `\"printf\"`,\n stdout: `\"alpha\"`,\n right stdout: `\"bravo\"`")]
    fn assert_command_stdout_eq_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'xy';
        let _x = assert_command_stdout_eq!(a, b);
    }

    #[test]
    fn assert_command_stdout_eq_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'el';
        let x = assert_command_stdout_eq!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stdout_eq_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let pattern = 'xy';
        let _x = assert_command_stdout_eq!(a, b, "message");
    }

}
