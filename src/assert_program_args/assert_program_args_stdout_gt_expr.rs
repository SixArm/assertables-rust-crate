//! Assert a command (built with program and args) stdout string is greater than an expression.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (expr into string)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let program = "bin/printf-stdout";
//! let args = ["%s", "hello"];
//! let s = String::from("hallo");
//! assert_program_args_stdout_gt_expr!(&program, &args, s);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_gt_expr`](macro@crate::assert_program_args_stdout_gt_expr)
//! * [`assert_program_args_stdout_gt_expr_as_result`](macro@crate::assert_program_args_stdout_gt_expr_as_result)
//! * [`debug_assert_program_args_stdout_gt_expr`](macro@crate::debug_assert_program_args_stdout_gt_expr)

/// Assert a command (built with program and args) stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (expr into string)
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
/// * [`assert_program_args_stdout_gt_expr`](macro@crate::assert_program_args_stdout_gt_expr)
/// * [`assert_program_args_stdout_gt_expr_as_result`](macro@crate::assert_program_args_stdout_gt_expr_as_result)
/// * [`debug_assert_program_args_stdout_gt_expr`](macro@crate::debug_assert_program_args_stdout_gt_expr)
///
#[macro_export]
macro_rules! assert_program_args_stdout_gt_expr_as_result {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {{
        match ($a_program, $a_args, $b_expr) {
            (a_program, a_args, b_expr) => {
                let a_output = assert_program_args_impl_prep!(a_program, a_args);
                if a_output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_program_args_stdout_gt_expr!(a_program, a_args, b_expr)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stdout_gt_expr.html\n",
                            " a_program label: `{}`,\n",
                            " a_program debug: `{:?}`,\n",
                            "    a_args label: `{}`,\n",
                            "    a_args debug: `{:?}`,\n",
                            "    b_expr label: `{}`,\n",
                            "    b_expr debug: `{:?}`,\n",
                            "        a output: `{:?}`"
                        ),
                        stringify!($a_program),
                        a_program,
                        stringify!($a_args),
                        a_args,
                        stringify!($b_expr),
                        b_expr,
                        a_output
                    ))
                } else {
                    let a_string = String::from_utf8(a_output.unwrap().stdout).unwrap();
                    if a_string > b_expr {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_program_args_stdout_gt_expr!(a_program, a_args, b_expr)`\n",
                                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stdout_gt_expr.html\n",
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
                            b_expr,
                            a_string,
                            b_expr
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
    fn test_assert_program_args_stdout_gt_expr_as_result_x_success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b = String::from("hallo");
        let result = assert_program_args_stdout_gt_expr_as_result!(&a_program, &a_args, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_program_args_stdout_gt_expr_as_result_x_failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "hello"];
        let b = String::from("hello");
        let result = assert_program_args_stdout_gt_expr_as_result!(&a_program, &a_args, b);
        let actual = result.unwrap_err();
        let expect = concat!(
          "assertion failed: `assert_program_args_stdout_gt_expr!(a_program, a_args, b_expr)`\n",
          "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stdout_gt_expr.html\n",
          " a_program label: `&a_program`,\n",
          " a_program debug: `\"bin/printf-stdout\"`,\n",
          "    a_args label: `&a_args`,\n",
          "    a_args debug: `[\"%s\", \"hello\"]`,\n",
          "    b_expr label: `b`,\n",
          "    b_expr debug: `\"hello\"`,\n",
          "               a: `\"hello\"`,\n",
          "               b: `\"hello\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (expr into string)
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
/// let program = "bin/printf-stdout";
/// let args = ["%s", "hello"];
/// let s = String::from("hallo");
/// assert_program_args_stdout_gt_expr!(&program, &args, s);
///
/// # let result = panic::catch_unwind(|| {
/// let program = "bin/printf-stdout";
/// let args = ["%s", "hello"];
/// let s = String::from("hullo");
/// assert_program_args_stdout_gt_expr!(&program, &args, s);
/// # });
/// // assertion failed: `assert_program_args_stdout_gt_expr!(a_program, a_args, b_expr)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stdout_gt_expr.html
/// //  a_program label: `&program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `&args`,
/// //     a_args debug: `[\"%s\", \"hello\"]`,
/// //     b_expr label: `s`,
/// //     b_expr debug: `\"hullo\"`,
/// //                a: `\"hello\"`,
/// //                b: `\"hullo\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stdout_gt_expr!(a_program, a_args, b_expr)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_program_args_stdout_gt_expr.html\n",
/// #     " a_program label: `&program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `&args`,\n",
/// #     "    a_args debug: `[\"%s\", \"hello\"]`,\n",
/// #     "    b_expr label: `s`,\n",
/// #     "    b_expr debug: `\"hullo\"`,\n",
/// #     "               a: `\"hello\"`,\n",
/// #     "               b: `\"hullo\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_gt_expr`](macro@crate::assert_program_args_stdout_gt_expr)
/// * [`assert_program_args_stdout_gt_expr_as_result`](macro@crate::assert_program_args_stdout_gt_expr_as_result)
/// * [`debug_assert_program_args_stdout_gt_expr`](macro@crate::debug_assert_program_args_stdout_gt_expr)
///
#[macro_export]
macro_rules! assert_program_args_stdout_gt_expr {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {{
        match assert_program_args_stdout_gt_expr_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $b_expr:expr, $($message:tt)+) => {{
        match assert_program_args_stdout_gt_expr_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout ⇒ string) > (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stdout_gt_expr`](macro.assert_program_args_stdout_gt_expr.html),
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
/// * [`assert_program_args_stdout_gt_expr`](macro@crate::assert_program_args_stdout_gt_expr)
/// * [`assert_program_args_stdout_gt_expr`](macro@crate::assert_program_args_stdout_gt_expr)
/// * [`debug_assert_program_args_stdout_gt_expr`](macro@crate::debug_assert_program_args_stdout_gt_expr)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_gt_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_gt_expr!($($arg)*);
        }
    };
}
