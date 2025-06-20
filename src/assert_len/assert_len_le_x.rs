//! Assert a length is less than or equal to an expression.
//!
//! Pseudocode:<br>
//! a.len() ≤ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "x";
//! let b = 2;
//! assert_len_le_x!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_len_le_x`](macro@crate::assert_len_le_x)
//! * [`assert_len_le_x_as_result`](macro@crate::assert_len_le_x_as_result)
//! * [`debug_assert_len_le_x`](macro@crate::debug_assert_len_le_x)

/// Assert a length is less than or equal to an expression.
///
/// Pseudocode:<br>
/// a.len() ≤ b
///
/// * If true, return Result `Ok((a.len(), b))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_len_le_x`](macro@crate::assert_len_le_x)
/// * [`assert_len_le_x_as_result`](macro@crate::assert_len_le_x_as_result)
/// * [`debug_assert_len_le_x`](macro@crate::debug_assert_len_le_x)
///
#[macro_export]
macro_rules! assert_len_le_x_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            (a, b) => {
                let a_len = a.len();
                if a_len <= b {
                    Ok((a_len, b))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_len_le_x!(a, b)`\n",
                            "https://docs.rs/assertables/",
                            env!("CARGO_PKG_VERSION"),
                            "/assertables/macro.assert_len_le_x.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " a.len(): `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`"
                        ),
                        stringify!($a),
                        a,
                        a_len,
                        stringify!($b),
                        b
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_len_le_x_as_result {
    use std::sync::Once;

    #[test]
    fn le() {
        let a = "x";
        let b = 2;
        for _ in 0..1 {
            let actual = assert_len_le_x_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 2));
        }
    }

    #[test]
    fn le_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "x"
        }

        static B: Once = Once::new();
        fn b() -> usize {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            2
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_len_le_x_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a = "x";
        let b = 1;
        for _ in 0..1 {
            let actual = assert_len_le_x_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 1));
        }
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "x"
        }

        static B: Once = Once::new();
        fn b() -> usize {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            2
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_len_le_x_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn gt() {
        let a = "xx";
        let b = 1;
        let actual = assert_len_le_x_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_len_le_x!(a, b)`\n",
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_len_le_x.html\n",
            " a label: `a`,\n",
            " a debug: `\"xx\"`,\n",
            " a.len(): `2`,\n",
            " b label: `b`,\n",
            " b debug: `1`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a length is less than or equal to an expression.
///
/// Pseudocode:<br>
/// a.len() ≤ b
///
/// * If true, return `(a.len(), b)`.
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
/// let a = "x";
/// let b = 2;
/// assert_len_le_x!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "xx";
/// let b = 1;
/// assert_len_le_x!(a, b);
/// # });
/// // assertion failed: `assert_len_le_x!(a, b)`
/// // https://docs.rs/assertables/9.7.0/assertables/macro.assert_len_le_x.html
/// //  a label: `a`,
/// //  a debug: `\"xx\"`,
/// //  a.len(): `2`",
/// //  b label: `b`,
/// //  b debug: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_len_le_x!(a, b)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_len_le_x.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"xx\"`,\n",
/// #     " a.len(): `2`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `1`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_len_le_x`](macro@crate::assert_len_le_x)
/// * [`assert_len_le_x_as_result`](macro@crate::assert_len_le_x_as_result)
/// * [`debug_assert_len_le_x`](macro@crate::debug_assert_len_le_x)
///
#[macro_export]
macro_rules! assert_len_le_x {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_len_le_x_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_len_le_x_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_len_le_x {
    use std::panic;

    #[test]
    fn gt() {
        let a = "x";
        let b = 2;
        for _ in 0..1 {
            let actual = assert_len_le_x!(a, b);
            assert_eq!(actual, (1, 2));
        }
    }

    #[test]
    fn eq() {
        let a = "x";
        let b = 1;
        for _ in 0..1 {
            let actual = assert_len_le_x!(a, b);
            assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn lt() {
        let a = "xx";
        let b = 1;
        let result = panic::catch_unwind(|| {
            let _actual = assert_len_le_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_len_le_x!(a, b)`\n",
            "https://docs.rs/assertables/",
            env!("CARGO_PKG_VERSION"),
            "/assertables/macro.assert_len_le_x.html\n",
            " a label: `a`,\n",
            " a debug: `\"xx\"`,\n",
            " a.len(): `2`,\n",
            " b label: `b`,\n",
            " b debug: `1`"
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

/// Assert a length is less than or equal to an expression.
///
/// Pseudocode:<br>
/// a.len() ≤ b
///
/// This macro provides the same statements as [`assert_len_le_x`](macro.assert_len_le_x.html),
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
/// * [`assert_len_le_x`](macro@crate::assert_len_le_x)
/// * [`assert_len_le_x`](macro@crate::assert_len_le_x)
/// * [`debug_assert_len_le_x`](macro@crate::debug_assert_len_le_x)
///
#[macro_export]
macro_rules! debug_assert_len_le_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_len_le_x!($($arg)*);
        }
    };
}
