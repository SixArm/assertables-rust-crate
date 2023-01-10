/// Assert a command stdout string is equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
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
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "hello";
/// let x = assert_command_stdout_eq_as_result!(command, s);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// let x = assert_command_stdout_eq_as_result!(command, s);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_eq!(left_command, right_expr)`\n",
///     " left_command label: `command`,\n",
///     " left_command debug: `\"printf\" \"%s\" \"hello\"`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"world\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"world\"`"
/// );
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
macro_rules! assert_command_stdout_eq_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        let a_output = $a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_eq!(left_command, right_expr)`\n",
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
                        "assertion failed: `assert_command_stdout_eq!(left_command, right_expr)`\n",
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
mod test_x_result {

    use std::process::Command;

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "alpha";
        let x = assert_command_stdout_eq_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "bravo";
        let x = assert_command_stdout_eq_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
          "assertion failed: `assert_command_stdout_eq!(left_command, right_expr)`\n",
          " left_command label: `a`,\n",
          " left_command debug: `\"printf\" \"%s\" \"alpha\"`,\n",
          "   right_expr label: `b`,\n",
          "   right_expr debug: `\"bravo\"`,\n",
          "               left: `\"alpha\"`,\n",
          "              right: `\"bravo\"`"
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
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "hello";
/// assert_command_stdout_eq!(command, s);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// assert_command_stdout_eq!(command, s);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_command_stdout_eq!(left_command, right_expr)`\n",
///     " left_command label: `command`,\n",
///     " left_command debug: `\"printf\" \"%s\" \"hello\"`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"world\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"world\"`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// assert_command_stdout_eq!(command, s, "message");
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
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command stdout string is equal to an expression.
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
