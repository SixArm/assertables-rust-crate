/// Assert a command stderr string is a match to a given regex.
///
/// * When true, return `()`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// use std::process::Command;
/// use regex::Regex;
///
/// # fn main() {
/// let mut command = Command::new("printf");
/// let matcher = Regex::new(r"usage").unwrap();
/// let x = assert_command_stderr_matches_as_result!(command, matcher);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// let matcher = Regex::new(r"xyz").unwrap();
/// let x = assert_command_stderr_matches_as_result!(command, matcher);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_matches!(left_command, right_expr)`\n",
///     "  left command name: `command`,\n",
///     "    right expr name: `matcher`,\n",
///     "       left command: `\"printf\"`,\n",
///     "         right expr: `xyz`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_matches_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(msg_with_left_command_and_right_expr!(
                "assertion failed",
                "assert_command_stderr_matches!",
                stringify!($a_command),
                stringify!($b_expr),
                $a_command.get_program(),
                $b_expr,
                a_output,
                $b_expr
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if $b_expr.is_match(&a_string) {
                Ok(())
            } else {
                Err(msg_with_left_command_and_right_expr!(
                    "assertion failed",
                    "assert_command_stderr_matches!",
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
    use regex::Regex;

    #[test]
    fn test_assert_command_stderr_matches_as_result_x_arity_2_success() {
        let mut a = Command::new("printf");
        let matcher = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches_as_result!(a, matcher);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_matches_as_result_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"xyz").unwrap();
        let x = assert_command_stderr_matches_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_matches!(left_command, right_expr)`\n",
            "  left command name: `a`,\n",
            "    right expr name: `b`,\n",
            "       left command: `\"printf\"`,\n",
            "         right expr: `xyz`,\n",
            "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "              right: `xyz`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is a match to a given regex.
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
/// use regex::Regex;
///
/// # fn main() {
/// let mut command = Command::new("printf");
/// let matcher = Regex::new(r"usage").unwrap();
/// assert_command_stderr_matches!(command, matcher);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// let matcher = Regex::new(r"xyz").unwrap();
/// assert_command_stderr_matches!(command, matcher);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_matches!(left_command, right_expr)`\n",
///     "  left command name: `command`,\n",
///     "    right expr name: `matcher`,\n",
///     "       left command: `\"printf\"`,\n",
///     "         right expr: `xyz`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_matches {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stderr_matches_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_command_stderr_matches_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    use std::process::Command;
    use regex::Regex;

    #[test]
    fn test_assert_command_stderr_matches_x_arity_2_success() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_matches!(left_command, right_expr)`\n  left command name: `a`,\n    right expr name: `b`,\n       left command: `\"printf\"`,\n         right expr: `xyz`,\n               left: `\"usage: printf format [arguments ...]\\n\"`,\n              right: `xyz`")]
    fn test_assert_command_stderr_matches_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_matches!(a, b);
    }

    #[test]
    fn test_assert_command_stderr_matches_x_arity_3_success() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stderr_matches_x_arity_3_failure() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"xyz").unwrap();
        let _x = assert_command_stderr_matches!(a, b, "message");
    }

}
