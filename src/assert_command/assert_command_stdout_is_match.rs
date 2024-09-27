//! Assert command stdout string is a match to a regex.
//!
//! Pseudocode:<br>
//! (command ⇒ stdout ⇒ string) is match (expr into string)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::process::Command;
//! use regex::Regex;
//!
//! # fn main() {
//! let mut command = Command::new("bin/printf-stdout");
//! command.args(["%s", "hello"]);
//! let matcher = Regex::new(r"ell").unwrap();
//! assert_command_stdout_is_match!(command, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stdout_is_match`](macro@crate::assert_command_stdout_is_match)
//! * [`assert_command_stdout_is_match_as_result`](macro@crate::assert_command_stdout_is_match_as_result)
//! * [`debug_assert_command_stdout_is_match`](macro@crate::debug_assert_command_stdout_is_match)

/// Assert command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_is_match`](macro@crate::assert_command_stdout_is_match)
/// * [`assert_command_stdout_is_match_as_result`](macro@crate::assert_command_stdout_is_match_as_result)
/// * [`debug_assert_command_stdout_is_match`](macro@crate::debug_assert_command_stdout_is_match)
///
#[macro_export]
macro_rules! assert_command_stdout_is_match_as_result {
    ($command:expr, $matcher:expr $(,)?) => {{
        match (/*&$command,*/ &$matcher) {
            matcher => {
                let output = $command.output();
                if output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_command_stdout_is_match!(command, matcher)`\n",
                            "https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_is_match.html\n",
                            " command label: `{}`,\n",
                            " command debug: `{:?}`,\n",
                            " matcher label: `{}`,\n",
                            " matcher debug: `{:?}`,\n",
                            " command output: `{:?}`"
                        ),
                        stringify!($command),
                        $command,
                        stringify!($matcher),
                        matcher,
                        output
                    ))
                } else {
                    let string = String::from_utf8(output.unwrap().stdout).unwrap();
                    if $matcher.is_match(&string) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_command_stdout_is_match!(command, matcher)`\n",
                                "https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_is_match.html\n",
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
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use regex::Regex;
    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_is_match_as_result_x_success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = Regex::new(r"ell").unwrap();
        let result = assert_command_stdout_is_match_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_is_match_as_result_x_failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = Regex::new(r"zzz").unwrap();
        let result = assert_command_stdout_is_match_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_is_match!(command, matcher)`\n",
            "https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_is_match.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
            " matcher label: `b`,\n",
            " matcher debug: `Regex(\"zzz\")`,\n",
            " command value: `\"hello\"`,\n",
            " matcher value: `Regex(\"zzz\")`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
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
/// use regex::Regex;
///
/// # fn main() {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_command_stdout_is_match!(command, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let matcher = Regex::new(r"zzz").unwrap();
/// assert_command_stdout_is_match!(command, &matcher);
/// # });
/// // assertion failed: `assert_command_stdout_is_match!(command, matcher)`
/// // https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_is_match.html
/// //  command label: `command`,
/// //  command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zzz\")`,
/// //  command value: `\"hello\"`,
/// //  matcher value: `Regex(\"zzz\")`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stdout_is_match!(command, matcher)`\n",
/// #     "https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_is_match.html\n",
/// #     " command label: `command`,\n",
/// #     " command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zzz\")`,\n",
/// #     " command value: `\"hello\"`,\n",
/// #     " matcher value: `Regex(\"zzz\")`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_is_match`](macro@crate::assert_command_stdout_is_match)
/// * [`assert_command_stdout_is_match_as_result`](macro@crate::assert_command_stdout_is_match_as_result)
/// * [`debug_assert_command_stdout_is_match`](macro@crate::debug_assert_command_stdout_is_match)
///
#[macro_export]
macro_rules! assert_command_stdout_is_match {
    ($command:expr, $matcher:expr $(,)?) => {{
        match $crate::assert_command_stdout_is_match_as_result!($command, $matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($command:expr, $matcher:expr, $($message:tt)+) => {{
        match $crate::assert_command_stdout_is_match_as_result!($command, $matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert command stdout string is a match to a regex.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) is match (expr into string)
///
/// This macro provides the same statements as [`assert_command_stdout_is_match`](macro.assert_command_stdout_is_match.html),
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
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_command_stdout_is_match`](macro@crate::assert_command_stdout_is_match)
/// * [`assert_command_stdout_is_match`](macro@crate::assert_command_stdout_is_match)
/// * [`debug_assert_command_stdout_is_match`](macro@crate::debug_assert_command_stdout_is_match)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_is_match!($($arg)*);
        }
    };
}
