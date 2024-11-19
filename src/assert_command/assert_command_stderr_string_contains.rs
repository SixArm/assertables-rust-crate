//! Assert a command stderr string contains a given containee.
//!
//! Pseudocode:<br>
//! (command ⇒ stderr ⇒ string) contains (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut command = Command::new("bin/printf-stderr");
//! command.args(["%s", "alfa"]);
//! let containee = "lf";
//! assert_command_stderr_string_contains!(command, &containee);
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_string_contains`](macro@crate::assert_command_stderr_string_contains)
//! * [`assert_command_stderr_string_contains_as_result`](macro@crate::assert_command_stderr_string_contains_as_result)
//! * [`debug_assert_command_stderr_string_contains`](macro@crate::debug_assert_command_stderr_string_contains)

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// * If true, return Result `Ok(command ⇒ stderr ⇒ string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_command_stderr_string_contains`](macro.assert_command_stderr_string_contains.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stderr_string_contains`](macro@crate::assert_command_stderr_string_contains)
/// * [`assert_command_stderr_string_contains_as_result`](macro@crate::assert_command_stderr_string_contains_as_result)
/// * [`debug_assert_command_stderr_string_contains`](macro@crate::debug_assert_command_stderr_string_contains)
///
#[macro_export]
macro_rules! assert_command_stderr_string_contains_as_result {
    ($command:expr, $containee:expr $(,)?) => {{
        match (/*&$command,*/ &$containee) {
            containee => {
                match $command.output() {
                    Ok(output) => {
                        let string = String::from_utf8(output.stderr).unwrap();
                        if string.contains($containee) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_command_stderr_string_contains!(command, containee)`\n",
                                        "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html\n",
                                        "   command label: `{}`,\n",
                                        "   command debug: `{:?}`,\n",
                                        " containee label: `{}`,\n",
                                        " containee debug: `{:?}`,\n",
                                        "          string: `{:?}`"
                                    ),
                                    stringify!($command),
                                    $command,
                                    stringify!($containee),
                                    containee,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_command_stderr_string_contains!(command, containee)`\n",
                                    "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html\n",
                                    "   command label: `{}`,\n",
                                    "   command debug: `{:?}`,\n",
                                    " containee label: `{}`,\n",
                                    " containee debug: `{:?}`,\n",
                                    "      output err: `{:?}`"
                                ),
                                stringify!($command),
                                $command,
                                stringify!($containee),
                                containee,
                                err
                            )
                        )
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_command_stderr_string_contains_as_result {
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = "lf";
        let actual = assert_command_stderr_string_contains_as_result!(a, b);
        assert_eq!(actual.unwrap(), "alfa");
    }

    #[test]
    fn failure() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = "zz";
        let actual = assert_command_stderr_string_contains_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stderr_string_contains!(command, containee)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            "          string: `\"alfa\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// * If true, return (command ⇒ stderr ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`::std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::process::Command;
///
/// # fn main() {
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "alfa"]);
/// let containee = "lf";
/// assert_command_stderr_string_contains!(command, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "alfa"]);
/// let containee = "zz";
/// assert_command_stderr_string_contains!(command, &containee);
/// # });
/// // assertion failed: `assert_command_stderr_string_contains!(command, containee)`
/// // https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html
/// //    command label: `command`,
/// //    command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zz\"`,
/// //           string: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_command_stderr_string_contains!(command, containee)`\n",
/// #     "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html\n",
/// #     "   command label: `command`,\n",
/// #     "   command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zz\"`,\n",
/// #     "          string: `\"alfa\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_string_contains`](macro@crate::assert_command_stderr_string_contains)
/// * [`assert_command_stderr_string_contains_as_result`](macro@crate::assert_command_stderr_string_contains_as_result)
/// * [`debug_assert_command_stderr_string_contains`](macro@crate::debug_assert_command_stderr_string_contains)
///
#[macro_export]
macro_rules! assert_command_stderr_string_contains {
    ($command:expr, $containee:expr $(,)?) => {{
        match $crate::assert_command_stderr_string_contains_as_result!($command, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($command:expr, $containee:expr, $($message:tt)+) => {{
        match $crate::assert_command_stderr_string_contains_as_result!($command, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_command_stderr_string_contains {
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = "lf";
        let actual = assert_command_stderr_string_contains!(a, b);
        assert_eq!(actual, "alfa");
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stderr");
            a.args(["%s", "alfa"]);
            let b = "zz";
            let _actual = assert_command_stderr_string_contains!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stderr_string_contains!(command, containee)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stderr_string_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            "          string: `\"alfa\"`",
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// This macro provides the same statements as [`assert_command_stderr_string_contains`](macro.assert_command_stderr_string_contains.html),
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
/// This macro is intended to work in a similar way to
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_command_stderr_string_contains`](macro@crate::assert_command_stderr_string_contains)
/// * [`assert_command_stderr_string_contains`](macro@crate::assert_command_stderr_string_contains)
/// * [`debug_assert_command_stderr_string_contains`](macro@crate::debug_assert_command_stderr_string_contains)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_string_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_string_contains!($($arg)*);
        }
    };
}
