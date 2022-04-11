/// Assert a command stderr string is a match to a given regex.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
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
///     "assertion failed: `assert_command_stderr_matches!(left_command, right_matcher)`\n",
///     " left_command label: `command`,\n",
///     " left_command debug: `\"printf\"`,\n",
///     "  right_matcher label: `matcher`,\n",
///     "  right_matcher debug: `xyz`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_matches_as_result {
    ($command:expr, $b:expr $(,)?) => ({
        let a_output = $command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stderr_matches!(left_command, right_matcher)`\n",
                    " left_command label: `{}`,\n",
                    " left_command debug: `{:?}`,\n",
                    "  right_matcher label: `{}`,\n",
                    "  right_matcher debug: `{:?}`,\n",
                    "        left output: `{:?}`"
                ),
                stringify!($command), $command,
                stringify!($b), $b,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if $b.is_match(&a_string) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_command_stderr_matches!(left_command, right_matcher)`\n",
                        " left_command label: `{}`,\n",
                        " left_command debug: `{:?}`,\n",
                        "  right_matcher label: `{}`,\n",
                        "  right_matcher debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($command), $command,
                    stringify!($b), $b,
                    a_string,
                    $b
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
        let b = Regex::new(r"usage").unwrap();
        let x = assert_command_stderr_matches_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_matches_as_result_x_arity_2_failure() {
        let mut a = Command::new("printf");
        let b = Regex::new(r"xyz").unwrap();
        let x = assert_command_stderr_matches_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_matches!(left_command, right_matcher)`\n",
            " left_command label: `a`,\n",
            " left_command debug: `\"printf\"`,\n",
            "  right_matcher label: `b`,\n",
            "  right_matcher debug: `xyz`,\n",
            "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "              right: `xyz`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is a match to a given regex.
///
/// * If true, return `()`.
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
///     "assertion failed: `assert_command_stderr_matches!(left_command, right_matcher)`\n",
///     " left_command label: `command`,\n",
///     " left_command debug: `\"printf\"`,\n",
///     "  right_matcher label: `matcher`,\n",
///     "  right_matcher debug: `xyz`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stderr_matches {
    ($command:expr, $b:expr $(,)?) => ({
        match assert_command_stderr_matches_as_result!($command, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($command:expr, $b:expr, $($arg:tt)+) => ({
        match assert_command_stderr_matches_as_result!($command, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
