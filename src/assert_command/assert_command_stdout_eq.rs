//! Assert a command stdout string is equal to another.
//!
//! Pseudocode:<br>
//! (command1 ⇒ stdout ⇒ string) = (command2 ⇒ stdout ⇒ string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/printf-stdout");
//! a.args(["%s", "hello"]);
//! let mut b = Command::new("bin/printf-stdout");
//! b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
//! assert_command_stdout_eq!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stdout_eq`](macro@crate::assert_command_stdout_eq)
//! * [`assert_command_stdout_eq_as_result`](macro@crate::assert_command_stdout_eq_as_result)
//! * [`debug_assert_command_stdout_eq`](macro@crate::debug_assert_command_stdout_eq)

/// Assert a command stdout string is equal to another.
///
/// Pseudocode:<br>
/// (command1 ⇒ stdout ⇒ string) = (command2 ⇒ stdout ⇒ string)
///
/// * If true, return `()`.
///
/// * Otherwise, return Result `Err` with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_eq`](macro@crate::assert_command_stdout_eq)
/// * [`assert_command_stdout_eq_as_result`](macro@crate::assert_command_stdout_eq_as_result)
/// * [`debug_assert_command_stdout_eq`](macro@crate::debug_assert_command_stdout_eq)
///
#[macro_export]
macro_rules! assert_command_stdout_eq_as_result {
    ($a_command:expr, $b_command:expr $(,)?) => {{
        let a_output = $a_command.output();
        let b_output = $b_command.output();
        if a_output.is_err() || b_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_eq!(a_command, b_command)`\n",
                    "https://docs.rs/assertables/8.11.0/assertables/macro.assert_command_stdout_eq.html\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`,\n",
                    " b label: `{}`,\n",
                    " b debug: `{:?}`,\n",
                    " a output: `{:?}`,\n",
                    " b output: `{:?}`"
                ),
                stringify!($a_command),
                $a_command,
                stringify!($b_command),
                $b_command,
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
                        "assertion failed: `assert_command_stdout_eq!(a_command, b_command)`\n",
                        "https://docs.rs/assertables/8.11.0/assertables/macro.assert_command_stdout_eq.html\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`,\n",
                        " b label: `{}`,\n",
                        " b debug: `{:?}`,\n",
                        "       a: `{:?}`,\n",
                        "       b: `{:?}`"
                    ),
                    stringify!($a_command),
                    $a_command,
                    stringify!($b_command),
                    $b_command,
                    a_string,
                    b_string
                ))
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let mut b = Command::new("bin/printf-stdout");
        b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
        let result = assert_command_stdout_eq_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let mut b = Command::new("bin/printf-stdout");
        b.args(["%s%s%s", "z", "z", "z"]);
        let result = assert_command_stdout_eq_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_eq!(a_command, b_command)`\n",
            "https://docs.rs/assertables/8.11.0/assertables/macro.assert_command_stdout_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stdout\" \"%s%s%s\" \"z\" \"z\" \"z\"`,\n",
            "       a: `\"hello\"`,\n",
            "       b: `\"zzz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stdout string is equal to another.
///
/// Pseudocode:<br>
/// (command1 ⇒ stdout ⇒ string) = (command2 ⇒ stdout ⇒ string)
///
/// * If true, return `()`.
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
///
/// # fn main() {
/// let mut a = Command::new("bin/printf-stdout");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("bin/printf-stdout");
/// b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
/// assert_command_stdout_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("bin/printf-stdout");
/// a.args(["%s", "hello"]);
/// let mut b = Command::new("bin/printf-stdout");
/// b.args(["%s%s%s", "z", "z", "z"]);
/// assert_command_stdout_eq!(a, b);
/// # });
/// // assertion failed: `assert_command_stdout_eq!(a_command, b_command)`
/// // https://docs.rs/assertables/8.11.0/assertables/macro.assert_command_stdout_eq.html
/// //  a label: `a`,
/// //  a debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,
/// //  b label: `b`,
/// //  b debug: `\"bin/printf-stdout\" \"%s%s%s\" \"z\" \"z\" \"z\"`,
/// //        a: `\"hello\"`,
/// //        b: `\"zzz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stdout_eq!(a_command, b_command)`\n",
/// #     "https://docs.rs/assertables/8.11.0/assertables/macro.assert_command_stdout_eq.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"bin/printf-stdout\" \"%s%s%s\" \"z\" \"z\" \"z\"`,\n",
/// #     "       a: `\"hello\"`,\n",
/// #     "       b: `\"zzz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_eq`](macro@crate::assert_command_stdout_eq)
/// * [`assert_command_stdout_eq_as_result`](macro@crate::assert_command_stdout_eq_as_result)
/// * [`debug_assert_command_stdout_eq`](macro@crate::debug_assert_command_stdout_eq)
///
#[macro_export]
macro_rules! assert_command_stdout_eq {
    ($a_command:expr, $b_command:expr $(,)?) => {{
        match $crate::assert_command_stdout_eq_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_command:expr, $b_command:expr, $($message:tt)+) => {{
        match $crate::assert_command_stdout_eq_as_result!($a_command, $b_command) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command stdout string is equal to another.
///
/// Pseudocode:<br>
/// (command1 ⇒ stdout ⇒ string) = (command2 ⇒ stdout ⇒ string)
///
/// This macro provides the same statements as [`assert_command_stdout_eq`](macro.assert_command_stdout_eq.html),
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
/// * [`assert_command_stdout_eq`](macro@crate::assert_command_stdout_eq)
/// * [`assert_command_stdout_eq`](macro@crate::assert_command_stdout_eq)
/// * [`debug_assert_command_stdout_eq`](macro@crate::debug_assert_command_stdout_eq)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_eq!($($arg)*);
        }
    };
}
