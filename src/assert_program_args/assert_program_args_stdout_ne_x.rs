//! Assert a command (built with program and args) stdout string is not equal to an expression.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stdout) ≠ (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let program = "bin/printf-stdout";
//! let args = ["%s", "alfa"];
//! let bytes = vec![b'z', b'z'];
//! assert_program_args_stdout_ne_x!(&program, &args, bytes);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_ne_x`](macro@crate::assert_program_args_stdout_ne_x)
//! * [`assert_program_args_stdout_ne_x_as_result`](macro@crate::assert_program_args_stdout_ne_x_as_result)
//! * [`debug_assert_program_args_stdout_ne_x`](macro@crate::debug_assert_program_args_stdout_ne_x)

/// Assert a command (built with program and args) stdout string is not equal to an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≠ (expr into string)
///
/// * If true, return Result `Ok(stdout)`.
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
/// * [`assert_program_args_stdout_ne_x`](macro@crate::assert_program_args_stdout_ne_x)
/// * [`assert_program_args_stdout_ne_x_as_result`](macro@crate::assert_program_args_stdout_ne_x_as_result)
/// * [`debug_assert_program_args_stdout_ne_x`](macro@crate::debug_assert_program_args_stdout_ne_x)
///
#[macro_export]
macro_rules! assert_program_args_stdout_ne_x_as_result {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {{
        match ($a_program, $a_args, &$b_expr) {
            (a_program, a_args, b_expr) => {
                match assert_program_args_impl_prep!(a_program, a_args) {
                    Ok(a_output) => {
                        let a = a_output.stdout;
                        if a.ne(&$b_expr) {
                            Ok(a)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stdout_ne_x!(a_program, a_args, b_expr)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_ne_x.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        "    b_expr label: `{}`,\n",
                                        "    b_expr debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($b_expr),
                                    $b_expr,
                                    a,
                                    b_expr
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stdout_ne_x!(a_program, a_args, b_expr)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_ne_x.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    "    b_expr label: `{}`,\n",
                                    "    b_expr debug: `{:?}`,\n",
                                    "             err: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($b_expr),
                                $b_expr,
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
    fn test_assert_program_args_stdout_ne_x_expr_as_result_x_success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'z', b'z'];
        let result = assert_program_args_stdout_ne_x_as_result!(&a_program, &a_args, b);
        assert_eq!(result.unwrap(), vec![b'a', b'l', b'f', b'a']);
    }

    #[test]
    fn test_assert_program_args_stdout_ne_x_expr_as_result_x_failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'l', b'f', b'a'];
        let result = assert_program_args_stdout_ne_x_as_result!(&a_program, &a_args, b);
        let actual = result.unwrap_err();
        let expect = concat!(
          "assertion failed: `assert_program_args_stdout_ne_x!(a_program, a_args, b_expr)`\n",
          "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_ne_x.html\n",
          " a_program label: `&a_program`,\n",
          " a_program debug: `\"bin/printf-stdout\"`,\n",
          "    a_args label: `&a_args`,\n",
          "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
          "    b_expr label: `b`,\n",
          "    b_expr debug: `[97, 108, 102, 97]`,\n",
          "               a: `[97, 108, 102, 97]`,\n",
          "               b: `[97, 108, 102, 97]`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is not equal to an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≠ (expr into string)
///
/// * If true, return `(stdout)`.
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
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let bytes = vec![b'z', b'z'];
/// assert_program_args_stdout_ne_x!(&program, &args, bytes);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let bytes = vec![b'a', b'l', b'f', b'a'];
/// assert_program_args_stdout_ne_x!(&program, &args, bytes);
/// # });
/// // assertion failed: `assert_program_args_stdout_ne_x!(a_program, a_args, b_expr)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_ne_x.html
/// //  a_program label: `&program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `&args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //     b_expr label: `bytes`,
/// //     b_expr debug: `[97, 108, 102, 97]`,
/// //                a: `[97, 108, 102, 97]`,
/// //                b: `[97, 108, 102, 97]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stdout_ne_x!(a_program, a_args, b_expr)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_program_args_stdout_ne_x.html\n",
/// #     " a_program label: `&program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `&args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     "    b_expr label: `bytes`,\n",
/// #     "    b_expr debug: `[97, 108, 102, 97]`,\n",
/// #     "               a: `[97, 108, 102, 97]`,\n",
/// #     "               b: `[97, 108, 102, 97]`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_ne_x`](macro@crate::assert_program_args_stdout_ne_x)
/// * [`assert_program_args_stdout_ne_x_as_result`](macro@crate::assert_program_args_stdout_ne_x_as_result)
/// * [`debug_assert_program_args_stdout_ne_x`](macro@crate::debug_assert_program_args_stdout_ne_x)
///
#[macro_export]
macro_rules! assert_program_args_stdout_ne_x {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {{
        match $crate::assert_program_args_stdout_ne_x_as_result!($a_program, $a_args, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_program_args_stdout_ne_x_as_result!($a_program, $a_args, $b_expr) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stdout string is not equal to an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≠ (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stdout_ne_x`](macro.assert_program_args_stdout_ne_x.html),
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
/// * [`assert_program_args_stdout_ne_x`](macro@crate::assert_program_args_stdout_ne_x)
/// * [`assert_program_args_stdout_ne_x`](macro@crate::assert_program_args_stdout_ne_x)
/// * [`debug_assert_program_args_stdout_ne_x`](macro@crate::debug_assert_program_args_stdout_ne_x)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_ne_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_ne_x!($($arg)*);
        }
    };
}
