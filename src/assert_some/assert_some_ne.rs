//! Assert two expressions are Some and their values are not equal.
//!
//! Pseudocode:<br>
//! (a ⇒ Some(a1) ⇒ a1) ≠ (b ⇒ Some(b1) ⇒ b1)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Option<i8> = Option::Some(1);
//! let b: Option<i8> = Option::Some(2);
//! assert_some_ne!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_some_ne`](macro@crate::assert_some_ne)
//! * [`assert_some_ne_as_result`](macro@crate::assert_some_ne_as_result)
//! * [`debug_assert_some_ne`](macro@crate::debug_assert_some_ne)

/// Assert two expressions are Some and their values are not equal.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a1) ⇒ a1) ≠ (b ⇒ Some(b1) ⇒ b1)
///
/// * If true, return Result `Ok((a1, b1)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_some_ne`](macro.assert_some_ne.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_some_ne`](macro@crate::assert_some_ne)
/// * [`assert_some_ne_as_result`](macro@crate::assert_some_ne_as_result)
/// * [`debug_assert_some_ne`](macro@crate::debug_assert_some_ne)
///
#[macro_export]
macro_rules! assert_some_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            (Some(a1), Some(b1)) => {
                if a1 != b1 {
                    Ok((a1, b1))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_some_ne!(a, b)`\n",
                            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " a inner: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            " b inner: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        a1,
                        stringify!($b),
                        $b,
                        b1
                    ))
                }
            }
            _ => Err(format!(
                concat!(
                    "assertion failed: `assert_some_ne!(a, b)`\n",
                    "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`,\n",
                    " b label: `{}`,\n",
                    " b debug: `{:?}`",
                ),
                stringify!($a),
                $a,
                stringify!($b),
                $b,
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_some_ne_as_result {
    use std::sync::Once;

    #[test]
    fn lt() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(2);
        for _ in 0..1 {
            let actual = assert_some_ne_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 2));
        }
    }

    #[test]
    fn lt_once() {
        static A: Once = Once::new();
        fn a() -> Option<i8> {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Option::Some(1)
        }

        static B: Once = Once::new();
        fn b() -> Option<i8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            Option::Some(2)
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_some_ne_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn gt() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(2);
        for _ in 0..1 {
            let actual = assert_some_ne_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 2));
        }
    }

    #[test]
    fn gt_once() {
        static A: Once = Once::new();
        fn a() -> Option<i8> {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Option::Some(2)
        }

        static B: Once = Once::new();
        fn b() -> Option<i8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            Option::Some(1)
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_some_ne_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(1);
        let actual = assert_some_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_some_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Some(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Some(1)`,\n",
            " b inner: `1`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn not_some() {
        let a: Option<i8> = Option::None;
        let b: Option<i8> = Option::Some(1);
        let actual = assert_some_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_some_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
            " a label: `a`,\n",
            " a debug: `None`,\n",
            " b label: `b`,\n",
            " b debug: `Some(1)`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert two expressions are Some and their values are not equal.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a1) ⇒ a1) ≠ (b ⇒ Some(b1) ⇒ b1)
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
/// let a: Option<i8> = Option::Some(1);
/// let b: Option<i8> = Option::Some(2);
/// assert_some_ne!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Option<i8> = Option::Some(1);
/// let b: Option<i8> = Option::Some(1);
/// assert_some_ne!(a, b);
/// # });
/// // assertion failed: `assert_some_ne!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_some_ne.html
/// //  a label: `a`,
/// //  a debug: `Some(1)`,
/// //  b label: `b`,
/// //  b debug: `Some(1)`,
/// //  a inner: `1`,
/// //  b inner: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_some_ne!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Some(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Some(1)`,\n",
/// #     " b inner: `1`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_some_ne`](macro@crate::assert_some_ne)
/// * [`assert_some_ne_as_result`](macro@crate::assert_some_ne_as_result)
/// * [`debug_assert_some_ne`](macro@crate::debug_assert_some_ne)
///
#[macro_export]
macro_rules! assert_some_ne {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_some_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_some_ne_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_some_ne {
    use std::panic;

    #[test]
    fn ne() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(2);
        for _ in 0..1 {
            let actual = assert_some_ne!(a, b);
            assert_eq!(actual, (1, 2));
        }
    }

    #[test]
    fn eq() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(1);
        let result = panic::catch_unwind(|| {
            let _actual = assert_some_ne!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_some_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
            " a label: `a`,\n",
            " a debug: `Some(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Some(1)`,\n",
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
    fn not_some() {
        let a: Option<i8> = Option::None;
        let b: Option<i8> = Option::Some(1);
        let result = panic::catch_unwind(|| {
            let _actual = assert_some_ne!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_some_ne!(a, b)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_some_ne.html\n",
            " a label: `a`,\n",
            " a debug: `None`,\n",
            " b label: `b`,\n",
            " b debug: `Some(1)`",
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

/// Assert two expressions are Some and their values are not equal.
///
/// Pseudocode:<br>
/// (a ⇒ Some(a1) ⇒ a1) ≠ (b ⇒ Some(b1) ⇒ b1)
///
/// This macro provides the same statements as [`assert_some_ne`](macro.assert_some_ne.html),
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
/// * [`assert_some_ne`](macro@crate::assert_some_ne)
/// * [`assert_some_ne`](macro@crate::assert_some_ne)
/// * [`debug_assert_some_ne`](macro@crate::debug_assert_some_ne)
///
#[macro_export]
macro_rules! debug_assert_some_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_some_ne!($($arg)*);
        }
    };
}
