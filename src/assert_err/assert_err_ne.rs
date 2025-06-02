//! Assert two expressions are Err and their values are not equal.
//!
//! Pseudocode:<br>
//! (a ⇒ Err(a1) ⇒ a1) ≠ (b ⇒ Err(b1) ⇒ b1)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Result<i8, i8> = Err(1);
//! let b: Result<i8, i8> = Err(2);
//! assert_err_ne!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_err_ne`](macro@crate::assert_err_ne)
//! * [`assert_err_ne_as_result`](macro@crate::assert_err_ne_as_result)
//! * [`debug_assert_err_ne`](macro@crate::debug_assert_err_ne)

/// Assert two expressions are Err and their values are not equal.
///
/// Pseudocode:<br>
/// (a ⇒ Err(a1) ⇒ a1) ≠ (b ⇒ Err(b1) ⇒ b1)
///
/// * If true, return Result `Ok((a1, b1))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_err_ne`](macro@crate::assert_err_ne)
/// * [`assert_err_ne_as_result`](macro@crate::assert_err_ne_as_result)
/// * [`debug_assert_err_ne`](macro@crate::debug_assert_err_ne)
///
#[macro_export]
macro_rules! assert_err_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        let a = ($a);
        let b = ($b);
        match (a, b) {
            (Err(a1), Err(b1)) => {
                if a1 != b1 {
                    Ok((a1, b1))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_err_ne!(a, b)`\n",
                                "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " a inner: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                " b inner: `{:?}`"
                            ),
                            stringify!($a),
                            a,
                            a1,
                            stringify!($b),
                            b,
                            b1
                        )
                    )
                }
            },
            _ => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_err_ne!(a, b)`\n",
                            "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b,
                    )
                )
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_err_ne_as_result {

    #[test]
    fn success() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Err(2);
        let actual = assert_err_ne_as_result!(a, b);
        assert_eq!(actual.unwrap(), (1, 2));
    }

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Err(1);
        let actual = assert_err_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_err_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Err(1)`,\n",
            " b inner: `1`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn failure_because_not_err() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Err(1);
        let actual = assert_err_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_err_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " b label: `b`,\n",
            " b debug: `Err(1)`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn idempotent() {
        let a = 100;
        let b = 200;
        let a_atomic = std::sync::atomic::AtomicU32::new(a);
        let a_increment = || Err::<u32, u32>(a_atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let b_atomic = std::sync::atomic::AtomicU32::new(b);
        let b_increment = || Err::<u32, u32>(b_atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let _ = assert_err_ne_as_result!(a_increment(), b_increment());
        assert_eq!(a_atomic.load(std::sync::atomic::Ordering::SeqCst), a + 1);
        assert_eq!(b_atomic.load(std::sync::atomic::Ordering::SeqCst), b + 1);
    }

}

/// Assert two expressions are Err and their values are not equal.
///
/// Pseudocode:<br>
/// (a ⇒ Err(a1) ⇒ a1) ≠ (b ⇒ Err(b1) ⇒ b1)
///
/// * If true, return `(a1, b1)`.
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
/// let a: Result<i8, i8> = Err(1);
/// let b: Result<i8, i8> = Err(2);
/// assert_err_ne!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Result<i8, i8> = Err(1);
/// let b: Result<i8, i8> = Err(1);
/// assert_err_ne!(a, b);
/// # });
/// // assertion failed: `assert_err_ne!(a, b)`
/// // https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html
/// //  a label: `a`,
/// //  a debug: `Err(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `Err(1)`,
/// //  b inner: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_err_ne!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Err(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Err(1)`,\n",
/// #     " b inner: `1`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_err_ne`](macro@crate::assert_err_ne)
/// * [`assert_err_ne_as_result`](macro@crate::assert_err_ne_as_result)
/// * [`debug_assert_err_ne`](macro@crate::debug_assert_err_ne)
///
#[macro_export]
macro_rules! assert_err_ne {
    ($a:expr, $b:expr $(,)?) => {{
        match $crate::assert_err_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_err_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_err_ne {
    use std::panic;

    #[test]
    fn success() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Err(2);
        let actual = assert_err_ne!(a, b);
        assert_eq!(actual, (1, 2));
    }

    #[test]
    fn eq() {
        let result = panic::catch_unwind(|| {
            let a: Result<i8, i8> = Err(1);
            let b: Result<i8, i8> = Err(1);
            let _actual = assert_err_ne!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_err_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Err(1)`,\n",
            " b inner: `1`",
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
    fn failure_because_not_err() {
        let result = panic::catch_unwind(|| {
            let a: Result<i8, i8> = Ok(1);
            let b: Result<i8, i8> = Err(1);
            let _actual = assert_err_ne!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_err_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.5.3/assertables/macro.assert_err_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " b label: `b`,\n",
            " b debug: `Err(1)`",
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

/// Assert two expressions are Err and their values are not equal.
///
/// This macro provides the same statements as [`assert_err_ne`](macro.assert_err_ne.html),
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
/// * [`assert_err_ne`](macro@crate::assert_err_ne)
/// * [`assert_err_ne`](macro@crate::assert_err_ne)
/// * [`debug_assert_err_ne`](macro@crate::debug_assert_err_ne)
///
#[macro_export]
macro_rules! debug_assert_err_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_err_ne!($($arg)*);
        }
    };
}
