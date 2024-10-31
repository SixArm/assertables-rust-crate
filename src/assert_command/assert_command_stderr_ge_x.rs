//! Assert a command stderr string is equal to an expression.
//!
//! Pseudocode:<br>
//! (command ⇒ stderr) = (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut command = Command::new("bin/printf-stderr");
//! command.args(["%s", "alfa"]);
//! let bytes = vec![b'a', b'a'];
//! assert_command_stderr_ge_x!(command, bytes);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_ge_x`](macro@crate::assert_command_stderr_ge_x)
//! * [`assert_command_stderr_ge_x_as_result`](macro@crate::assert_command_stderr_ge_x_as_result)
//! * [`debug_assert_command_stderr_ge_x`](macro@crate::debug_assert_command_stderr_ge_x)

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr) = (expr into string)
///
/// * If true, return Result `Ok(stderr)`.
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
/// * [`assert_command_stderr_ge_x`](macro@crate::assert_command_stderr_ge_x)
/// * [`assert_command_stderr_ge_x_as_result`](macro@crate::assert_command_stderr_ge_x_as_result)
/// * [`debug_assert_command_stderr_ge_x`](macro@crate::debug_assert_command_stderr_ge_x)
///
#[macro_export]
macro_rules! assert_command_stderr_ge_x_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => {{
        match (/*&$command,*/ &$b_expr) {
            b => {
                match $a_command.output() {
                    Ok(a) => {
                        let a = a.stderr;
                        if a.ge(&$b_expr) {
                            Ok(a)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_command_stderr_ge_x!(command, expr)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stderr_ge_x.html\n",
                                        " command label: `{}`,\n",
                                        " command debug: `{:?}`,\n",
                                        "    expr label: `{}`,\n",
                                        "    expr debug: `{:?}`,\n",
                                        " command value: `{:?}`,\n",
                                        "    expr value: `{:?}`"
                                    ),
                                    stringify!($a_command),
                                    $a_command,
                                    stringify!($b_expr),
                                    $b_expr,
                                    a,
                                    b
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_command_stderr_ge_x!(command, expr)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stderr_ge_x.html\n",
                                    "  command label: `{}`,\n",
                                    "  command debug: `{:?}`,\n",
                                    "     expr label: `{}`,\n",
                                    "     expr debug: `{:?}`,\n",
                                    "  output is err: `{:?}`"
                                ),
                                stringify!($a_command),
                                $a_command,
                                stringify!($b_expr),
                                b,
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
mod tests {

    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'a'];
        let result = assert_command_stderr_ge_x_as_result!(a, b);
        assert_eq!(result.unwrap(), vec![b'a', b'l', b'f', b'a']);
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'l', b'f', b'a'];
        let result = assert_command_stderr_ge_x_as_result!(a, b);
        assert_eq!(result.unwrap(), vec![b'a', b'l', b'f', b'a']);
    }

    #[test]
    fn lt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let b = vec![b'z', b'z'];
        let result = assert_command_stderr_ge_x_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_ge_x!(command, expr)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stderr_ge_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[122, 122]`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr value: `[122, 122]`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr) = (expr into string)
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
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "alfa"]);
/// let bytes = vec![b'a', b'a'];
/// assert_command_stderr_ge_x!(command, bytes);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "alfa"]);
/// let bytes = vec![b'z', b'z'];
/// assert_command_stderr_ge_x!(command, bytes);
/// # });
/// // assertion failed: `assert_command_stderr_ge_x!(command, expr)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stderr_ge_x.html
/// //  command label: `command`,
/// //  command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,
/// //     expr label: `bytes`,
/// //     expr debug: `[122, 122]`,
/// //  command value: `[97, 108, 102, 97]`,
/// //     expr value: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stderr_ge_x!(command, expr)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stderr_ge_x.html\n",
/// #     " command label: `command`,\n",
/// #     " command debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
/// #     "    expr label: `bytes`,\n",
/// #     "    expr debug: `[122, 122]`,\n",
/// #     " command value: `[97, 108, 102, 97]`,\n",
/// #     "    expr value: `[122, 122]`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_ge_x`](macro@crate::assert_command_stderr_ge_x)
/// * [`assert_command_stderr_ge_x_as_result`](macro@crate::assert_command_stderr_ge_x_as_result)
/// * [`debug_assert_command_stderr_ge_x`](macro@crate::debug_assert_command_stderr_ge_x)
///
#[macro_export]
macro_rules! assert_command_stderr_ge_x {
    ($a_command:expr, $b_expr:expr $(,)?) => {{
        match $crate::assert_command_stderr_ge_x_as_result!($a_command, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_command:expr, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_command_stderr_ge_x_as_result!($a_command, $b_expr) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr) = (expr into string)
///
/// This macro provides the same statements as [`assert_command_stderr_ge_x`](macro.assert_command_stderr_ge_x.html),
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
/// * [`assert_command_stderr_ge_x`](macro@crate::assert_command_stderr_ge_x)
/// * [`assert_command_stderr_ge_x`](macro@crate::assert_command_stderr_ge_x)
/// * [`debug_assert_command_stderr_ge_x`](macro@crate::debug_assert_command_stderr_ge_x)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_ge_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_ge_x!($($arg)*);
        }
    };
}
