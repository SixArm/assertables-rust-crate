/// Assert a command (built with program and args) stdout string is equal to another.
///
/// * If true, return `()`.
///
/// * If true, return Result `Err` with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
///
/// # fn main() {
/// let a_program = "printf";
/// let a_args = ["%s", "hello"];
/// let b_program = "printf";
/// let b_args = ["%s%s%s%s%s", "h", "e", "l", "l", "o"];
/// let x = assert_program_args_stdout_eq_other_as_result!(&a_program, &a_args, &b_program, &b_args);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a_program = "printf";
/// let a_args = ["%s", "hello"];
/// let b_program = "printf";
/// let b_args = ["%s%s%s%s%s", "w", "o", "r", "l", "d"];
/// let x = assert_program_args_stdout_eq_other_as_result!(&a_program, &a_args, &b_program, &b_args);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_eq_other!(left_program, left_args, right_program, right_args)`\n",
///     "  left_program label: `&a_program`,\n",
///     "  left_program debug: `\"printf\"`,\n",
///     "     left_args label: `&a_args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_program label: `&b_program`,\n",
///     " right_program debug: `\"printf\"`,\n",
///     "    right_args label: `&b_args`,\n",
///     "    right_args debug: `[\"%s%s%s%s%s\", \"w\", \"o\", \"r\", \"l\", \"d\"]`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `\"world\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_program_args_stdout_eq_other_as_result {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        let mut b_command = ::std::process::Command::new($b_program);
        a_command.args($a_args);
        b_command.args($b_args);
        let a_output = a_command.output();
        let b_output = b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stdout_eq_other!(left_program, left_args, right_program, right_args)`\n",
                    "  left_program label: `{}`,\n",
                    "  left_program debug: `{:?}`,\n",
                    "     left_args label: `{}`,\n",
                    "     left_args debug: `{:?}`,\n",
                    " right_program label: `{}`,\n",
                    " right_program debug: `{:?}`,\n",
                    "    right_args label: `{}`,\n",
                    "    right_args debug: `{:?}`,\n",
                    "         left output: `{:?}`,\n",
                    "        right output: `{:?}`"
                ),
                stringify!($a_program), $a_program,
                stringify!($a_args), $a_args,
                stringify!($b_program), $b_program,
                stringify!($b_args), $b_args,
                a_output,
                b_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            let b_string = String::from_utf8(b_output.unwrap().stdout).unwrap();
            if a_string == b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stdout_eq_other!(left_program, left_args, right_program, right_args)`\n",
                    "  left_program label: `{}`,\n",
                    "  left_program debug: `{:?}`,\n",
                        "     left_args label: `{}`,\n",
                        "     left_args debug: `{:?}`,\n",
                        " right_program label: `{}`,\n",
                        " right_program debug: `{:?}`,\n",
                        "    right_args label: `{}`,\n",
                        "    right_args debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($a_program), $a_program,
                    stringify!($a_args), $a_args,
                    stringify!($b_program), $b_program,
                    stringify!($b_args), $b_args,
                    a_string,
                    b_string
                ))
            }
        }
    });
}

#[cfg(test)]
mod assert_tests_as_result {

    #[test]
    fn test_assert_program_args_stdout_eq_other_as_result_x_arity_2_success() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b_program = "printf";
        let b_args = ["%s%s%s%s%s", "a", "l", "p", "h", "a"];
        let x = assert_program_args_stdout_eq_other_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stdout_eq_other_as_result_x_arity_2_failure() {
        let a_program = "printf";
        let a_args = ["%s", "alpha"];
        let b_program = "printf";
        let b_args = ["%s%s%s%s%s", "b", "r", "a", "v", "o"];
        let x = assert_program_args_stdout_eq_other_as_result!(&a_program, &a_args, &b_program, &b_args);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_eq_other!(left_program, left_args, right_program, right_args)`\n",
            "  left_program label: `&a_program`,\n",
            "  left_program debug: `\"printf\"`,\n",
            "     left_args label: `&a_args`,\n",
            "     left_args debug: `[\"%s\", \"alpha\"]`,\n",
            " right_program label: `&b_program`,\n",
            " right_program debug: `\"printf\"`,\n",
            "    right_args label: `&b_args`,\n",
            "    right_args debug: `[\"%s%s%s%s%s\", \"b\", \"r\", \"a\", \"v\", \"o\"]`,\n",
            "                left: `\"alpha\"`,\n",
            "               right: `\"bravo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is equal to another.
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
///
/// # fn main() {
/// let a_program = "printf";
/// let a_args = ["%s", "hello"];
/// let b_program = "printf";
/// let b_args = ["%s%s%s%s%s", "h", "e", "l", "l", "o"];
/// assert_program_args_stdout_eq_other!(&a_program, &a_args, &b_program, &b_args);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a_program = "printf";
/// let a_args = ["%s", "hello"];
/// let b_program = "printf";
/// let b_args = ["%s%s%s%s%s", "w", "o", "r", "l", "d"];
/// assert_program_args_stdout_eq_other!(&a_program, &a_args, &b_program, &b_args);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_eq_other!(left_program, left_args, right_program, right_args)`\n",
///     "  left_program label: `&a_program`,\n",
///     "  left_program debug: `\"printf\"`,\n",
///     "     left_args label: `&a_args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_program label: `&b_program`,\n",
///     " right_program debug: `\"printf\"`,\n",
///     "    right_args label: `&b_args`,\n",
///     "    right_args debug: `[\"%s%s%s%s%s\", \"w\", \"o\", \"r\", \"l\", \"d\"]`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `\"world\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_program_args_stdout_eq_other {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => ({
        match assert_program_args_stdout_eq_other_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_program:expr, $($arg:tt)+) => ({
        match assert_program_args_stdout_eq_other_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
