//! Assert a process status code value is less than another.
//!
//! Pseudocode:<br>
//! a ⇒ status ⇒ code ⇒ value < b ⇒ status ⇒ code ⇒ value
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/exit-with-arg"); a.arg("1");
//! let mut b = Command::new("bin/exit-with-arg"); b.arg("2");
//! assert_process_status_code_value_lt!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_process_status_code_value_lt`](macro@crate::assert_process_status_code_value_lt)
//! * [`assert_process_status_code_value_lt_as_result`](macro@crate::assert_process_status_code_value_lt_as_result)
//! * [`debug_assert_process_status_code_value_lt`](macro@crate::debug_assert_process_status_code_value_lt)

/// Assert a process status code value is less than another.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value < b ⇒ status ⇒ code ⇒ value
///
/// * If true, return Result `Ok((a value, b value)))`.
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
/// * [`assert_process_status_code_value_lt`](macro@crate::assert_process_status_code_value_lt)
/// * [`assert_process_status_code_value_lt_as_result`](macro@crate::assert_process_status_code_value_lt_as_result)
/// * [`debug_assert_process_status_code_value_lt`](macro@crate::debug_assert_process_status_code_value_lt)
///
#[macro_export]
macro_rules! assert_process_status_code_value_lt_as_result {
    ($a_process:expr, $b_process:expr $(,)?) => {{
        match ($a_process.status(), $b_process.status()) {
            (Ok(a1), Ok(b1)) => {
                match (a1.code(), b1.code()) {
                    (Some(a2), Some(b2)) => {
                        if a2 < b2 {
                            Ok((a2, b2))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
                                        " a label: `{}`,\n",
                                        " a debug: `{:?}`,\n",
                                        " a value: `{:?}`,\n",
                                        " b label: `{}`,\n",
                                        " b debug: `{:?}`\n",
                                        " b value: `{:?}`",
                                    ),
                                    stringify!($a_process),
                                    $a_process,
                                    a2,
                                    stringify!($b_process),
                                    $b_process,
                                    b2
                                )
                            )
                        }
                    },
                    (a_code, b_code) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`,\n",
                                    "  a code: `{:?}`,\n",
                                    " b label: `{}`,\n",
                                    " b debug: `{:?}`\n",
                                    "  b code: `{:?}`",
                                ),
                                stringify!($a_process),
                                $a_process,
                                a_code,
                                stringify!($b),
                                $b_process,
                                b_code
                            )
                        )
                    }
                }
            },
            (a_status, b_status) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
                            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
                            "  a label: `{}`,\n",
                            "  a debug: `{:?}`,\n",
                            " a status: `{:?}`,\n",
                            "  b label: `{}`,\n",
                            "  b debug: `{:?}`\n",
                            " b status: `{:?}`",
                        ),
                        stringify!($a_process),
                        $a_process,
                        a_status,
                        stringify!($b),
                        $b_process,
                        b_status
                    )
                )
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn lt() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("2");
        let result = assert_process_status_code_value_lt_as_result!(a, b);
        assert_eq!(result, Ok((1, 2)));
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("1");
        let result = assert_process_status_code_value_lt_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
                " a label: `a`,\n",
                " a debug: `\"bin/exit-with-arg\" \"1\"`,\n",
                " a value: `1`,\n",
                " b label: `b`,\n",
                " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
                " b value: `1`"
            )
        );
    }

    #[test]
    fn gt() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("2");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("1");
        let result = assert_process_status_code_value_lt_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
                " a label: `a`,\n",
                " a debug: `\"bin/exit-with-arg\" \"2\"`,\n",
                " a value: `2`,\n",
                " b label: `b`,\n",
                " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
                " b value: `1`"
            )
        );
    }
}

/// Assert a process status code value is less than another.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value < b ⇒ status ⇒ code ⇒ value
///
/// * If true, return `(a value, b value)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// use std::process::Command;
/// # use std::panic;
///
/// # fn main() {
/// let mut a = Command::new("bin/exit-with-arg"); a.arg("1");
/// let mut b = Command::new("bin/exit-with-arg"); b.arg("2");
/// assert_process_status_code_value_lt!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = Command::new("bin/exit-with-arg"); a.arg("2");
/// let mut b = Command::new("bin/exit-with-arg"); b.arg("1");
/// assert_process_status_code_value_lt!(a, b);
/// # });
/// // assertion failed: `assert_process_status_code_value_lt!(a, b)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html
/// //  a label: `a`,
/// //  a debug: `\"bin/exit-with-arg\" \"1\"`,
/// //  a value: `2`",
/// //  b label: `b`,
/// //  b debug: `\"bin/exit-with-arg\" \"1\"`,
/// //  b value: `1`"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_process_status_code_value_lt!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_process_status_code_value_lt.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/exit-with-arg\" \"2\"`,\n",
/// #     " a value: `2`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
/// #     " b value: `1`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_process_status_code_value_lt`](macro@crate::assert_process_status_code_value_lt)
/// * [`assert_process_status_code_value_lt_as_result`](macro@crate::assert_process_status_code_value_lt_as_result)
/// * [`debug_assert_process_status_code_value_lt`](macro@crate::debug_assert_process_status_code_value_lt)
///
#[macro_export]
macro_rules! assert_process_status_code_value_lt {
    ($a_process:expr, $b_process:expr $(,)?) => {{
        match $crate::assert_process_status_code_value_lt_as_result!($a_process, $b_process) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_process:expr, $b_process:expr, $($message:tt)+) => {{
        match $crate::assert_process_status_code_value_lt_as_result!($a_process, $b_process) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a value is greater than an expression.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value < b ⇒ status ⇒ code ⇒ value
///
/// This macro provides the same statements as [`assert_process_status_code_value_lt`](macro.assert_process_status_code_value_lt.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a "bin/exit-with-arg" in an inconsistent state to
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
/// * [`assert_process_status_code_value_lt`](macro@crate::assert_process_status_code_value_lt)
/// * [`assert_process_status_code_value_lt`](macro@crate::assert_process_status_code_value_lt)
/// * [`debug_assert_process_status_code_value_lt`](macro@crate::debug_assert_process_status_code_value_lt)
///
#[macro_export]
macro_rules! debug_assert_process_status_code_value_lt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_process_status_code_value_lt!($($arg)*);
        }
    };
}
