//! Assert a command (built with program and args) stderr string is a match to a regex.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stderr ⇒ string) is match (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! # fn main() {
//! let program = "bin/printf-stderr";
//! let args = ["%s", "alfa"];
//! let matcher = Regex::new(r"lf").unwrap();
//! assert_program_args_stderr_string_is_match!(&program, &args, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stderr_string_is_match`](macro@crate::assert_program_args_stderr_string_is_match)
//! * [`assert_program_args_stderr_string_is_match_as_result`](macro@crate::assert_program_args_stderr_string_is_match_as_result)
//! * [`debug_assert_program_args_stderr_string_is_match`](macro@crate::debug_assert_program_args_stderr_string_is_match)

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) is match (expr into string)
///
/// * If true, return Result `Ok(program1 + args1 ⇒ command ⇒ stderr ⇒ string)`.
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
/// * [`assert_program_args_stderr_string_is_match`](macro@crate::assert_program_args_stderr_string_is_match)
/// * [`assert_program_args_stderr_string_is_match_as_result`](macro@crate::assert_program_args_stderr_string_is_match_as_result)
/// * [`debug_assert_program_args_stderr_string_is_match`](macro@crate::debug_assert_program_args_stderr_string_is_match)
///
#[macro_export]
macro_rules! assert_program_args_stderr_string_is_match_as_result {
    ($a_program:expr, $a_args:expr, $matcher:expr $(,)?) => {{
        match ($a_program, $a_args, &$matcher) {
            (a_program, a_args, matcher) => {
                match assert_program_args_impl_prep!(a_program, a_args) {
                    Ok(a_output) => {
                        let a_string = String::from_utf8(a_output.stderr).unwrap();
                        if $matcher.is_match(&a_string) {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stderr_string_is_match!(a_program, b_matcher)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stderr_string_is_match.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        " b_matcher label: `{}`,\n",
                                        " b_matcher debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($matcher),
                                    matcher,
                                    a_string,
                                    $matcher
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stderr_string_is_match!(a_program, b_matcher)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stderr_string_is_match.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    " b_matcher label: `{}`,\n",
                                    " b_matcher debug: `{:?}`,\n",
                                    "         a output: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
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
mod tests {

    use regex::Regex;

    #[test]
    fn test_assert_program_args_stderr_string_is_match_as_result_x_success() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"lf").unwrap();
        let result = assert_program_args_stderr_string_is_match_as_result!(&a_program, &a_args, b);
        assert_eq!(result.unwrap(), "alfa");
    }

    #[test]
    fn test_assert_program_args_stderr_string_is_match_as_result_x_failure() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"zz").unwrap();
        let result = assert_program_args_stderr_string_is_match_as_result!(&a_program, &a_args, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_string_is_match!(a_program, b_matcher)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stderr_string_is_match.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_matcher label: `b`,\n",
            " b_matcher debug: `Regex(\"zz\")`,\n",
            "               a: `\"alfa\"`,\n",
            "               b: `Regex(\"zz\")`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) is match (expr into string)
///
/// * If true, return (program1 + args1 ⇒ command ⇒ stderr ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use regex::Regex;
///
/// # fn main() {
/// let program = "bin/printf-stderr";
/// let args = ["%s", "alfa"];
/// let matcher = Regex::new(r"lf").unwrap();
/// assert_program_args_stderr_string_is_match!(&program, &args, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let program = "bin/printf-stderr";
/// let args = ["%s", "alfa"];
/// let matcher = Regex::new(r"zz").unwrap();
/// assert_program_args_stderr_string_is_match!(&program, &args, &matcher);
/// # });
/// // assertion failed: `assert_program_args_stderr_string_is_match!(a_program, b_matcher)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stderr_string_is_match.html
/// //  a_program label: `&program`,
/// //  a_program debug: `\"bin/printf-stderr\"`,
/// //     a_args label: `&args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //  b_matcher label: `&matcher`,
/// //  b_matcher debug: `Regex(\"zz\")`,
/// //                a: `\"alfa\"`,
/// //                b: `Regex(\"zz\")`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stderr_string_is_match!(a_program, b_matcher)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stderr_string_is_match.html\n",
/// #     " a_program label: `&program`,\n",
/// #     " a_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    a_args label: `&args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     " b_matcher label: `&matcher`,\n",
/// #     " b_matcher debug: `Regex(\"zz\")`,\n",
/// #     "               a: `\"alfa\"`,\n",
/// #     "               b: `Regex(\"zz\")`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stderr_string_is_match`](macro@crate::assert_program_args_stderr_string_is_match)
/// * [`assert_program_args_stderr_string_is_match_as_result`](macro@crate::assert_program_args_stderr_string_is_match_as_result)
/// * [`debug_assert_program_args_stderr_string_is_match`](macro@crate::debug_assert_program_args_stderr_string_is_match)
///
#[macro_export]
macro_rules! assert_program_args_stderr_string_is_match {
    ($a_program:expr, $a_args:expr, $matcher:expr $(,)?) => {{
        match $crate::assert_program_args_stderr_string_is_match_as_result!($a_program, $a_args, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $matcher:expr, $($message:tt)+) => {{
        match $crate::assert_program_args_stderr_string_is_match_as_result!($a_program, $a_args, $matcher) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stderr string is a match to a regex.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) is match (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stderr_string_is_match`](macro.assert_program_args_stderr_string_is_match.html),
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
/// * [`assert_program_args_stderr_string_is_match`](macro@crate::assert_program_args_stderr_string_is_match)
/// * [`assert_program_args_stderr_string_is_match`](macro@crate::assert_program_args_stderr_string_is_match)
/// * [`debug_assert_program_args_stderr_string_is_match`](macro@crate::debug_assert_program_args_stderr_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_string_is_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_string_is_match!($($arg)*);
        }
    };
}
