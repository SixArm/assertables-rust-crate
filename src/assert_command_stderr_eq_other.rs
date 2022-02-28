/// Assert a command stderr string is equal to another.
///
/// * When true, return `()`.
///
/// * When true, return Result `Err` with a message and the values of the
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
/// let mut b = Command::new("printf");
/// let x = assert_command_stderr_eq_other_as_result!(a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut a = Command::new("printf");
/// let mut b = Command::new("printf");
/// b.arg("-v");
/// let x = assert_command_stderr_eq_other_as_result!(a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_eq_other!(left_command, right_command)`\n",
///     "  left command name: `a`,\n",
///     " right command name: `b`,\n",
///     "       left command: `\"printf\"`,\n",
///     "      right command: `\"printf\"`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_eq_other_as_result {
    ($a_command:expr, $b_command:expr $(,)?) => ({
        let a_output = $a_command.output();
        let b_output = $b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(msg_with_left_command_and_right_command!(
                "assertion failed",
                "assert_command_stderr_eq_other!",
                stringify!($a_command),
                stringify!($b_command),
                $a_command.get_program(),
                $b_command.get_program(),
                a_output,
                b_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            let b_string = String::from_utf8(b_output.unwrap().stderr).unwrap();
            if a_string == b_string {
                Ok(())
            } else {
                Err(msg_with_left_command_and_right_command!(
                    "assertion failed",
                    "assert_command_stderr_eq_other!",
                    stringify!($a_command),
                    stringify!($b_command),
                    $a_command.get_program(),
                    $b_command.get_program(),
                    a_string,
                    b_string
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    use std::process::Command;

    #[test]
    fn test_assert_command_stderr_eq_other_as_result_x_arity_2_success() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        let x = assert_command_stderr_eq_other_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_eq_other_as_result_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        b.arg("-v");
        let x = assert_command_stderr_eq_other_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_eq_other!(left_command, right_command)`\n",
            "  left command name: `a`,\n",
            " right command name: `b`,\n",
            "       left command: `\"printf\"`,\n",
            "      right command: `\"printf\"`,\n",
            "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "              right: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is equal to another.
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
/// let mut b = Command::new("printf");
/// assert_command_stderr_eq_other!(a, b);
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// let mut b = Command::new("printf");
/// b.arg("-v");
/// assert_command_stderr_eq_other!(a, b);
/// //-> panic!("…")
/// # });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_eq_other!(left_command, right_command)`\n",
///     "  left command name: `a`,\n",
///     " right command name: `b`,\n",
///     "       left command: `\"printf\"`,\n",
///     "      right command: `\"printf\"`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stderr_eq_other {
    ($a_command:expr, $b_command:expr $(,)?) => ({
        match assert_command_stderr_eq_other_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_command:expr, $($arg:tt)+) => ({
        match assert_command_stderr_eq_other_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    use std::process::Command;

    #[test]
    fn test_assert_command_stderr_eq_other_x_arity_2_success() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        let x = assert_command_stderr_eq_other!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stderr_eq_other!(left_command, right_command)`\n  left command name: `a`,\n right command name: `b`,\n       left command: `\"printf\"`,\n      right command: `\"printf\"`,\n               left: `\"printf: illegal option -- v\\nusage: printf format [arguments ...]\\n\"`,\n              right: `\"usage: printf format [arguments ...]\\n\"`")]
    fn test_assert_command_stderr_eq_other_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.arg("-v");
        let mut b = Command::new("printf");
        let _x = assert_command_stderr_eq_other!(a, b);
    }

    #[test]
    fn test_assert_command_stderr_eq_other_x_arity_3_success() {
        let mut a = Command::new("printf");
        let mut b = Command::new("printf");
        let x = assert_command_stderr_eq_other!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_command_stderr_eq_other_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.arg("-v");
        let mut b = Command::new("printf");
        let _x = assert_command_stderr_eq_other!(a, b, "message");
    }

}
