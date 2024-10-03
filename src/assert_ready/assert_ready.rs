//! Assert an expression is Ready(_).
//!
//! Pseudocode:<br>
//! a is Ready(_)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//! # fn main() {
//! let a: Poll<i8> = Ready(1);
//! assert_ready!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_ready`](macro@crate::assert_ready)
//! * [`assert_ready_as_result`](macro@crate::assert_ready_as_result)
//! * [`debug_assert_ready`](macro@crate::debug_assert_ready)

/// Assert an expression is Ready(_).
///
/// Pseudocode:<br>
/// a is Ready(_)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_ready`](macro.assert_ready.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ready`](macro@crate::assert_ready)
/// * [`assert_ready_as_result`](macro@crate::assert_ready_as_result)
/// * [`debug_assert_ready`](macro@crate::debug_assert_ready)
///
#[macro_export]
macro_rules! assert_ready_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a => {
                match (a) {
                    Ready(_) => {
                        Ok(())
                    },
                    _ => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_ready!(a)`\n",
                                "https://docs.rs/assertables/8.13.0/assertables/macro.assert_ready.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`",
                            ),
                            stringify!($a),
                            a,
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn test_assert_ready_as_result_x_success() {
        let a: Poll<i8> = Ready(1);
        let result = assert_ready_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_ready_as_result_x_failure() {
        let a: Poll<i8> = Pending;
        let result = assert_ready_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_ready!(a)`\n",
                "https://docs.rs/assertables/8.13.0/assertables/macro.assert_ready.html\n",
                " a label: `a`,\n",
                " a debug: `Pending`",
            )
        );
    }
}

/// Assert an expression is Ready(_).
///
/// Pseudocode:<br>
/// a is Ready(_)
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
/// use std::task::Poll;
/// use std::task::Poll::*;
/// # fn main() {
/// let a: Poll<i8> = Ready(1);
/// assert_ready!(a);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Poll<i8> = Pending;
/// assert_ready!(a);
/// # });
/// // assertion failed: `assert_ready!(a)`
/// // https://docs.rs/assertables/8.13.0/assertables/macro.assert_ready.html
/// //  a label: `a`,
/// //  a debug: `Pending`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_ready!(a)`\n",
/// #     "https://docs.rs/assertables/8.13.0/assertables/macro.assert_ready.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Pending`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ready`](macro@crate::assert_ready)
/// * [`assert_ready_as_result`](macro@crate::assert_ready_as_result)
/// * [`debug_assert_ready`](macro@crate::debug_assert_ready)
///
#[macro_export]
macro_rules! assert_ready {
    ($a:expr $(,)?) => {{
        match $crate::assert_ready_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $($message:tt)+) => {{
        match $crate::assert_ready_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert poll.is_ready() is true.
///
/// Pseudocode:<br>
/// a is Ready(_)
///
/// This macro provides the same statements as [`assert_ready`](macro.assert_ready.html),
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
/// * [`assert_ready`](macro@crate::assert_ready)
/// * [`assert_ready`](macro@crate::assert_ready)
/// * [`debug_assert_ready`](macro@crate::debug_assert_ready)
///
#[macro_export]
macro_rules! debug_assert_ready {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_ready!($($arg)*);
        }
    };
}
