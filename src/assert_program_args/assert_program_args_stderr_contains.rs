//! Assert a command (built with program and args) stderr string contains a given containee.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stderr ⇒ string) contains (expr into string)
//!
//! This uses [`std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
//!
//! * The containee can be a &str, char, a slice of chars, or a function or
//! closure that determines if a character contains.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let program = "bin/printf-stderr";
//! let args = ["%s", "hello"];
//! let containee = "ell";
//! assert_program_args_stderr_contains!(&program, &args, &containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stderr_contains`](macro@crate::assert_program_args_stderr_contains)
//! * [`assert_program_args_stderr_contains_as_result`](macro@crate::assert_program_args_stderr_contains_as_result)
//! * [`debug_assert_program_args_stderr_contains`](macro@crate::debug_assert_program_args_stderr_contains)

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) contains (expr into string)
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
/// * [`assert_program_args_stderr_contains`](macro@crate::assert_program_args_stderr_contains)
/// * [`assert_program_args_stderr_contains_as_result`](macro@crate::assert_program_args_stderr_contains_as_result)
/// * [`debug_assert_program_args_stderr_contains`](macro@crate::debug_assert_program_args_stderr_contains)
///
#[macro_export]
macro_rules! assert_program_args_stderr_contains_as_result {
    ($a_program:expr, $a_args:expr, $containee:expr $(,)?) => {{
        match ($a_program, $a_args, $containee) {
            (a_program, a_args, containee) => {
                let a_output = assert_program_args_impl_prep!(a_program, a_args);
                if a_output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_program_args_stderr_contains!(a_program, a_args, containee)`\n",
                            "https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stderr_contains.html\n",
                            " a_program label: `{}`,\n",
                            " a_program debug: `{:?}`,\n",
                            "    a_args label: `{}`,\n",
                            "    a_args debug: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`,\n",
                            "        a output: `{:?}`"
                        ),
                        stringify!($a_program),
                        a_program,
                        stringify!($a_args),
                        a_args,
                        stringify!($containee),
                        containee,
                        a_output
                    ))
                } else {
                    let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
                    if a_string.contains($containee) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_program_args_stderr_contains!(a_program, a_args, containee)`\n",
                                "https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stderr_contains.html\n",
                                " a_program label: `{}`,\n",
                                " a_program debug: `{:?}`,\n",
                                "    a_args label: `{}`,\n",
                                "    a_args debug: `{:?}`,\n",
                                " containee label: `{}`,\n",
                                " containee debug: `{:?}`,\n",
                                "               a: `{:?}`,\n",
                                "       containee: `{:?}`"
                            ),
                            stringify!($a_program),
                            a_program,
                            stringify!($a_args),
                            a_args,
                            stringify!($containee),
                            containee,
                            a_string,
                            $containee
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_command_stderr_contains_x_success() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b = "ell";
        let result = assert_program_args_stderr_contains_as_result!(&a_program, &a_args, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_contains_x_failure() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b = "zzz";
        let result = assert_program_args_stderr_contains_as_result!(&a_program, &a_args, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_contains!(a_program, a_args, containee)`\n",
            "https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stderr_contains.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"hello\"]`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zzz\"`,\n",
            "               a: `\"hello\"`,\n",
            "       containee: `\"zzz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) contains (expr into string)
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
///
/// # fn main() {
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let containee = "ell";
/// assert_program_args_stderr_contains!(&program, &args, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let containee = "zzz";
/// assert_program_args_stderr_contains!(&program, &args, &containee);
/// # });
/// // assertion failed: `assert_program_args_stderr_contains!(a_program, a_args, containee)`
/// // https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stderr_contains.html
/// //  a_program label: `&program`,
/// //  a_program debug: `\"bin/printf-stderr\"`,
/// //     a_args label: `&args`,
/// //     a_args debug: `[\"%s\", \"hello\"]`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zzz\"`,
/// //                a: `\"hello\"`,
/// //        containee: `\"zzz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stderr_contains!(a_program, a_args, containee)`\n",
/// #     "https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stderr_contains.html\n",
/// #     " a_program label: `&program`,\n",
/// #     " a_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    a_args label: `&args`,\n",
/// #     "    a_args debug: `[\"%s\", \"hello\"]`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zzz\"`,\n",
/// #     "               a: `\"hello\"`,\n",
/// #     "       containee: `\"zzz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stderr_contains`](macro@crate::assert_program_args_stderr_contains)
/// * [`assert_program_args_stderr_contains_as_result`](macro@crate::assert_program_args_stderr_contains_as_result)
/// * [`debug_assert_program_args_stderr_contains`](macro@crate::debug_assert_program_args_stderr_contains)
///
#[macro_export]
macro_rules! assert_program_args_stderr_contains {
    ($a_program:expr, $a_args:expr, $containee:expr $(,)?) => {{
        match assert_program_args_stderr_contains_as_result!($a_program, $a_args, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $containee:expr, $($message:tt)+) => {{
        match assert_program_args_stderr_contains_as_result!($a_program, $a_args, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stderr ⇒ string) contains (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stderr_contains`](macro.assert_program_args_stderr_contains.html),
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
/// * [`assert_program_args_stderr_contains`](macro@crate::assert_program_args_stderr_contains)
/// * [`assert_program_args_stderr_contains`](macro@crate::assert_program_args_stderr_contains)
/// * [`debug_assert_program_args_stderr_contains`](macro@crate::debug_assert_program_args_stderr_contains)
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_contains!($($arg)*);
        }
    };
}
