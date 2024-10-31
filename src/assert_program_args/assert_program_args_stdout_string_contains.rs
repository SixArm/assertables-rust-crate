//! Assert a command (built with program and args) stdout string contains a given containee.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stdout ⇒ string) contains (expr into string)
//!
//! This uses [`::std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
//!
//! * The containee can be a &str, char, a slice of chars, or a function or
//! closure that determines if a character contains.
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let program = "bin/printf-stdout";
//! let args = ["%s", "alfa"];
//! let containee = "lf";
//! assert_program_args_stdout_string_contains!(&program, &args, &containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_string_contains`](macro@crate::assert_program_args_stdout_string_contains)
//! * [`assert_program_args_stdout_string_contains_as_result`](macro@crate::assert_program_args_stdout_string_contains_as_result)
//! * [`debug_assert_program_args_stdout_string_contains`](macro@crate::debug_assert_program_args_stdout_string_contains)

/// Assert a command (built with program and args) stdout string contains a given containee.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) contains (expr into string)
///
/// * If true, return Result `Ok(program1 + args1 ⇒ command ⇒ stdout ⇒ string)`.
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
/// * [`assert_program_args_stdout_string_contains`](macro@crate::assert_program_args_stdout_string_contains)
/// * [`assert_program_args_stdout_string_contains_as_result`](macro@crate::assert_program_args_stdout_string_contains_as_result)
/// * [`debug_assert_program_args_stdout_string_contains`](macro@crate::debug_assert_program_args_stdout_string_contains)
///
#[macro_export]
macro_rules! assert_program_args_stdout_string_contains_as_result {
    ($a_program:expr, $a_args:expr, $containee:expr $(,)?) => {{
        match ($a_program, $a_args, &$containee) {
            (a_program, a_args, containee) => {
                match assert_program_args_impl_prep!(a_program, a_args) {
                    Ok(a_output) => {
                        let a_string = String::from_utf8(a_output.stdout).unwrap();
                        if a_string.contains($containee) {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stdout_string_contains!(a_program, a_args, containee)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_string_contains.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        " containee label: `{}`,\n",
                                        " containee debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($containee),
                                    containee,
                                    a_string,
                                    $containee
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stdout_string_contains!(a_program, a_args, containee)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_string_contains.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    " containee label: `{}`,\n",
                                    " containee debug: `{:?}`,\n",
                                    "             err: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($containee),
                                containee,
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

    #[test]
    fn test_assert_command_stdout_contains_x_success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = "lf";
        let result = assert_program_args_stdout_string_contains_as_result!(&a_program, &a_args, b);
        assert_eq!(result.unwrap(), "alfa");
    }

    #[test]
    fn test_assert_command_stdout_contains_x_failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = "zz";
        let result = assert_program_args_stdout_string_contains_as_result!(&a_program, &a_args, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_string_contains!(a_program, a_args, containee)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_string_contains.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            "               a: `\"alfa\"`,\n",
            "               b: `\"zz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string contains a given containee.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) contains (expr into string)
///
/// * If true, return (program1 + args1 ⇒ command ⇒ stdout ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`::std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
///
/// # fn main() {
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let containee = "lf";
/// assert_program_args_stdout_string_contains!(&program, &args, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let containee = "zz";
/// assert_program_args_stdout_string_contains!(&program, &args, &containee);
/// # });
/// // assertion failed: `assert_program_args_stdout_string_contains!(a_program, a_args, containee)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_string_contains.html
/// //  a_program label: `&program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `&args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zz\"`,
/// //                a: `\"alfa\"`,
/// //                b: `\"zz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stdout_string_contains!(a_program, a_args, containee)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_string_contains.html\n",
/// #     " a_program label: `&program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `&args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zz\"`,\n",
/// #     "               a: `\"alfa\"`,\n",
/// #     "               b: `\"zz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_string_contains`](macro@crate::assert_program_args_stdout_string_contains)
/// * [`assert_program_args_stdout_string_contains_as_result`](macro@crate::assert_program_args_stdout_string_contains_as_result)
/// * [`debug_assert_program_args_stdout_string_contains`](macro@crate::debug_assert_program_args_stdout_string_contains)
///
#[macro_export]
macro_rules! assert_program_args_stdout_string_contains {
    ($a_program:expr, $a_args:expr, $containee:expr $(,)?) => {{
        match $crate::assert_program_args_stdout_string_contains_as_result!($a_program, $a_args, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $containee:expr, $($message:tt)+) => {{
        match $crate::assert_program_args_stdout_string_contains_as_result!($a_program, $a_args, $containee) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stdout string contains a given containee.
///
/// This macro provides the same statements as [`assert_program_args_stdout_string_contains`](macro.assert_program_args_stdout_string_contains.html),
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
/// * [`assert_program_args_stdout_string_contains`](macro@crate::assert_program_args_stdout_string_contains)
/// * [`assert_program_args_stdout_string_contains`](macro@crate::assert_program_args_stdout_string_contains)
/// * [`debug_assert_program_args_stdout_string_contains`](macro@crate::debug_assert_program_args_stdout_string_contains)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_string_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_string_contains!($($arg)*);
        }
    };
}
