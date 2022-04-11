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
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "hello";
/// let x = assert_command_stdout_eq_as_result!(command, s);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// let x = assert_command_stdout_eq_as_result!(command, s);
/// //-> Err(…)
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
    fn test_assert_command_stdout_eq_as_result_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let b = "alpha";
        let x = assert_command_stdout_eq_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stdout_eq_as_result_x_arity_2_failure() {
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
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "hello";
/// assert_command_stdout_eq!(command, s);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut command = Command::new("printf");
/// command.args(["%s", "hello"]);
/// let s = "world";
/// assert_command_stdout_eq!(command, s);
/// //-> panic!
/// });
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
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_command_stdout_eq {
    ($a_command:expr, $b_expr:expr $(,)?) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_command:expr, $b_expr:expr, $($arg:tt)+) => ({
        match assert_command_stdout_eq_as_result!($a_command, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}
