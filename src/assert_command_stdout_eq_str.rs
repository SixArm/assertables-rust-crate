/// Assert a command stdout string is equal to a target string.
///
/// * When true, return `()`.
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
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// assert_command_stdout_eq_str!(a, "hello");
/// //-> ()
///
/// # let result = panic::catch_unwind(|| {
/// let mut a = Command::new("printf");
/// a.args(["%s", "hello"]);
/// assert_command_stdout_eq_str!(a, "world");
/// //-> panic!("…")
/// // assertion failed: `assert_command_stdout_eq_str!(command, expect)`
/// //  command program: `\"printf\"`,
/// //  stdout: `\"hello\"`,
/// //  expect: `\"world\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = "assertion failed: `assert_command_stdout_eq_str!(command, expect)`\n command program: `\"printf\"`,\n stdout: `\"hello\"`,\n expect: `\"world\"`";
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assert_command_stdout_eq_str {
    ($command:expr, $expect:expr $(,)?) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual == $expect {
                    ()
                } else {
                    panic!("assertion failed: `assert_command_stdout_eq_str!(command, expect)`\n command program: `{:?}`,\n stdout: `{:?}`,\n expect: `{:?}`", $command.get_program(), actual, $expect)
                }
            }
            Err(err) => {
                panic!("assertion failed: `assert_command_stdout_eq_str!(command, expect)`\n command program: `{:?}`,\n expect: `{:?}`,\n err: {:?}", $command.get_program(), $expect, err)
            }
        }
    });
    ($command:expr, $expect:expr, $($arg:tt)+) => ({
        match $command.output() {
            Ok(output) => {
                let actual = String::from_utf8(output.stdout).unwrap();
                if actual == $expect {
                    ()
                } else {
                    panic!("{:?}", $($arg)+)
                }
            }
            Err(_) => {
                panic!("{:?}", $($arg)+)
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn assert_command_stdout_eq_str_x_arity_2_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "alpha";
        let x = assert_command_stdout_eq_str!(a, expect);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_command_stdout_eq_str!(command, expect)`\n command program: `\"printf\"`,\n stdout: `\"alpha\"`,\n expect: `\"bravo\"`")]
    fn assert_command_stdout_eq_str_x_arity_2_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "bravo";
        let _x = assert_command_stdout_eq_str!(a, expect);
    }

    #[test]
    fn assert_command_stdout_eq_str_x_arity_3_success() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "alpha";
        let x = assert_command_stdout_eq_str!(a, expect, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn assert_command_stdout_eq_str_x_arity_3_failure() {
        let mut a = Command::new("printf");
        a.args(["%s", "alpha"]);
        let expect = "bravo";
        let _x = assert_command_stdout_eq_str!(a, expect, "message");
    }

}
