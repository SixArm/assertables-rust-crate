//! Assert a command stderr string is greater than another.
//!
//! Pseudocode:<br>
//! (a_command ⇒ stderr) = (b_command ⇒ stderr)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut a = Command::new("bin/printf-stderr");
//! a.args(["%s", "alfa"]);
//! let mut b = Command::new("bin/printf-stderr");
//! b.args(["%s", "aa"]);
//! assert_command_stderr_gt!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_gt`](macro@crate::assert_command_stderr_gt)
//! * [`assert_command_stderr_gt_as_result`](macro@crate::assert_command_stderr_gt_as_result)
//! * [`debug_assert_command_stderr_gt`](macro@crate::debug_assert_command_stderr_gt)

/// Assert a command stderr string is greater than another.
///
/// Pseudocode:<br>
/// (a_command ⇒ stderr) = (b_command ⇒ stderr)
///
/// * If true, return Result `Ok(stderr)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stderr_gt`](macro@crate::assert_command_stderr_gt)
/// * [`assert_command_stderr_gt_as_result`](macro@crate::assert_command_stderr_gt_as_result)
/// * [`debug_assert_command_stderr_gt`](macro@crate::debug_assert_command_stderr_gt)
///
#[macro_export]
macro_rules! assert_command_stderr_gt_as_result {
    ($a_command:expr, $b_command:expr $(,)?) => {
        match ($a_command.output(), $b_command.output()) {
            (Ok(a), Ok(b)) => {
                let a = a.stderr;
                let b = b.stderr;
                if a.gt(&b) {
                    Ok((a, b))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
                            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " a value: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            " b value: `{:?}`"
                        ),
                        stringify!($a_command),
                        $a_command,
                        a,
                        stringify!($b_command),
                        $b_command,
                        b
                    ))
                }
            }
            (a, b) => Err(format!(
                concat!(
                    "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
                    "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`,\n",
                    " a value: `{:?}`,\n",
                    " b label: `{}`,\n",
                    " b debug: `{:?}`,\n",
                    " b value: `{:?}`"
                ),
                stringify!($a_command),
                $a_command,
                a,
                stringify!($b_command),
                $b_command,
                b
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_command_stderr_gt_as_result {
    use std::process::Command;
    use std::sync::Once;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "aa"]);
        for _ in 0..1 {
            let actual = assert_command_stderr_gt_as_result!(a, b);
            assert_eq!(
                actual.unwrap(),
                (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a'])
            );
        }
    }

    #[test]
    fn gt_once() {
        static A: Once = Once::new();
        fn a() -> Command {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            let mut a = Command::new("bin/printf-stderr");
            a.args(["%s", "alfa"]);
            a
        }

        static B: Once = Once::new();
        fn b() -> Command {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            let mut b = Command::new("bin/printf-stderr");
            b.args(["%s", "aa"]);
            b
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_command_stderr_gt_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "alfa"]);
        let actual = assert_command_stderr_gt_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " a value: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " b value: `[97, 108, 102, 97]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn lt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "zz"]);
        let actual = assert_command_stderr_gt_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " a value: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,\n",
            " b value: `[122, 122]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command stderr string is greater than another.
///
/// Pseudocode:<br>
/// (a_command ⇒ stderr) = (b_command ⇒ stderr)
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
/// use std::process::Command;
///
/// # fn main() {
/// let mut a = Command::new("bin/printf-stderr");
/// a.args(["%s", "alfa"]);
/// let mut b = Command::new("bin/printf-stderr");
/// b.args(["%s", "aa"]);
/// assert_command_stderr_gt!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = Command::new("bin/printf-stderr");
/// a.args(["%s", "alfa"]);
/// let mut b = Command::new("bin/printf-stderr");
/// b.args(["%s", "zz"]);
/// assert_command_stderr_gt!(a, b);
/// # });
/// // assertion failed: `assert_command_stderr_gt!(a_command, b_command)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_command_stderr_gt.html
/// //  a label: `a`,
/// //  a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,
/// //  b label: `b`,
/// //  b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,
/// //  a value: `[97, 108, 102, 97]`,
/// //  b value: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
/// #     " a value: `[97, 108, 102, 97]`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,\n",
/// #     " b value: `[122, 122]`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_gt`](macro@crate::assert_command_stderr_gt)
/// * [`assert_command_stderr_gt_as_result`](macro@crate::assert_command_stderr_gt_as_result)
/// * [`debug_assert_command_stderr_gt`](macro@crate::debug_assert_command_stderr_gt)
///
#[macro_export]
macro_rules! assert_command_stderr_gt {
    ($a_command:expr, $b_command:expr $(,)?) => {
        match $crate::assert_command_stderr_gt_as_result!($a_command, $b_command) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_command:expr, $b_command:expr, $($message:tt)+) => {
        match $crate::assert_command_stderr_gt_as_result!($a_command, $b_command) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_command_stderr_gt {
    use std::panic;
    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "aa"]);
        for _ in 0..1 {
            let actual = assert_command_stderr_gt!(a, b);
            assert_eq!(actual, (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a']));
        }
    }

    #[test]
    fn eq() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stderr");
            a.args(["%s", "alfa"]);
            let mut b = Command::new("bin/printf-stderr");
            b.args(["%s", "alfa"]);
            let _actual = assert_command_stderr_gt!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " a value: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " b value: `[97, 108, 102, 97]`"
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

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stderr");
            a.args(["%s", "alfa"]);
            let mut b = Command::new("bin/printf-stderr");
            b.args(["%s", "zz"]);
            let _actual = assert_command_stderr_gt!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stderr_gt!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_command_stderr_gt.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " a value: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,\n",
            " b value: `[122, 122]`"
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

/// Assert a command stderr string is greater than another.
///
/// This macro provides the same statements as [`assert_command_stderr_gt {`](macro.assert_command_stderr_gt {.html),
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
/// * [`assert_command_stderr_gt {`](macro@crate::assert_command_stderr_gt {)
/// * [`assert_command_stderr_gt {`](macro@crate::assert_command_stderr_gt {)
/// * [`debug_assert_command_stderr_gt {`](macro@crate::debug_assert_command_stderr_gt {)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_gt!($($arg)*);
        }
    };
}
