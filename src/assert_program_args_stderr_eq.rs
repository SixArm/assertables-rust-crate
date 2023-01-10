/// Assert a command (built with program and args) stderr string is equal to an expression.
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
///
/// # fn main() {
/// // Return Ok
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let s = "usage: printf format [arguments ...]\n";
/// let x = assert_program_args_stderr_eq_as_result!(&program, &args, s);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let s = "hello";
/// let x = assert_program_args_stderr_eq_as_result!(&program, &args, s);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_eq!(left_program, left_args, right_expr)`\n",
///     " left_program label: `&program`,\n",
///     " left_program debug: `\"printf\"`,\n",
///     "    left_args label: `&args`,\n",
///     "    left_args debug: `[]`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"hello\"`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `\"hello\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_program_args_stderr_eq`]
/// * [`assert_program_args_stderr_eq_as_result`]
/// * [`debug_assert_program_args_stderr_eq`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_eq_as_result {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stderr_eq!(left_program, left_args, right_expr)`\n",
                    " left_program label: `{}`,\n",
                    " left_program debug: `{:?}`,\n",
                    "    left_args label: `{}`,\n",
                    "    left_args debug: `{:?}`,\n",
                    "   right_expr label: `{}`,\n",
                    "   right_expr debug: `{:?}`,\n",
                    "        left output: `{:?}`"
                ),
                stringify!($a_program), $a_program,
                stringify!($a_args), $a_args,
                stringify!($b_expr), $b_expr,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if a_string == $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stderr_eq!(left_program, left_args, right_expr)`\n",
                        " left_program label: `{}`,\n",
                        " left_program debug: `{:?}`,\n",
                        "    left_args label: `{}`,\n",
                        "    left_args debug: `{:?}`,\n",
                        "   right_expr label: `{}`,\n",
                        "   right_expr debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($a_program), $a_program,
                    stringify!($a_args), $a_args,
                    stringify!($b_expr), $b_expr,
                    a_string,
                    $b_expr
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_program_args_stderr_eq_as_result_x_success() {
        let a_program = "printf";
        let a_args: [&str; 0] = [];
        let b = "usage: printf format [arguments ...]\n";
        let x = assert_program_args_stderr_eq_as_result!(&a_program, &a_args, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stderr_eq_as_result_x_failure() {
        let a_program = "printf";
        let a_args: [&str; 0] = [];
        let b = "hello";
        let x = assert_program_args_stderr_eq_as_result!(&a_program, &a_args, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_eq!(left_program, left_args, right_expr)`\n",
            " left_program label: `&a_program`,\n",
            " left_program debug: `\"printf\"`,\n",
            "    left_args label: `&a_args`,\n",
            "    left_args debug: `[]`,\n",
            "   right_expr label: `b`,\n",
            "   right_expr debug: `\"hello\"`,\n",
            "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "              right: `\"hello\"`");
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string is equal to an expression.
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
/// // Return Ok
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let s = "usage: printf format [arguments ...]\n";
/// assert_program_args_stderr_eq!(&program, &args, s);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let s = "hello";
/// assert_program_args_stderr_eq!(&program, &args, s);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_eq!(left_program, left_args, right_expr)`\n",
///     " left_program label: `&program`,\n",
///     " left_program debug: `\"printf\"`,\n",
///     "    left_args label: `&args`,\n",
///     "    left_args debug: `[]`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"hello\"`,\n",
///     "               left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "              right: `\"hello\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_program_args_stderr_eq`]
/// * [`assert_program_args_stderr_eq_as_result`]
/// * [`debug_assert_program_args_stderr_eq`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_eq {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => ({
        match assert_program_args_stderr_eq_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_program_args_stderr_eq_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
