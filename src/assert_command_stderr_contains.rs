/// Assert a command stderr string contains a given containee.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This uses [`std::String`] method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::process::Command;
///
/// # fn main() {
/// // Return Ok
/// let mut command = Command::new("printf");
/// let containee = "usage";
/// let x = assert_command_stderr_contains_as_result!(command, containee);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// let containee = "xyz";
/// let x = assert_command_stderr_contains_as_result!(command, containee);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_contains!(left_command, right_containee)`\n",
///     "    left_command label: `command`,\n",
///     "    left_command debug: `\"printf\"`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_contains_as_result {
    ($a_command:expr, $b_containee:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stderr_contains!(left_command, right_containee)`\n",
                    "    left_command label: `{}`,\n",
                    "    left_command debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "           left output: `{:?}`"
                ),
                stringify!($a_command), $a_command,
                stringify!($b_containee), $b_containee,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_command_stderr_contains!(left_command, right_containee)`\n",
                        "    left_command label: `{}`,\n",
                        "    left_command debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`"
                    ),
                    stringify!($a_command), $a_command,
                    stringify!($b_containee), $b_containee,
                    a_string,
                    $b_containee
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    use std::process::Command;

    #[test]
    fn test_asserterable_command_stderr_contains_x_success() {
        let mut a = Command::new("printf");
        let b = "usage";
        let x = assert_command_stderr_contains_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stderr_contains_x_failure() {
        let mut a = Command::new("printf");
        let b = "xyz";
        let x = assert_command_stderr_contains_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_contains!(left_command, right_containee)`\n",
            "    left_command label: `a`,\n",
            "    left_command debug: `\"printf\"`,\n",
            " right_containee label: `b`,\n",
            " right_containee debug: `\"xyz\"`,\n",
            "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "                 right: `\"xyz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string contains a given containee.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`std::String`] method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// use std::process::Command;
///
/// # fn main() {
/// // Return Ok
/// let mut command = Command::new("printf");
/// let containee = "usage";
/// assert_command_stderr_contains!(command, containee);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// let containee = "xyz";
/// assert_command_stderr_contains!(command, containee);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stderr_contains!(left_command, right_containee)`\n",
///     "    left_command label: `command`,\n",
///     "    left_command debug: `\"printf\"`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// let containee = "xyz";
/// assert_command_stderr_contains!(command, containee, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_contains {
    ($a_command:expr, $b:expr $(,)?) => ({
        match assert_command_stderr_contains_as_result!($a_command, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b:expr, $($message:tt)+) => ({
        match assert_command_stderr_contains_as_result!($a_command, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
