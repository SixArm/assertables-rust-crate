/// Assert a command (built with program and args) stderr string contains a given containee.
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
/// // Return Ok
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let containee = "usage";
/// let x = assert_program_args_stderr_contains_as_result!(&program, &args, containee);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let containee = "xyz";
/// let x = assert_program_args_stderr_contains_as_result!(&program, &args, containee);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_contains!(left_program, left_args, right_containee)`\n",
///     "    left_program label: `&program`,\n",
///     "    left_program debug: `\"printf\"`,\n",
///     "       left_args label: `&args`,\n",
///     "       left_args debug: `[]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_program_args_stderr_contains`]
/// * [`assert_program_args_stderr_contains_as_result`]
/// * [`debug_assert_program_args_stderr_contains`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_contains_as_result {
    ($a_program:expr, $a_args:expr, $b_containee:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stderr_contains!(left_program, left_args, right_containee)`\n",
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
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stderr_contains!(left_program, left_args, right_containee)`\n",
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
    fn test_asserterable_command_stderr_contains_x_success() {
        let a_program = "printf";
        let a_args: [&str; 0] = [];
        let b = "usage";
        let x = assert_program_args_stderr_contains_as_result!(&a_program, &a_args, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stderr_contains_x_failure() {
        let a_program = "printf";
        let a_args: [&str; 0] = [];
        let b = "xyz";
        let x = assert_program_args_stderr_contains_as_result!(&a_program, &a_args, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_contains!(left_program, left_args, right_containee)`\n",
            "    left_program label: `&a_program`,\n",
            "    left_program debug: `\"printf\"`,\n",
            "       left_args label: `&a_args`,\n",
            "       left_args debug: `[]`,\n",
            " right_containee label: `b`,\n",
            " right_containee debug: `\"xyz\"`,\n",
            "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
            "                 right: `\"xyz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string contains a given containee.
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
/// // Return Ok
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let containee = "usage";
/// assert_program_args_stderr_contains!(&program, &args, containee);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let program = "printf";
/// let args: [&str; 0] = [];
/// let containee = "xyz";
/// assert_program_args_stderr_contains!(&program, &args, containee);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_contains!(left_program, left_args, right_containee)`\n",
///     "    left_program label: `&program`,\n",
///     "    left_program debug: `\"printf\"`,\n",
///     "       left_args label: `&args`,\n",
///     "       left_args debug: `[]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"xyz\"`,\n",
///     "                  left: `\"usage: printf format [arguments ...]\\n\"`,\n",
///     "                 right: `\"xyz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_program_args_stderr_contains`]
/// * [`assert_program_args_stderr_contains_as_result`]
/// * [`debug_assert_program_args_stderr_contains`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_contains {
    ($a_program:expr, $a_args:expr, $b_containee:expr $(,)?) => ({
        match assert_program_args_stderr_contains_as_result!($a_program, $a_args, $b_containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_containee:expr, $($message:tt)+) => ({
        match assert_program_args_stderr_contains_as_result!($a_program, $a_args, $b_containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// This macro provides the same statements as [`assert_program_args_stderr_contains`],
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
/// * [`assert_program_args_stderr_contains`]
/// * [`assert_program_args_stderr_contains`]
/// * [`debug_assert_program_args_stderr_contains`]
/// 
#[macro_export]
macro_rules! debug_assert_program_args_stderr_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_contains!($($arg)*);
        }
    };
}
