//! Assert a poll ready value is equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::task::Poll;
//! # fn main() {
//! let a: Poll<i8> = Poll::Ready(1);
//! let b: Poll<i8> = Poll::Ready(1);
//! assert_poll_ready_eq!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_poll_ready_eq`](macro@crate::assert_poll_ready_eq)
//! * [`assert_poll_ready_eq_as_result`](macro@crate::assert_poll_ready_eq_as_result)
//! * [`debug_assert_poll_ready_eq`](macro@crate::debug_assert_poll_ready_eq)

/// Assert a poll ready value is equal to another.
///
/// * If true, return Result `Some(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_poll_ready_eq`](macro.assert_poll_ready_eq.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_poll_ready_eq`](macro@crate::assert_poll_ready_eq)
/// * [`assert_poll_ready_eq_as_result`](macro@crate::assert_poll_ready_eq_as_result)
/// * [`debug_assert_poll_ready_eq`](macro@crate::debug_assert_poll_ready_eq)
///
#[macro_export]
macro_rules! assert_poll_ready_eq_as_result {
    ($a_poll:expr, $b_poll:expr $(,)?) => {
        match (&$a_poll, &$b_poll) {
            (Poll::Ready(a), Poll::Ready(b)) =>
                if a == b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_poll_ready_eq!(a, b)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "       a: `{:?}`,\n",
                            "       b: `{:?}`"
                        ),
                        stringify!($a_poll),
                        $a_poll,
                        stringify!($b_poll),
                        $b_poll,
                        a,
                        b
                    ))
                }
            _ =>
                Err(format!(
                    concat!(
                        "assertion failed: `assert_poll_ready_eq!(a, b)`\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`,\n",
                        " b label: `{}`,\n",
                        " b debug: `{:?}`",
                    ),
                    stringify!($a_poll),
                    $a_poll,
                    stringify!($b_poll),
                    $b_poll
                ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::task::Poll;

    #[test]
    fn test_assert_poll_ready_eq_as_result_x_success() {
        let a: Poll<i8> = Poll::Ready(1);
        let b: Poll<i8> = Poll::Ready(1);
        let result = assert_poll_ready_eq_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_poll_ready_eq_as_result_x_failure_because_ne() {
        let a: Poll<i8> = Poll::Ready(1);
        let b: Poll<i8> = Poll::Ready(2);
        let result = assert_poll_ready_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_poll_ready_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ready(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Ready(2)`,\n",
                "       a: `1`,\n",
                "       b: `2`",
            )
        );
    }

    #[test]
    fn test_assert_poll_ready_eq_as_result_x_failure_because_not_ready() {
        let a: Poll<i8> = Poll::Ready(1);
        let b: Poll<i8> = Poll::Pending;
        let result = assert_poll_ready_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_poll_ready_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Ready(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Pending`",
            )
        );
    }

}

/// Assert a poll ready value is equal to another.
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
/// use std::task::Poll;
/// # fn main() {
/// let a: Poll<i8> = Poll::Ready(1);
/// let b: Poll<i8> = Poll::Ready(1);
/// assert_poll_ready_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Poll<i8> = Poll::Ready(1);
/// let b: Poll<i8> = Poll::Ready(2);
/// assert_poll_ready_eq!(a, b);
/// # });
/// // assertion failed: `assert_poll_ready_eq!(a, b)`
/// //  a label: `a`,
/// //  a debug: `Ready(1)`,
/// //  b label: `b`,
/// //  b debug: `Ready(2)`,
/// //        a: `1`,
/// //        b: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_poll_ready_eq!(a, b)`\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ready(1)`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Ready(2)`,\n",
/// #     "       a: `1`,\n",
/// #     "       b: `2`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_poll_ready_eq`](macro@crate::assert_poll_ready_eq)
/// * [`assert_poll_ready_eq_as_result`](macro@crate::assert_poll_ready_eq_as_result)
/// * [`debug_assert_poll_ready_eq`](macro@crate::debug_assert_poll_ready_eq)
///
#[macro_export]
macro_rules! assert_poll_ready_eq {
    ($a_poll:expr, $b_poll:expr $(,)?) => {
        match assert_poll_ready_eq_as_result!($a_poll, $b_poll) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a_poll:expr, $b_poll:expr, $($message:tt)+) => {
        match assert_poll_ready_eq_as_result!($a_poll, $b_poll) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    };
}

/// Assert a poll ready value is equal to another.
///
/// This macro provides the same statements as [`assert_poll_ready_eq`](macro.assert_poll_ready_eq.html),
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
/// * [`assert_poll_ready_eq`](macro@crate::assert_poll_ready_eq)
/// * [`assert_poll_ready_eq`](macro@crate::assert_poll_ready_eq)
/// * [`debug_assert_poll_ready_eq`](macro@crate::debug_assert_poll_ready_eq)
///
#[macro_export]
macro_rules! debug_assert_poll_ready_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_poll_ready_eq!($($arg)*);
        }
    };
}
