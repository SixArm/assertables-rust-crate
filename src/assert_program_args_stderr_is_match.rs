/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_program_args_stderr_is_match`]
/// * [`assert_program_args_stderr_is_match_as_result`]
/// * [`debug_assert_program_args_stderr_is_match`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_is_match_as_result {
    ($a_program:expr, $a_args:expr, $b_matcher:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stderr_is_match!(left_program, right_matcher)`\n",
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
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if $b_matcher.is_match(&a_string) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stderr_is_match!(left_program, right_matcher)`\n",
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
mod tests {

    use regex::Regex;

    #[test]
    fn test_assert_program_args_stderr_is_match_as_result_x_success() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b = Regex::new(r"ell").unwrap();
        let x = assert_program_args_stderr_is_match_as_result!(&a_program, &a_args, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stderr_is_match_as_result_x_failure() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b = Regex::new(r"zzz").unwrap();
        let x = assert_program_args_stderr_is_match_as_result!(&a_program, &a_args, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_is_match!(left_program, right_matcher)`\n",
            "  left_program label: `&a_program`,\n",
            "  left_program debug: `\"bin/printf-stderr\"`,\n",
            "     left_args label: `&a_args`,\n",
            "     left_args debug: `[\"%s\", \"hello\"]`,\n",
            " right_matcher label: `b`,\n",
            " right_matcher debug: `zzz`,\n",
            "                left: `\"hello\"`,\n",
            "               right: `zzz`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
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
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_program_args_stderr_is_match!(&program, &args, matcher);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let matcher = Regex::new(r"zzz").unwrap();
/// assert_program_args_stderr_is_match!(&program, &args, matcher);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_is_match!(left_program, right_matcher)`\n",
///     "  left_program label: `&program`,\n",
///     "  left_program debug: `\"bin/printf-stderr\"`,\n",
///     "     left_args label: `&args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `zzz`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `zzz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_program_args_stderr_is_match`]
/// * [`assert_program_args_stderr_is_match_as_result`]
/// * [`debug_assert_program_args_stderr_is_match`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_is_match {
    ($a_program:expr, $a_args:expr, $b_matcher:expr $(,)?) => ({
        match assert_program_args_stderr_is_match_as_result!($a_program, $a_args, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_matcher:expr, $($message:tt)+) => ({
        match assert_program_args_stderr_is_match_as_result!($a_program, $a_args, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// This macro provides the same statements as [`assert_program_args_stderr_is_match`],
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Related
///
/// * [`assert_program_args_stderr_is_match`]
/// * [`assert_program_args_stderr_is_match`]
/// * [`debug_assert_program_args_stderr_is_match`]
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_is_match!($($arg)*);
        }
    };
}
