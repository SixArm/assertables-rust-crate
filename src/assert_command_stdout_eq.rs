/// Assert a command stdout string is equal to another.
///
/// * If true, return `()`.
///
/// * Otherwise, return Result `Err` with a message and the values of the
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
/// * [`assert_command_stdout_eq`]
/// * [`assert_command_stdout_eq_as_result`]
/// * [`debug_assert_command_stdout_eq`]
///
#[macro_export]
macro_rules! assert_command_stdout_eq_as_result {
    ($a_command:expr, $b_command:expr $(,)?) => ({
        let a_output = $a_command.output();
        let b_output = $b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n",
                    "  left_command label: `{}`,\n",
                    "  left_command debug: `{:?}`,\n",
                    " right_command label: `{}`,\n",
                    " right_command debug: `{:?}`,\n",
                    "         left output: `{:?}`,\n",
                    "        right output: `{:?}`"
                ),
                stringify!($a_command), $a_command,
                stringify!($b_command), $b_command,
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
                        "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n",
                        "  left_command label: `{}`,\n",
                        "  left_command debug: `{:?}`,\n",
                        " right_command label: `{}`,\n",
                        " right_command debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`"
                    ),
                    stringify!($a_command), $a_command,
                    stringify!($b_command), $b_command,
                    a_string,
                    b_string
                ))
            }
        }
    });
}

#[cfg(test)]
mod assert_tests_as_result {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "a", "l", "p", "h", "a"]);
        let x = assert_command_stdout_eq_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let mut b = Command::new("printf");
        b.args(["%s%s%s%s%s", "b", "r", "a", "v", "o"]);
        let x = assert_command_stdout_eq_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n",
            "  left_command label: `a`,\n",
            "  left_command debug: `\"printf\" \"%s\" \"alpha\"`,\n",
            " right_command label: `b`,\n",
            " right_command debug: `\"printf\" \"%s%s%s%s%s\" \"b\" \"r\" \"a\" \"v\" \"o\"`,\n",
            "                left: `\"alpha\"`,\n",
            "               right: `\"bravo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stdout string is equal to another.
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
///
/// # fn main() {
/// // Return Ok
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
/// assert_command_stdout_eq!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
/// assert_command_stdout_eq!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_eq!(left_command, right_command)`\n",
///     "  left_command label: `a`,\n",
///     "  left_command debug: `\"printf\" \"%s\" \"hello\"`,\n",
///     " right_command label: `b`,\n",
///     " right_command debug: `\"printf\" \"%s%s%s%s%s\" \"w\" \"o\" \"r\" \"l\" \"d\"`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `\"world\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("printf");
/// b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
/// assert_command_stdout_eq!(a, b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_command_stdout_eq`]
/// * [`assert_command_stdout_eq_as_result`]
/// * [`debug_assert_command_stdout_eq`]
///
#[macro_export]
macro_rules! assert_command_stdout_eq {
    ($a_command:expr, $b_command:expr $(,)?) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_command:expr, $($message:tt)+) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command stdout string is equal to another.
///
/// This macro provides the same statements as [`assert_command_stdout_eq`],
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
/// * [`assert_command_stdout_eq`]
/// * [`assert_command_stdout_eq`]
/// * [`debug_assert_command_stdout_eq`]
///
#[macro_export]
macro_rules! debug_assert_command_stdout_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_eq!($($arg)*);
        }
    };
}
