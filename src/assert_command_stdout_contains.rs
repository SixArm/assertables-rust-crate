/// Assert a command stdout string contains a given containee.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// This uses [`std::String`] method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character matches.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::process::Command;
///
/// # fn main() {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let containee = "ell";
/// let x = assert_command_stdout_contains_as_result!(command, containee);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let containee = "xyz";
/// let x = assert_command_stdout_contains_as_result!(command, containee);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_contains!(left_command, right_expr)`\n",
///     "  left command name: `command`,\n",
///     "    right expr name: `containee`,\n",
///     "       left command: `\"printf\"`,\n",
///     "         right expr: `\"xyz\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stdout_contains_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(msg_with_left_command_and_right_expr!(
                "assertion failed",
                "assert_command_stdout_contains!",
                stringify!($a_command), 
                stringify!($b_expr), 
                $a_command.get_program(), 
                $b_expr, 
                a_output,
                $b_expr
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            if a_string.contains($b_expr) {
                Ok(())
            } else {
                Err(msg_with_left_command_and_right_expr!(
                    "assertion failed",
                    "assert_command_stdout_contains!",
                    stringify!($a_command), 
                    stringify!($b_expr), 
                    $a_command.get_program(), 
                    $b_expr, 
                    a_string,
                    $b_expr
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    use std::process::Command;

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "lph";
        let x = assert_command_stdout_contains_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "xyz";
        let x = assert_command_stdout_contains_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_contains!(left_command, right_expr)`\n",
            "  left command name: `a`,\n",
            "    right expr name: `b`,\n",
            "       left command: `\"printf\"`,\n",
            "         right expr: `\"xyz\"`,\n",
            "               left: `\"alpha\"`,\n",
            "              right: `\"xyz\"`"
        );
        assert_eq!(actual, expect);
    }

}

/// Assert a command stdout string contains a given containee.
///
/// * When true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`std::String`] method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character matches.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// use std::process::Command;
///
/// # fn main() {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let containee = "ell";
/// assert_command_stdout_contains!(command, containee);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let containee = "xyz";
/// assert_command_stdout_contains!(command, containee);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_contains!(left_command, right_expr)`\n",
///     "  left command name: `command`,\n",
///     "    right expr name: `containee`,\n",
///     "       left command: `\"printf\"`,\n",
///     "         right expr: `\"xyz\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stdout_contains {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stdout_contains_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_command_stdout_contains_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });    
}

#[cfg(test)]
mod test_x_panic {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_contains_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "lph";
        let x = assert_command_stdout_contains!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assert_command_stdout_contains!(left_command, right_expr)`\n  left command name: `a`,\n    right expr name: `b`,\n       left command: `\"printf\"`,\n         right expr: `\"xyz\"`,\n               left: `\"alpha\"`,\n              right: `\"xyz\"`")]
    fn test_assert_command_stdout_contains_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "xyz";
        let _x = assert_command_stdout_contains!(a, b);
    }

    #[test]
    fn test_assert_command_stdout_contains_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "lph";
        let x = assert_command_stdout_contains!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stdout_contains_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "xyz";
        let _x = assert_command_stdout_contains!(a, b, "message");
    }

}
