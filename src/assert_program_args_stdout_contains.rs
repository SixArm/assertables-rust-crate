/// Assert a command (built with program and args) stdout string contains a given containee.
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
///
/// # fn main() {
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let containee = "ell";
/// let x = assert_program_args_stdout_contains_as_result!(&program, &args, containee);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let containee = "xyz";
/// let x = assert_program_args_stdout_contains_as_result!(&program, &args, containee);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_contains!(left_program, left_args, right_containee)`\n",
///     "    left_program label: `&program`,\n",
///     "    left_program debug: `\"printf\"`,\n",
///     "       left_args label: `&args`,\n",
///     "       left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_program_args_stdout_contains_as_result {
    ($a_program:expr, $a_args:expr, $b_containee:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stdout_contains!(left_program, left_args, right_containee)`\n",
                    "    left_program label: `{}`,\n",
                    "    left_program debug: `{:?}`,\n",
                    "       left_args label: `{}`,\n",
                    "       left_args debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "           left output: `{:?}`"
                ),
                stringify!($a_program), $a_program,
                stringify!($a_args), $a_args,
                stringify!($b_containee), $b_containee,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stdout_contains!(left_program, left_args, right_containee)`\n",
                        "    left_program label: `{}`,\n",
                        "    left_program debug: `{:?}`,\n",
                        "       left_args label: `{}`,\n",
                        "       left_args debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`"
                    ),
                    stringify!($a_program), $a_program,
                    stringify!($a_args), $a_args,
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

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_2_success() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b = "lph";
        let x = assert_program_args_stdout_contains_as_result!(&a_program, &a_args, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_arity_2_failure() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b = "xyz";
        let x = assert_program_args_stdout_contains_as_result!(&a_program, &a_args, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_contains!(left_program, left_args, right_containee)`\n",
            "    left_program label: `&a_program`,\n",
            "    left_program debug: `\"printf\"`,\n",
            "       left_args label: `&a_args`,\n",
            "       left_args debug: `[\"%s\", \"alpha\"]`,\n",
            " right_containee label: `b`,\n",
            " right_containee debug: `\"xyz\"`,\n",
            "                  left: `\"alpha\"`,\n",
            "                 right: `\"xyz\"`"
        );
        assert_eq!(actual, expect);
    }

}

/// Assert a command (built with program and args) stdout string contains a given containee.
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
///
/// # fn main() {
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let containee = "ell";
/// assert_program_args_stdout_contains!(&program, &args, containee);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let program = "printf";
/// let args = ["%s", "hello"];
/// let containee = "xyz";
/// assert_program_args_stdout_contains!(&program, &args, containee);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_contains!(left_program, left_args, right_containee)`\n",
///     "    left_program label: `&program`,\n",
///     "    left_program debug: `\"printf\"`,\n",
///     "       left_args label: `&args`,\n",
///     "       left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_program_args_stdout_contains {
    ($a_program:expr, $a_args:expr, $b_containee:expr $(,)?) => ({
        match assert_program_args_stdout_contains_as_result!($a_program, $a_args, $b_containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_containee:expr, $($arg:tt)+) => ({
        match assert_program_args_stdout_contains_as_result!($a_program, $a_args, $b_containee) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
