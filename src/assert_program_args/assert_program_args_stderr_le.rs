//! Assert a command (built with program and args) stderr string is less than or equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a_program = "bin/printf-stderr";
//! let a_args = ["%s", "hello"];
//! let b_program = "bin/printf-stderr";
//! let b_args = ["%s", "hullo"];
//! assert_program_args_stderr_le!(&a_program, &a_args, &b_program, &b_args);
//! # }
//! ```
//!
//! /// # Module macros
//!
//! * [`assert_program_args_stderr_le`](macro@crate::assert_program_args_stderr_le)
//! * [`assert_program_args_stderr_le_as_result`](macro@crate::assert_program_args_stderr_le_as_result)
//! * [`debug_assert_program_args_stderr_le`](macro@crate::debug_assert_program_args_stderr_le)

/// Assert a command (built with program and args) stderr string is less than or equal to another.
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
/// * [`assert_program_args_stderr_le`](macro@crate::assert_program_args_stderr_le)
/// * [`assert_program_args_stderr_le_as_result`](macro@crate::assert_program_args_stderr_le_as_result)
/// * [`debug_assert_program_args_stderr_le`](macro@crate::debug_assert_program_args_stderr_le)
///
#[macro_export]
macro_rules! assert_program_args_stderr_le_as_result {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => ({
        match ($a_program, $a_args, $b_program, $b_args) {
            (a_program, a_args, b_program, b_args) => {
                let a_output = assert_program_args_impl_prep!(a_program, a_args);
                let b_output = assert_program_args_impl_prep!(b_program, b_args);
                if a_output.is_err() || b_output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stderr_le.html\n",
                            " a_program label: `{}`,\n",
                            " a_program debug: `{:?}`,\n",
                            "    a_args label: `{}`,\n",
                            "    a_args debug: `{:?}`,\n",
                            " b_program label: `{}`,\n",
                            " b_program debug: `{:?}`,\n",
                            "    b_args label: `{}`,\n",
                            "    b_args debug: `{:?}`,\n",
                            "        a output: `{:?}`,\n",
                            "        b output: `{:?}`"
                        ),
                        stringify!($a_program),
                        a_program,
                        stringify!($a_args),
                        a_args,
                        stringify!($b_program),
                        b_program,
                        stringify!($b_args),
                        b_args,
                        a_output,
                        b_output
                    ))
                } else {
                    let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
                    let b_string = String::from_utf8(b_output.unwrap().stderr).unwrap();
                    if a_string <= b_string {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`\n",
                                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stderr_le.html\n",
                                " a_program label: `{}`,\n",
                                " a_program debug: `{:?}`,\n",
                                "    a_args label: `{}`,\n",
                                "    a_args debug: `{:?}`,\n",
                                " b_program label: `{}`,\n",
                                " b_program debug: `{:?}`,\n",
                                "    b_args label: `{}`,\n",
                                "    b_args debug: `{:?}`,\n",
                                "               a: `{:?}`,\n",
                                "               b: `{:?}`"
                            ),
                            stringify!($a_program),
                            a_program,
                            stringify!($a_args),
                            a_args,
                            stringify!($b_program),
                            b_program,
                            stringify!($b_args),
                            b_args,
                            a_string,
                            b_string
                        ))
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_program_args_stderr_le_as_result_x_success() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "hullo"];
        let result = assert_program_args_stderr_le_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stderr_le_as_result_x_failure() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "hallo"];
        let result = assert_program_args_stderr_le_as_result!(&a_program, &a_args, &b_program, &b_args);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stderr_le.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"hello\"]`,\n",
            " b_program label: `&b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `&b_args`,\n",
            "    b_args debug: `[\"%s\", \"hallo\"]`,\n",
            "               a: `\"hello\"`,\n",
            "               b: `\"hallo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stderr string is less than or equal to another.
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
///
/// # fn main() {
/// let a_program = "bin/printf-stderr";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stderr";
/// let b_args = ["%s", "hullo"];
/// assert_program_args_stderr_le!(&a_program, &a_args, &b_program, &b_args);
///
/// # let result = panic::catch_unwind(|| {
/// let a_program = "bin/printf-stderr";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stderr";
/// let b_args = ["%s", "hallo"];
/// assert_program_args_stderr_le!(&a_program, &a_args, &b_program, &b_args);
/// # });
/// // assertion failed: `assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stderr_le.html
/// //  a_program label: `&a_program`,
/// //  a_program debug: `\"bin/printf-stderr\"`,
/// //     a_args label: `&a_args`,
/// //     a_args debug: `[\"%s\", \"hello\"]`,
/// //  b_program label: `&b_program`,
/// //  b_program debug: `\"bin/printf-stderr\"`,
/// //     b_args label: `&b_args`,
/// //     b_args debug: `[\"%s\", \"hallo\"]`,
/// //                a: `\"hello\"`,
/// //                b: `\"hallo\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stderr_le.html\n",
/// #     " a_program label: `&a_program`,\n",
/// #     " a_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    a_args label: `&a_args`,\n",
/// #     "    a_args debug: `[\"%s\", \"hello\"]`,\n",
/// #     " b_program label: `&b_program`,\n",
/// #     " b_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    b_args label: `&b_args`,\n",
/// #     "    b_args debug: `[\"%s\", \"hallo\"]`,\n",
/// #     "               a: `\"hello\"`,\n",
/// #     "               b: `\"hallo\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// /// # Module macros
///
/// * [`assert_program_args_stderr_le`](macro@crate::assert_program_args_stderr_le)
/// * [`assert_program_args_stderr_le_as_result`](macro@crate::assert_program_args_stderr_le_as_result)
/// * [`debug_assert_program_args_stderr_le`](macro@crate::debug_assert_program_args_stderr_le)
///
#[macro_export]
macro_rules! assert_program_args_stderr_le {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => ({
        match assert_program_args_stderr_le_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_program:expr, $($message:tt)+) => ({
        match assert_program_args_stderr_le_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command (built with program and args) stderr string is less than or equal to another.
///
/// This macro provides the same statements as [`assert_program_args_stderr_le`](macro.assert_program_args_stderr_le.html),
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
/// * [`assert_program_args_stderr_le`](macro@crate::assert_program_args_stderr_le)
/// * [`assert_program_args_stderr_le`](macro@crate::assert_program_args_stderr_le)
/// * [`debug_assert_program_args_stderr_le`](macro@crate::debug_assert_program_args_stderr_le)
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_le {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_le!($($arg)*);
        }
    };
}
