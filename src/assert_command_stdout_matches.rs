/// Assert a command stdout string is a match to a given regex.
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
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"el").unwrap();
/// let x = assert_command_stdout_matches_as_result!(command, matcher);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"xyz").unwrap();
/// let x = assert_command_stdout_matches_as_result!(command, matcher);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_matches!(left_command, right_matcher)`\n",
///     "  left_command label: `command`,\n",
///     "  left_command debug: `\"printf\" \"%s\" \"hello\"`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `xyz`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stdout_matches_as_result {
    ($a_command:expr, $b_matcher:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_matches!(left_command, right_matcher)`\n",
                    "  left_command label: `{}`,\n",
                    "  left_command debug: `{:?}`,\n",
                    " right_matcher label: `{}`,\n",
                    " right_matcher debug: `{:?}`,\n",
                    "         left output: `{:?}`"
                ),
                stringify!($a_command), $a_command,
                stringify!($b_matcher), $b_matcher,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            if $b_matcher.is_match(&a_string) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_command_stdout_matches!(left_command, right_matcher)`\n",
                        "  left_command label: `{}`,\n",
                        "  left_command debug: `{:?}`,\n",
                        " right_matcher label: `{}`,\n",
                        " right_matcher debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($a_command), $a_command,
                    stringify!($b_matcher), $b_matcher,
                    a_string,
                    $b_matcher
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
    fn test_assert_command_stdout_matches_as_result_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = Regex::new(r"lph").unwrap();
        let x = assert_command_stdout_matches_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_matches_as_result_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = Regex::new(r"xyz").unwrap();
        let x = assert_command_stdout_matches_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_matches!(left_command, right_matcher)`\n",
            "  left_command label: `a`,\n",
            "  left_command debug: `\"printf\" \"%s\" \"alpha\"`,\n",
            " right_matcher label: `b`,\n",
            " right_matcher debug: `xyz`,\n",
            "                left: `\"alpha\"`,\n",
            "               right: `xyz`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stdout string is a match to a given regex.
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
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"el").unwrap();
/// assert_command_stdout_matches!(command, matcher);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"xyz").unwrap();
/// assert_command_stdout_matches!(command, matcher);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_matches!(left_command, right_matcher)`\n",
///     "  left_command label: `command`,\n",
///     "  left_command debug: `\"printf\" \"%s\" \"hello\"`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `xyz`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stdout_matches {
    ($a_command:expr, $b_matcher:expr $(,)?) => ({
        match assert_command_stdout_matches_as_result!($a_command, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_matcher:expr, $($arg:tt)+) => ({
        match assert_command_stdout_matches_as_result!($a_command, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
