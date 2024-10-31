//! Assert an expression (such as a string or array) is empty.
//!
//! Pseudocode:<br>
//! a.is_empty()
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = "";
//! assert_is_empty!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_is_empty`](macro@crate::assert_is_empty)
//! * [`assert_is_empty_as_result`](macro@crate::assert_is_empty_as_result)
//! * [`debug_assert_is_empty`](macro@crate::debug_assert_is_empty)

/// Assert an expression (such as a regex) is a match for an expression (such as a string).
///
/// Pseudocode:<br>
/// a.is_empty()
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_is_empty`](macro.assert_is_empty.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_is_empty`](macro@crate::assert_is_empty)
/// * [`assert_is_empty_as_result`](macro@crate::assert_is_empty_as_result)
/// * [`debug_assert_is_empty`](macro@crate::debug_assert_is_empty)
///
#[macro_export]
macro_rules! assert_is_empty_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a => {
                if a.is_empty() {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_is_empty!(a)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_is_empty.html\n",
                                " label: `{}`,\n",
                                " debug: `{:?}`",
                            ),
                            stringify!($a),
                            a,
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_is_empty_as_result_x_success() {
        let a = "";
        let result = assert_is_empty_as_result!(a);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_is_empty_as_result_x_failure() {
        let a = "alfa";
        let result = assert_is_empty_as_result!(a);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_is_empty!(a)`\n",
            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_is_empty.html\n",
            " label: `a`,\n",
            " debug: `\"alfa\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert an expression (such as a string or array) is empty.
///
/// Pseudocode:<br>
/// a.is_empty()
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
///
/// # fn main() {
/// let a = "";
/// assert_is_empty!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "alfa";
/// assert_is_empty!(a);
/// # });
/// // assertion failed: `assert_is_empty!(a)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_is_empty.html
/// //  label: `a`,
/// //  debug: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_is_empty!(a)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_is_empty.html\n",
/// #     " label: `a`,\n",
/// #     " debug: `\"alfa\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_is_empty`](macro@crate::assert_is_empty)
/// * [`assert_is_empty_as_result`](macro@crate::assert_is_empty_as_result)
/// * [`debug_assert_is_empty`](macro@crate::debug_assert_is_empty)
///
#[macro_export]
macro_rules! assert_is_empty {
    ($a:expr $(,)?) => {{
        match $crate::assert_is_empty_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $($message:tt)+) => {{
        match $crate::assert_is_empty_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an expression (such as a string or array) is empty.
///
/// Pseudocode:<br>
/// a.is_empty()
///
/// This macro provides the same statements as [`assert_is_empty`](macro.assert_is_empty.html),
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
/// * [`assert_is_empty`](macro@crate::assert_is_empty)
/// * [`assert_is_empty`](macro@crate::assert_is_empty)
/// * [`debug_assert_is_empty`](macro@crate::debug_assert_is_empty)
///
#[macro_export]
macro_rules! debug_assert_is_empty {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_is_empty!($($arg)*);
        }
    };
}
