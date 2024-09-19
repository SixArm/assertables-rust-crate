//! Assert a command stderr string is equal to an expression.
//!
//! Pseudocode:<br>
//! (command ⇒ stderr ⇒ string) = (expr into string)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut command = Command::new("bin/printf-stderr");
//! command.args(["%s", "hello"]);
//! let s = String::from("hello");
//! assert_command_stderr_eq_expr!(command, s);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_eq_expr`](macro@crate::assert_command_stderr_eq_expr)
//! * [`assert_command_stderr_eq_expr_as_result`](macro@crate::assert_command_stderr_eq_expr_as_result)
//! * [`debug_assert_command_stderr_eq_expr`](macro@crate::debug_assert_command_stderr_eq_expr)

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) = (expr into string)
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
/// * [`assert_command_stderr_eq_expr`](macro@crate::assert_command_stderr_eq_expr)
/// * [`assert_command_stderr_eq_expr_as_result`](macro@crate::assert_command_stderr_eq_expr_as_result)
/// * [`debug_assert_command_stderr_eq_expr`](macro@crate::debug_assert_command_stderr_eq_expr)
///
#[macro_export]
macro_rules! assert_command_stderr_eq_expr_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match (/*&$command,*/ &$b_expr) {
            b_expr => {
                let a_output = $a_command.output();
                if a_output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_command_stderr_eq_expr!(command, expr)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_command_stderr_eq_expr.html\n",
                            "  command label: `{}`,\n",
                            "  command debug: `{:?}`,\n",
                            "     expr label: `{}`,\n",
                            "     expr debug: `{:?}`,\n",
                            " command output: `{:?}`"
                        ),
                        stringify!($a_command),
                        $a_command,
                        stringify!($b_expr),
                        b_expr,
                        a_output
                    ))
                } else {
                    let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
                    if a_string == String::from(b_expr) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_command_stderr_eq_expr!(command, expr)`\n",
                                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_command_stderr_eq_expr.html\n",
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
                            b_expr,
                            a_string,
                            b_expr
                        ))
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stderr_eq_expr_as_result_x_success() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "hello"]);
        let b = String::from("hello");
        let result = assert_command_stderr_eq_expr_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_eq_expr_as_result_x_failure() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "hello"]);
        let b = String::from("zzz");
        let result = assert_command_stderr_eq_expr_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_eq_expr!(command, expr)`\n",
            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_command_stderr_eq_expr.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `\"zzz\"`,\n",
            " command value: `\"hello\"`,\n",
            "    expr value: `\"zzz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) = (expr into string)
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
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "hello"]);
/// let s = String::from("hello");
/// assert_command_stderr_eq_expr!(command, s);
///
/// # let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "hello"]);
/// let s = String::from("zzz");
/// assert_command_stderr_eq_expr!(command, s);
/// # });
/// // assertion failed: `assert_command_stderr_eq_expr!(command, expr)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_command_stderr_eq_expr.html
/// //  command label: `command`,
/// //  command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,
/// //     expr label: `s`,
/// //     expr debug: `\"zzz\"`,
/// //  command value: `\"hello\"`,
/// //     expr value: `\"zzz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stderr_eq_expr!(command, expr)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_command_stderr_eq_expr.html\n",
/// #     " command label: `command`,\n",
/// #     " command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,\n",
/// #     "    expr label: `s`,\n",
/// #     "    expr debug: `\"zzz\"`,\n",
/// #     " command value: `\"hello\"`,\n",
/// #     "    expr value: `\"zzz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_eq_expr`](macro@crate::assert_command_stderr_eq_expr)
/// * [`assert_command_stderr_eq_expr_as_result`](macro@crate::assert_command_stderr_eq_expr_as_result)
/// * [`debug_assert_command_stderr_eq_expr`](macro@crate::debug_assert_command_stderr_eq_expr)
///
#[macro_export]
macro_rules! assert_command_stderr_eq_expr {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stderr_eq_expr_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_command_stderr_eq_expr_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command stderr string is equal to an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) = (expr into string)
/// 
/// This macro provides the same statements as [`assert_command_stderr_eq_expr`](macro.assert_command_stderr_eq_expr.html),
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
/// * [`assert_command_stderr_eq_expr`](macro@crate::assert_command_stderr_eq_expr)
/// * [`assert_command_stderr_eq_expr`](macro@crate::assert_command_stderr_eq_expr)
/// * [`debug_assert_command_stderr_eq_expr`](macro@crate::debug_assert_command_stderr_eq_expr)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_eq_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_eq_expr!($($arg)*);
        }
    };
}
