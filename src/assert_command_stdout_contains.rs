/// Assert a command stdout string contains a given containee.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_command_stdout_contains`]
/// * [`assert_command_stdout_contains_as_result`]
/// * [`debug_assert_command_stdout_contains`]
///
#[macro_export]
macro_rules! assert_command_stdout_contains_as_result {
    ($a_command:expr, $b_containee:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_contains!(left_command, right_containee)`\n",
                    "    left_command label: `{}`,\n",
                    "    left_command debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "           left output: `{:?}`"
                ),
                stringify!($a_command), $a_command,
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
                        "assertion failed: `assert_command_stdout_contains!(left_command, right_containee)`\n",
                        "    left_command label: `{}`,\n",
                        "    left_command debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`"
                    ),
                    stringify!($a_command), $a_command,
                    stringify!($b_containee), $b_containee,
                    a_string,
                    $b_containee
                ))
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_asserterable_command_stdout_contains_x_success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = "ell";
        let x = assert_command_stdout_contains_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_asserterable_command_stdout_contains_x_failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = "zzz";
        let x = assert_command_stdout_contains_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_contains!(left_command, right_containee)`\n",
            "    left_command label: `a`,\n",
            "    left_command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
            " right_containee label: `b`,\n",
            " right_containee debug: `\"zzz\"`,\n",
            "                  left: `\"hello\"`,\n",
            "                 right: `\"zzz\"`"
        );
        assert_eq!(actual, expect);
    }

}

/// Assert a command stdout string contains a given containee.
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
/// use std::process::Command;
///
/// # fn main() {
/// // Return Ok
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let containee = "ell";
/// assert_command_stdout_contains!(command, containee);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let containee = "zzz";
/// assert_command_stdout_contains!(command, containee);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_contains!(left_command, right_containee)`\n",
///     "    left_command label: `command`,\n",
///     "    left_command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"zzz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"zzz\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let containee = "zzz";
/// assert_command_stdout_contains!(command, containee, "message");
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
/// * [`assert_command_stdout_contains`]
/// * [`assert_command_stdout_contains_as_result`]
/// * [`debug_assert_command_stdout_contains`]
///
#[macro_export]
macro_rules! assert_command_stdout_contains {
    ($a_command:expr, $b:expr $(,)?) => ({
        match assert_command_stdout_contains_as_result!($a_command, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b:expr, $($message:tt)+) => ({
        match assert_command_stdout_contains_as_result!($a_command, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command stdout string contains a given containee.
///
/// This macro provides the same statements as [`assert_command_stdout_contains`],
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
/// * [`assert_command_stdout_contains`]
/// * [`assert_command_stdout_contains`]
/// * [`debug_assert_command_stdout_contains`]
///
#[macro_export]
macro_rules! debug_assert_command_stdout_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_contains!($($arg)*);
        }
    };
}
