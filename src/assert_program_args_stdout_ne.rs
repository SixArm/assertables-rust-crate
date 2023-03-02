/// Assert a command (built with program and args) stdout string is not equal to another.
///
/// * If true, return `()`.
///
/// * If true, return Result `Err` with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_program_args_stdout_ne`]
/// * [`assert_program_args_stdout_ne_as_result`]
/// * [`debug_assert_program_args_stdout_ne`]
///
#[macro_export]
macro_rules! assert_program_args_stdout_ne_as_result {
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
                    "assertion failed: `assert_program_args_stdout_ne!(left_program, left_args, right_program, right_args)`\n",
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
            if a_string != b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stdout_ne!(left_program, left_args, right_program, right_args)`\n",
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
mod tests {

    #[test]
    fn test_assert_program_args_stdout_ne_as_result_x_success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s%s", "z", "z", "z"];
        let x = assert_program_args_stdout_ne_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stdout_ne_as_result_x_failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s%s%s%s", "h", "e", "l", "l", "o"];
        let x = assert_program_args_stdout_ne_as_result!(&a_program, &a_args, &b_program, &b_args);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_ne!(left_program, left_args, right_program, right_args)`\n",
            "  left_program label: `&a_program`,\n",
            "  left_program debug: `\"bin/printf-stdout\"`,\n",
            "     left_args label: `&a_args`,\n",
            "     left_args debug: `[\"%s\", \"hello\"]`,\n",
            " right_program label: `&b_program`,\n",
            " right_program debug: `\"bin/printf-stdout\"`,\n",
            "    right_args label: `&b_args`,\n",
            "    right_args debug: `[\"%s%s%s%s%s\", \"h\", \"e\", \"l\", \"l\", \"o\"]`,\n",
            "                left: `\"hello\"`,\n",
            "               right: `\"hello\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is not equal to another.
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
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s%s", "z", "z", "z"];
/// assert_program_args_stdout_ne!(&a_program, &a_args, &b_program, &b_args);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s%s%s%s", "h", "e", "l", "l", "o"];
/// assert_program_args_stdout_ne!(&a_program, &a_args, &b_program, &b_args);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stdout_ne!(left_program, left_args, right_program, right_args)`\n",
///     "  left_program label: `&a_program`,\n",
///     "  left_program debug: `\"bin/printf-stdout\"`,\n",
///     "     left_args label: `&a_args`,\n",
///     "     left_args debug: `[\"%s\", \"hello\"]`,\n",
///     " right_program label: `&b_program`,\n",
///     " right_program debug: `\"bin/printf-stdout\"`,\n",
///     "    right_args label: `&b_args`,\n",
///     "    right_args debug: `[\"%s%s%s%s%s\", \"h\", \"e\", \"l\", \"l\", \"o\"]`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `\"hello\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_program_args_stdout_ne`]
/// * [`assert_program_args_stdout_ne_as_result`]
/// * [`debug_assert_program_args_stdout_ne`]
///
#[macro_export]
macro_rules! assert_program_args_stdout_ne {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => ({
        match assert_program_args_stdout_ne_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_program:expr, $($message:tt)+) => ({
        match assert_program_args_stdout_ne_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command (built with program and args) stdout string is not equal to another.
///
/// This macro provides the same statements as [`assert_program_args_stdout_ne`],
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
/// * [`assert_program_args_stdout_ne`]
/// * [`assert_program_args_stdout_ne`]
/// * [`debug_assert_program_args_stdout_ne`]
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_ne!($($arg)*);
        }
    };
}
