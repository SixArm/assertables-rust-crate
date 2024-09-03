//! Assert a value is less than an expression.
//!
//! * If true, return `()`.
//!
//! * Otherwise, call [`panic!`] with a message and the values of the
//!   expressions with their debug representations.
//!
//! # Examples
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! // Return Ok
//! let a = 1;
//! let b = 2;
//! assert_lt!(a, b);
//! //-> ()
//!
//! let a = 2;
//! let b = 1;
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_lt!(a, b);
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = concat!(
//!     "assertion failed: `assert_lt!(left, right)`\n",
//!     "  left label: `a`,\n",
//!     "  left debug: `2`,\n",
//!     " right label: `b`,\n",
//!     " right debug: `1`,\n",
//!     "        left: `2`,\n",
//!     "       right: `1`"
//! );
//! assert_eq!(actual, expect);
//!
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_lt!(a, b, "message");
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = "message";
//! assert_eq!(actual, expect);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_lt`](macro@crate::assert_lt)
//! * [`assert_lt_as_result`](macro@crate::assert_lt_as_result)
//! * [`debug_assert_lt`](macro@crate::debug_assert_lt)

/// Assert a value is less than an expression.
///
/// * If true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_lt`](macro@crate::assert_lt)
/// * [`assert_lt_as_result`](macro@crate::assert_lt_as_result)
/// * [`debug_assert_lt`](macro@crate::debug_assert_lt)
///
#[macro_export]
macro_rules! assert_lt_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val < b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_lt!(left, right)`\n",
                            "  left label: `{}`,\n",
                            "  left debug: `{:?}`,\n",
                            " right label: `{}`,\n",
                            " right debug: `{:?}`,\n",
                            "        left: `{:?}`,\n",
                            "       right: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_lt_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 2;
        let x = assert_lt_as_result!(a, b);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_lt_as_result_x_failure() {
        let a: i32 = 2;
        let b: i32 = 1;
        let x = assert_lt_as_result!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_lt!(left, right)`\n",
                "  left label: `a`,\n",
                "  left debug: `2`,\n",
                " right label: `b`,\n",
                " right debug: `1`,\n",
                "        left: `2`,\n",
                "       right: `1`"
            )
        );
    }
}

/// Assert a value is less than an expression.
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
/// # fn main() {
/// // Return Ok
/// let a = 1;
/// let b = 2;
/// assert_lt!(a, b);
/// //-> ()
///
/// let a = 2;
/// let b = 1;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_lt!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_lt!(left, right)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `2`,\n",
///     " right label: `b`,\n",
///     " right debug: `1`,\n",
///     "        left: `2`,\n",
///     "       right: `1`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_lt!(a, b, "message");
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
/// * [`assert_lt`](macro@crate::assert_lt)
/// * [`assert_lt_as_result`](macro@crate::assert_lt_as_result)
/// * [`debug_assert_lt`](macro@crate::debug_assert_lt)
///
#[macro_export]
macro_rules! assert_lt {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_lt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_lt_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a value is less than an expression.
///
/// This macro provides the same statements as [`assert_lt`](macro.assert_lt.html),
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
/// * [`assert_lt`](macro@crate::assert_lt)
/// * [`assert_lt`](macro@crate::assert_lt)
/// * [`debug_assert_lt`](macro@crate::debug_assert_lt)
///
#[macro_export]
macro_rules! debug_assert_lt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_lt!($($arg)*);
        }
    };
}
