//! Assert poll.is_pending() is true.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::task::Poll;
//!
//! # fn main() {
//! let a: Poll<i8> = Poll::Pending;
//! assert_poll_pending!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_poll_pending`](macro@crate::assert_poll_pending)
//! * [`assert_poll_pending_as_result`](macro@crate::assert_poll_pending_as_result)
//! * [`debug_assert_poll_pending`](macro@crate::debug_assert_poll_pending)

/// Assert an expression.is_pending() is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_poll_pending`](macro.assert_poll_pending.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_poll_pending`](macro@crate::assert_poll_pending)
/// * [`assert_poll_pending_as_result`](macro@crate::assert_poll_pending_as_result)
/// * [`debug_assert_poll_pending`](macro@crate::debug_assert_poll_pending)
///
#[macro_export]
macro_rules! assert_poll_pending_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a_val => {
                let is_pending = a_val.is_pending();
                if is_pending {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_poll_pending!(expr)`\n",
                            "        expr label: `{}`,\n",
                            "        expr debug: `{:?}`,\n",
                            " expr.is_pending(): `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                        is_pending,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::task::Poll;

    #[test]
    fn test_assert_poll_pending_as_result_x_success() {
        let a: Poll<i8> = Poll::Pending;
        let result = assert_poll_pending_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_poll_pending_as_result_x_failure() {
        let a: Poll<i8> = Poll::Ready(1);
        let result = assert_poll_pending_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_poll_pending!(expr)`\n",
                "        expr label: `a`,\n",
                "        expr debug: `Ready(1)`,\n",
                " expr.is_pending(): `false`"
            )
        );
    }
}

/// Assert poll.is_pending() is true.
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
/// let a: Poll<i8> = Poll::Pending;
/// assert_poll_pending!(a);
/// //-> ()
///
/// let a: Result<(), i8> = Result::Err(1);
/// // Panic with error message
/// let a: Poll<i8> = Poll::Ready(1);
/// let result = panic::catch_unwind(|| {
/// assert_poll_pending!(a);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_poll_pending!(expr)`\n",
///     "        expr label: `a`,\n",
///     "        expr debug: `Ready(1)`,\n",
///     " expr.is_pending(): `false`",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_poll_pending!(a, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_poll_pending`](macro@crate::assert_poll_pending)
/// * [`assert_poll_pending_as_result`](macro@crate::assert_poll_pending_as_result)
/// * [`debug_assert_poll_pending`](macro@crate::debug_assert_poll_pending)
///
#[macro_export]
macro_rules! assert_poll_pending {
    ($a:expr $(,)?) => ({
        match assert_poll_pending_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $($message:tt)+) => ({
        match assert_poll_pending_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert poll.is_pending() is true.
///
/// This macro provides the same statements as [`assert_poll_pending`](macro.assert_poll_pending.html),
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
/// * [`assert_poll_pending`](macro@crate::assert_poll_pending)
/// * [`assert_poll_pending`](macro@crate::assert_poll_pending)
/// * [`debug_assert_poll_pending`](macro@crate::debug_assert_poll_pending)
///
#[macro_export]
macro_rules! debug_assert_poll_pending {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_poll_pending!($($arg)*);
        }
    };
}
