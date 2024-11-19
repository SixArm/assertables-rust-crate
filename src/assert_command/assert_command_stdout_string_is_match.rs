//! Assert a command stdout string is a match to a regex.
//!
//! Pseudocode:<br>
//! (command ⇒ stdout ⇒ string) is match (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//! use regex::Regex;
//!
//! let mut command = Command::new("bin/printf-stdout");
//! command.args(["%s", "alfa"]);
//! let matcher = Regex::new(r"lf").expect("regex");
//! assert_command_stdout_string_is_match!(command, &matcher);
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stdout_string_is_match`](macro@crate::assert_command_stdout_string_is_match)
//! * [`assert_command_stdout_string_is_match_as_result`](macro@crate::assert_command_stdout_string_is_match_as_result)
//! * [`debug_assert_command_stdout_string_is_match`](macro@crate::debug_assert_command_stdout_string_is_match)

/// Assert a command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
///
/// * If true, return Result `Ok(command ⇒ stdout ⇒ string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_string_is_match`](macro@crate::assert_command_stdout_string_is_match)
/// * [`assert_command_stdout_string_is_match_as_result`](macro@crate::assert_command_stdout_string_is_match_as_result)
/// * [`debug_assert_command_stdout_string_is_match`](macro@crate::debug_assert_command_stdout_string_is_match)
///
#[macro_export]
macro_rules! assert_command_stdout_string_is_match_as_result {
    ($command:expr, $matcher:expr $(,)?) => {{
        match (/*&$command,*/ &$matcher) {
            matcher => {
                match $command.output() {
                    Ok(output) => {
                        let string = String::from_utf8(output.stdout).unwrap();
                        if $matcher.is_match(&string) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`\n",
                                        "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html\n",
                                        " command label: `{}`,\n",
                                        " command debug: `{:?}`,\n",
                                        " matcher label: `{}`,\n",
                                        " matcher debug: `{:?}`,\n",
                                        " command value: `{:?}`,\n",
                                        " matcher value: `{:?}`"
                                    ),
                                    stringify!($command),
                                    $command,
                                    stringify!($matcher),
                                    matcher,
                                    string,
                                    matcher
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`\n",
                                    "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html\n",
                                    "  command label: `{}`,\n",
                                    "  command debug: `{:?}`,\n",
                                    "  matcher label: `{}`,\n",
                                    "  matcher debug: `{:?}`,\n",
                                    "  output is err: `{:?}`"
                                ),
                                stringify!($command),
                                $command,
                                stringify!($matcher),
                                matcher,
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
mod test_assert_command_stdout_string_is_match_as_result {
    use regex::Regex;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = Regex::new(r"lf").expect("regex");
        let actual = assert_command_stdout_string_is_match_as_result!(a, b);
        assert_eq!(actual.unwrap(), "alfa");
    }

    #[test]
    fn failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = Regex::new(r"zz").expect("regex");
        let actual = assert_command_stdout_string_is_match_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " matcher label: `b`,\n",
            " matcher debug: `Regex(\"zz\")`,\n",
            " command value: `\"alfa\"`,\n",
            " matcher value: `Regex(\"zz\")`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
///
/// * If true, return (command ⇒ stdout ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::process::Command;
/// use regex::Regex;
///
/// # fn main() {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "alfa"]);
/// let matcher = Regex::new(r"lf").expect("regex");
/// assert_command_stdout_string_is_match!(command, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "alfa"]);
/// let matcher = Regex::new(r"zz").expect("regex");
/// assert_command_stdout_string_is_match!(command, &matcher);
/// # });
/// // assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`
/// // https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html
/// //  command label: `command`,
/// //  command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zz\")`,
/// //  command value: `\"alfa\"`,
/// //  matcher value: `Regex(\"zz\")`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`\n",
/// #     "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html\n",
/// #     " command label: `command`,\n",
/// #     " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zz\")`,\n",
/// #     " command value: `\"alfa\"`,\n",
/// #     " matcher value: `Regex(\"zz\")`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_string_is_match`](macro@crate::assert_command_stdout_string_is_match)
/// * [`assert_command_stdout_string_is_match_as_result`](macro@crate::assert_command_stdout_string_is_match_as_result)
/// * [`debug_assert_command_stdout_string_is_match`](macro@crate::debug_assert_command_stdout_string_is_match)
///
#[macro_export]
macro_rules! assert_command_stdout_string_is_match {
    ($command:expr, $matcher:expr $(,)?) => {{
        match $crate::assert_command_stdout_string_is_match_as_result!($command, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($command:expr, $matcher:expr, $($message:tt)+) => {{
        match $crate::assert_command_stdout_string_is_match_as_result!($command, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_command_stdout_string_is_match {
    use regex::Regex;
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = Regex::new(r"lf").expect("regex");
        let actual = assert_command_stdout_string_is_match!(a, b);
        assert_eq!(actual, "alfa");
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = Regex::new(r"zz").expect("regex");
            let _actual = assert_command_stdout_string_is_match!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_string_is_match!(command, matcher)`\n",
            "https://docs.rs/assertables/9.5.0/assertables/macro.assert_command_stdout_string_is_match.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " matcher label: `b`,\n",
            " matcher debug: `Regex(\"zz\")`,\n",
            " command value: `\"alfa\"`,\n",
            " matcher value: `Regex(\"zz\")`"
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

/// Assert a command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
///
/// This macro provides the same statements as [`assert_command_stdout_string_is_match`](macro.assert_command_stdout_string_is_match.html),
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
/// * [`assert_command_stdout_string_is_match`](macro@crate::assert_command_stdout_string_is_match)
/// * [`assert_command_stdout_string_is_match`](macro@crate::assert_command_stdout_string_is_match)
/// * [`debug_assert_command_stdout_string_is_match`](macro@crate::debug_assert_command_stdout_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_string_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_string_is_match!($($arg)*);
        }
    };
}
