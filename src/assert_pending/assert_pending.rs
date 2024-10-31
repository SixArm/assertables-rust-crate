//! Assert an expression is Pending.
//!
//! Pseudocode:<br>
//! a is Pending
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//! # fn main() {
//! let a: Poll<i8> = Pending;
//! assert_pending!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_pending`](macro@crate::assert_pending)
//! * [`assert_pending_as_result`](macro@crate::assert_pending_as_result)
//! * [`debug_assert_pending`](macro@crate::debug_assert_pending)

/// Assert an expression.is_pending() is true.
///
/// Pseudocode:<br>
/// a is Pending
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_pending`](macro.assert_pending.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_pending`](macro@crate::assert_pending)
/// * [`assert_pending_as_result`](macro@crate::assert_pending_as_result)
/// * [`debug_assert_pending`](macro@crate::debug_assert_pending)
///
#[macro_export]
macro_rules! assert_pending_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a => {
                match (a) {
                    Pending => {
                        Ok(())
                    },
                    _ => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_pending!(a)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_pending.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`",
                                ),
                                stringify!($a),
                                a
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
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn test_assert_pending_as_result_x_success() {
        let a: Poll<i8> = Pending;
        let result = assert_pending_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_pending_as_result_x_failure() {
        let a: Poll<i8> = Ready(1);
        let result = assert_pending_as_result!(a);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_pending!(a)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_pending.html\n",
                " a label: `a`,\n",
                " a debug: `Ready(1)`"
            )
        );
    }
}

/// Assert an expression is Pending.
///
/// Pseudocode:<br>
/// a is Pending
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
/// let a: Poll<i8> = Pending;
/// assert_pending!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Poll<i8> = Ready(1);
/// assert_pending!(a);
/// # });
/// // assertion failed: `assert_pending!(a)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_pending.html
/// //  a label: `a`,
/// //  a debug: `Ready(1)`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_pending!(a)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_pending.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ready(1)`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_pending`](macro@crate::assert_pending)
/// * [`assert_pending_as_result`](macro@crate::assert_pending_as_result)
/// * [`debug_assert_pending`](macro@crate::debug_assert_pending)
///
#[macro_export]
macro_rules! assert_pending {
    ($a:expr $(,)?) => {{
        match $crate::assert_pending_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $($message:tt)+) => {{
        match $crate::assert_pending_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an expression is Pending.
///
/// Pseudocode:<br>
/// a is Pending
///
/// This macro provides the same statements as [`assert_pending`](macro.assert_pending.html),
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
/// * [`assert_pending`](macro@crate::assert_pending)
/// * [`assert_pending`](macro@crate::assert_pending)
/// * [`debug_assert_pending`](macro@crate::debug_assert_pending)
///
#[macro_export]
macro_rules! debug_assert_pending {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_pending!($($arg)*);
        }
    };
}
