/// Assert a command (built with program and args) stdout string is a match to a regex.
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
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"el").unwrap();
/// let x = assert_program_args_stdout_matches_as_result!(&program, &args, matcher);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"xyz").unwrap();
/// let x = assert_program_args_stdout_matches_as_result!(&program, &args, matcher);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_matches!(left_program, right_matcher)`\n",
///     "  left_program label: `&program`,\n",
///     "  left_program debug: `\"printf\"`,\n",
///     "     left_args label: `&args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
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
macro_rules! assert_program_args_stdout_matches_as_result {
    ($a_program:expr, $a_args:expr, $b_matcher:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stdout_matches!(left_program, right_matcher)`\n",
                    "  left_program label: `{}`,\n",
                    "  left_program debug: `{:?}`,\n",
                    "     left_args label: `{}`,\n",
                    "     left_args debug: `{:?}`,\n",
                    " right_matcher label: `{}`,\n",
                    " right_matcher debug: `{:?}`,\n",
                    "         left output: `{:?}`"
                ),
                stringify!($a_program), $a_program,
                stringify!($a_args), $a_args,
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
                        "assertion failed: `assert_program_args_stdout_matches!(left_program, right_matcher)`\n",
                        "  left_program label: `{}`,\n",
                        "  left_program debug: `{:?}`,\n",
                        "     left_args label: `{}`,\n",
                        "     left_args debug: `{:?}`,\n",
                        " right_matcher label: `{}`,\n",
                        " right_matcher debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($a_program), $a_program,
                    stringify!($a_args), $a_args,
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

    use regex::Regex;

    #[test]
    fn test_assert_program_args_stdout_matches_as_result_x_success() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b = Regex::new(r"lph").unwrap();
        let x = assert_program_args_stdout_matches_as_result!(&a_program, &a_args, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stdout_matches_as_result_x_failure() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b = Regex::new(r"xyz").unwrap();
        let x = assert_program_args_stdout_matches_as_result!(&a_program, &a_args, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_matches!(left_program, right_matcher)`\n",
            "  left_program label: `&a_program`,\n",
            "  left_program debug: `\"printf\"`,\n",
            "     left_args label: `&a_args`,\n",
            "     left_args debug: `[\"%s\", \"alpha\"]`,\n",
            " right_matcher label: `b`,\n",
            " right_matcher debug: `xyz`,\n",
            "                left: `\"alpha\"`,\n",
            "               right: `xyz`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is a match to a regex.
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
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"el").unwrap();
/// assert_program_args_stdout_matches!(&program, &args, matcher);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"xyz").unwrap();
/// assert_program_args_stdout_matches!(&program, &args, matcher);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_matches!(left_program, right_matcher)`\n",
///     "  left_program label: `&program`,\n",
///     "  left_program debug: `\"printf\"`,\n",
///     "     left_args label: `&args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
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
macro_rules! assert_program_args_stdout_matches {
    ($a_program:expr, $a_args:expr, $b_matcher:expr $(,)?) => ({
        match assert_program_args_stdout_matches_as_result!($a_program, $a_args, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_matcher:expr, $($message:tt)+) => ({
        match assert_program_args_stdout_matches_as_result!($a_program, $a_args, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
