/// Assert a command stdout string is equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_eq_expr`](macro.assert_command_stdout_eq_expr.html)
/// * [`assert_command_stdout_eq_expr_as_result`](macro.assert_command_stdout_eq_expr_as_result.html)
/// * [`debug_assert_command_stdout_eq_expr`](macro.debug_assert_command_stdout_eq_expr.html)
///
#[macro_export]
macro_rules! assert_command_stdout_eq_expr_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_eq_expr!(left_command, right_expr)`\n",
                    " left_command label: `{}`,\n",
                    " left_command debug: `{:?}`,\n",
                    "   right_expr label: `{}`,\n",
                    "   right_expr debug: `{:?}`,\n",
                    "        left output: `{:?}`"
                ),
                stringify!($a_command), $a_command,
                stringify!($b_expr), $b_expr,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
            if a_string == $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_command_stdout_eq_expr!(left_command, right_expr)`\n",
                        " left_command label: `{}`,\n",
                        " left_command debug: `{:?}`,\n",
                        "   right_expr label: `{}`,\n",
                        "   right_expr debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($a_command), $a_command,
                    stringify!($b_expr), $b_expr,
                    a_string,
                    $b_expr
                ))
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_eq_expr_as_result_x_success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = String::from("hello");
        let x = assert_command_stdout_eq_expr_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_eq_expr_as_result_x_failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "hello"]);
        let b = String::from("zzz");
        let x = assert_command_stdout_eq_expr_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stdout_eq_expr!(left_command, right_expr)`\n",
            " left_command label: `a`,\n",
            " left_command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
            "   right_expr label: `b`,\n",
            "   right_expr debug: `\"zzz\"`,\n",
            "               left: `\"hello\"`,\n",
            "              right: `\"zzz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stdout string is equal to an expression.
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
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let s = String::from("hello");
/// assert_command_stdout_eq_expr!(command, s);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let s = String::from("zzz");
/// assert_command_stdout_eq_expr!(command, s);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_eq_expr!(left_command, right_expr)`\n",
///     " left_command label: `command`,\n",
///     " left_command debug: `\"bin/printf-stdout\" \"%s\" \"hello\"`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"zzz\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"zzz\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// assert_command_stdout_eq_expr!(command, s, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_eq_expr`](macro.assert_command_stdout_eq_expr.html)
/// * [`assert_command_stdout_eq_expr_as_result`](macro.assert_command_stdout_eq_expr_as_result.html)
/// * [`debug_assert_command_stdout_eq_expr`](macro.debug_assert_command_stdout_eq_expr.html)
///
#[macro_export]
macro_rules! assert_command_stdout_eq_expr {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stdout_eq_expr_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_command_stdout_eq_expr_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command stdout string is equal to an expression.
///
/// This macro provides the same statements as [`assert_command_stdout_eq_expr`],
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
/// # Module macros
///
/// * [`assert_command_stdout_eq_expr`](macro.assert_command_stdout_eq_expr.html)
/// * [`assert_command_stdout_eq_expr`](macro.assert_command_stdout_eq_expr.html)
/// * [`debug_assert_command_stdout_eq_expr`](macro.debug_assert_command_stdout_eq_expr.html)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_eq_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stdout_eq_expr!($($arg)*);
        }
    };
}
