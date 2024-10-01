//! Assert a command (built with program and args) stdout string is greater than another.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (program2 + args2 ⇒ command ⇒ stdout ⇒ string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! # fn main() {
//! let a_program = "bin/printf-stdout";
//! let a_args = ["%s", "hello"];
//! let b_program = "bin/printf-stdout";
//! let b_args = ["%s%s%s%s%s", "h", "a", "l", "l", "o"];
//! assert_program_args_stdout_gt!(&a_program, &a_args, &b_program, &b_args);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_gt`](macro@crate::assert_program_args_stdout_gt)
//! * [`assert_program_args_stdout_gt_as_result`](macro@crate::assert_program_args_stdout_gt_as_result)
//! * [`debug_assert_program_args_stdout_gt`](macro@crate::debug_assert_program_args_stdout_gt)

/// Assert a command (built with program and args) stdout string is greater than to another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (program2 + args2 ⇒ command ⇒ stdout ⇒ string)
///
/// * If true, return `()`.
///
/// * If true, return Result `Err` with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_program_args_stdout_gt`](macro@crate::assert_program_args_stdout_gt)
/// * [`assert_program_args_stdout_gt_as_result`](macro@crate::assert_program_args_stdout_gt_as_result)
/// * [`debug_assert_program_args_stdout_gt`](macro@crate::debug_assert_program_args_stdout_gt)
///
#[macro_export]
macro_rules! assert_program_args_stdout_gt_as_result {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {{
        match ($a_program, $a_args, $b_program, $b_args) {
            (a_program, a_args, b_program, b_args) => {
                let a_output = assert_program_args_impl_prep!(a_program, a_args);
                let b_output = assert_program_args_impl_prep!(b_program, b_args);
                if a_output.is_err() || b_output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`\n",
                            "https://docs.rs/assertables/8.11.0/assertables/macro.assert_program_args_stdout_gt.html\n",
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
                    let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
                    let b_string = String::from_utf8(b_output.unwrap().stdout).unwrap();
                    if a_string > b_string {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`\n",
                                "https://docs.rs/assertables/8.11.0/assertables/macro.assert_program_args_stdout_gt.html\n",
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
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_program_args_stdout_gt_as_result_x_success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s%s%s%s", "h", "a", "l", "l", "o"];
        let result = assert_program_args_stdout_gt_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stdout_gt_as_result_x_failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s%s%s%s", "h", "u", "l", "l", "o"];
        let result = assert_program_args_stdout_gt_as_result!(&a_program, &a_args, &b_program, &b_args);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/8.11.0/assertables/macro.assert_program_args_stdout_gt.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"hello\"]`,\n",
            " b_program label: `&b_program`,\n",
            " b_program debug: `\"bin/printf-stdout\"`,\n",
            "    b_args label: `&b_args`,\n",
            "    b_args debug: `[\"%s%s%s%s%s\", \"h\", \"u\", \"l\", \"l\", \"o\"]`,\n",
            "               a: `\"hello\"`,\n",
            "               b: `\"hullo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is greater than another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (program2 + args2 ⇒ command ⇒ stdout ⇒ string)
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
///
/// # fn main() {
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s%s%s%s", "h", "a", "l", "l", "o"];
/// assert_program_args_stdout_gt!(&a_program, &a_args, &b_program, &b_args);
///
/// # let result = panic::catch_unwind(|| {
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "hello"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s%s%s%s", "h", "u", "l", "l", "o"];
/// assert_program_args_stdout_gt!(&a_program, &a_args, &b_program, &b_args);
/// # });
/// // assertion failed: `assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`
/// // https://docs.rs/assertables/8.11.0/assertables/macro.assert_program_args_stdout_gt.html
/// //  a_program label: `&a_program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `&a_args`,
/// //     a_args debug: `[\"%s\", \"hello\"]`,
/// //  b_program label: `&b_program`,
/// //  b_program debug: `\"bin/printf-stdout\"`,
/// //     b_args label: `&b_args`,
/// //     b_args debug: `[\"%s%s%s%s%s\", \"h\", \"u\", \"l\", \"l\", \"o\"]`,
/// //                a: `\"hello\"`,
/// //                b: `\"hullo\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`\n",
/// #     "https://docs.rs/assertables/8.11.0/assertables/macro.assert_program_args_stdout_gt.html\n",
/// #     " a_program label: `&a_program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `&a_args`,\n",
/// #     "    a_args debug: `[\"%s\", \"hello\"]`,\n",
/// #     " b_program label: `&b_program`,\n",
/// #     " b_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    b_args label: `&b_args`,\n",
/// #     "    b_args debug: `[\"%s%s%s%s%s\", \"h\", \"u\", \"l\", \"l\", \"o\"]`,\n",
/// #     "               a: `\"hello\"`,\n",
/// #     "               b: `\"hullo\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_gt`](macro@crate::assert_program_args_stdout_gt)
/// * [`assert_program_args_stdout_gt_as_result`](macro@crate::assert_program_args_stdout_gt_as_result)
/// * [`debug_assert_program_args_stdout_gt`](macro@crate::debug_assert_program_args_stdout_gt)
///
#[macro_export]
macro_rules! assert_program_args_stdout_gt {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {{
        match $crate::assert_program_args_stdout_gt_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $b_program:expr, $($message:tt)+) => {{
        match $crate::assert_program_args_stdout_gt_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stdout string is greater than another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (program2 + args2 ⇒ command ⇒ stdout ⇒ string)
///
/// This macro provides the same statements as [`assert_program_args_stdout_gt`](macro.assert_program_args_stdout_gt.html),
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
/// * [`assert_program_args_stdout_gt`](macro@crate::assert_program_args_stdout_gt)
/// * [`assert_program_args_stdout_gt`](macro@crate::assert_program_args_stdout_gt)
/// * [`debug_assert_program_args_stdout_gt`](macro@crate::debug_assert_program_args_stdout_gt)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_gt!($($arg)*);
        }
    };
}
