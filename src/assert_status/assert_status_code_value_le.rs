//! Assert a status code value is less than or equal to another.
//!
//! Pseudocode:<br>
//! a ⇒ status ⇒ code ⇒ value ≤ b ⇒ status ⇒ code ⇒ value
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut a = Command::new("bin/exit-with-arg"); a.arg("1");
//! let mut b = Command::new("bin/exit-with-arg"); b.arg("2");
//! assert_status_code_value_le!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_status_code_value_le`](macro@crate::assert_status_code_value_le)
//! * [`assert_status_code_value_le_as_result`](macro@crate::assert_status_code_value_le_as_result)
//! * [`debug_assert_status_code_value_le`](macro@crate::debug_assert_status_code_value_le)

/// Assert a status code value is less than or equal to another.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value ≤ b ⇒ status ⇒ code ⇒ value
///
/// * If true, return Result `Ok((a value, b value)))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_status_code_value_le`](macro@crate::assert_status_code_value_le)
/// * [`assert_status_code_value_le_as_result`](macro@crate::assert_status_code_value_le_as_result)
/// * [`debug_assert_status_code_value_le`](macro@crate::debug_assert_status_code_value_le)
///
#[macro_export]
macro_rules! assert_status_code_value_le_as_result {
    ($a_process:expr, $b_process:expr $(,)?) => {
        match ($a_process.status(), $b_process.status()) {
            (Ok(a_status), Ok(b_status)) => {
                match (a_status.code(), b_status.code()) {
                    (Some(a_code), Some(b_code)) => {
                        if a_code <= b_code {
                            Ok((a_code, b_code))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_status_code_value_le!(a, b)`\n",
                                        "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
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
                                    stringify!($b_process),
                                    $b_process,
                                    b_code,
                                )
                            )
                        }
                    },
                    _ => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_status_code_value_le!(a, b)`\n",
                                    "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`,\n",
                                    " b label: `{}`,\n",
                                    " b debug: `{:?}`",
                                ),
                                stringify!($a_process),
                                $a_process,
                                stringify!($b),
                                $b_process,
                            )
                        )
                    }
                }
            },
            _ => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_status_code_value_le!(a, b)`\n",
                            "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
                            "  a label: `{}`,\n",
                            "  a debug: `{:?}`,\n",
                            "  b label: `{}`,\n",
                            "  b debug: `{:?}`",
                        ),
                        stringify!($a_process),
                        $a_process,
                        stringify!($b),
                        $b_process,
                    )
                )
            }
        }
    };
}

#[cfg(test)]
mod test_assert_status_code_value_le_as_result {
    use std::sync::Once;
    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("2");
        let actual = assert_status_code_value_le_as_result!(a, b);
        assert_eq!(actual.unwrap(), (1, 2));
    }

    #[test]
    fn lt_once() {

        static A: Once = Once::new();
        fn a() -> Command {
            if A.is_completed() { panic!("A.is_completed()") } else { A.call_once(|| {}) }
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("1");
            a
        }

        static B: Once = Once::new();
        fn b() -> Command {
            if B.is_completed() { panic!("B.is_completed()") } else { B.call_once(|| {}) }
            let mut b = Command::new("bin/exit-with-arg");
            b.arg("2");
            b
        }
        
        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_status_code_value_le_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("1");
        let actual = assert_status_code_value_le_as_result!(a, b);
        assert_eq!(actual.unwrap(), (1, 1));
    }

    #[test]
    fn eq_once() {

        static A: Once = Once::new();
        fn a() -> Command {
            if A.is_completed() { panic!("A.is_completed()") } else { A.call_once(|| {}) }
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("1");
            a
        }

        static B: Once = Once::new();
        fn b() -> Command {
            if B.is_completed() { panic!("B.is_completed()") } else { B.call_once(|| {}) }
            let mut b = Command::new("bin/exit-with-arg");
            b.arg("1");
            b
        }
        
        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_status_code_value_le_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        
    }

    #[test]
    fn lt() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("2");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("1");
        let actual = assert_status_code_value_le_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_status_code_value_le!(a, b)`\n",
            "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/exit-with-arg\" \"2\"`,\n",
            "  a code: `2`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
            "  b code: `1`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a status code value is less than or equal to another.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value ≤ b ⇒ status ⇒ code ⇒ value
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
/// assert_status_code_value_le!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = Command::new("bin/exit-with-arg"); a.arg("2");
/// let mut b = Command::new("bin/exit-with-arg"); b.arg("1");
/// assert_status_code_value_le!(a, b);
/// # });
/// // assertion failed: `assert_status_code_value_le!(a, b)`
/// // https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html
/// //  a label: `a`,
/// //  a debug: `\"bin/exit-with-arg\" \"1\"`,
/// //  a value: `2`",
/// //  b label: `b`,
/// //  b debug: `\"bin/exit-with-arg\" \"1\"`,
/// //  b value: `1`"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_status_code_value_le!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/exit-with-arg\" \"2\"`,\n",
/// #     "  a code: `2`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
/// #     "  b code: `1`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_status_code_value_le`](macro@crate::assert_status_code_value_le)
/// * [`assert_status_code_value_le_as_result`](macro@crate::assert_status_code_value_le_as_result)
/// * [`debug_assert_status_code_value_le`](macro@crate::debug_assert_status_code_value_le)
///
#[macro_export]
macro_rules! assert_status_code_value_le {
    ($a_process:expr, $b_process:expr $(,)?) => {
        match $crate::assert_status_code_value_le_as_result!($a_process, $b_process) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_process:expr, $b_process:expr, $($message:tt)+) => {
        match $crate::assert_status_code_value_le_as_result!($a_process, $b_process) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_status_code_value_le {
    use std::panic;
    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("2");
        let actual = assert_status_code_value_le!(a, b);
        assert_eq!(actual, (1, 2));
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let mut b = Command::new("bin/exit-with-arg");
        b.arg("1");
        let actual = assert_status_code_value_le!(a, b);
        assert_eq!(actual, (1, 1));
    }

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("2");
            let mut b = Command::new("bin/exit-with-arg");
            b.arg("1");
            let _actual = assert_status_code_value_le!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_status_code_value_le!(a, b)`\n",
            "https://docs.rs/assertables/9.5.6/assertables/macro.assert_status_code_value_le.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/exit-with-arg\" \"2\"`,\n",
            "  a code: `2`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/exit-with-arg\" \"1\"`\n",
            "  b code: `1`"
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert a status code value is less than or equal to another.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ code ⇒ value ≤ b ⇒ status ⇒ code ⇒ value
///
/// This macro provides the same statements as [`assert_status_code_value_le`](macro.assert_status_code_value_le.html),
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
/// * [`assert_status_code_value_le`](macro@crate::assert_status_code_value_le)
/// * [`assert_status_code_value_le`](macro@crate::assert_status_code_value_le)
/// * [`debug_assert_status_code_value_le`](macro@crate::debug_assert_status_code_value_le)
///
#[macro_export]
macro_rules! debug_assert_status_code_value_le {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_status_code_value_le!($($arg)*);
        }
    };
}
