//! Assert an expression (such as a string) does not contain an expression (such as a substring).
//!
//! Pseudocode:<br>
//! ¬ a.contains(b)
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! // String contains substring?
//! let a = "alfa";
//! let b = "zz";
//! assert_not_contains!(a, b);
//!
//! // Range contains value?
//! let a = 1..5;
//! let b = 6;
//! assert_not_contains!(a, &b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_not_contains`](macro@crate::assert_not_contains)
//! * [`assert_not_contains_as_result`](macro@crate::assert_not_contains_as_result)
//! * [`debug_assert_not_contains`](macro@crate::debug_assert_not_contains)

/// Assert an expression (such as a string) does not contain an expression (such as a substring).
///
/// Pseudocode:<br>
/// ¬ a.contains(b)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_not_contains`](macro.assert_not_contains.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_not_contains`](macro@crate::assert_not_contains)
/// * [`assert_not_contains_as_result`](macro@crate::assert_not_contains_as_result)
/// * [`debug_assert_not_contains`](macro@crate::debug_assert_not_contains)
///
#[macro_export]
macro_rules! assert_not_contains_as_result {
    ($container:expr, $containee:expr $(,)?) => {{
        match (&$container, &$containee) {
            (container, containee) => {
                if !(container.contains($containee)) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_not_contains!(container, containee)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_not_contains.html\n",
                            " container label: `{}`,\n",
                            " container debug: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`",
                        ),
                        stringify!($container),
                        container,
                        stringify!($containee),
                        containee,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    //// str

    #[test]
    fn test_assert_not_contains_as_result_x_str_x_success() {
        let a = "alfa";
        let b = "zz";
        let result = assert_not_contains_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_not_contains_as_result_x_str_x_failure() {
        let a = "alfa";
        let b = "lf";
        let result = assert_not_contains_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_not_contains!(container, containee)`\n",
            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_not_contains.html\n",
            " container label: `a`,\n",
            " container debug: `\"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"lf\"`"
        );
        assert_eq!(actual, expect);
    }

    //// Range

    #[test]
    fn test_assert_not_contains_as_result_x_range_x_success() {
        let a = 1..5;
        let b = 6;
        let result = assert_not_contains_as_result!(a, &b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_not_contains_as_result_x_range_x_failure() {
        let a = 1..5;
        let b = 2;
        let result = assert_not_contains_as_result!(a, &b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_not_contains!(container, containee)`\n",
            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_not_contains.html\n",
            " container label: `a`,\n",
            " container debug: `1..5`,\n",
            " containee label: `&b`,\n",
            " containee debug: `2`"
        );
        assert_eq!(actual, expect);
    }

}

/// Assert an expression (such as a string) does not contain an expression (such as a substring).
///
/// Pseudocode:<br>
/// ¬ a.contains(b)
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
/// // String contains substring?
/// let a = "alfa";
/// let b = "zz";
/// assert_not_contains!(a, b);
///
/// // Range contains value?
/// let a = 1..5;
/// let b = 6;
/// assert_not_contains!(a, &b);
/// //->
///
/// # let result = panic::catch_unwind(|| {
/// let a = "alfa";
/// let b = "lf";
/// assert_not_contains!(a, b);
/// # });
/// // assertion failed: `assert_not_contains!(container, containee)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_not_contains.html
/// //  container label: `a`,
/// //  container debug: `\"alfa\"`,
/// //  containee label: `b`,
/// //  containee debug: `\"lf\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_not_contains!(container, containee)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_not_contains.html\n",
/// #     " container label: `a`,\n",
/// #     " container debug: `\"alfa\"`,\n",
/// #     " containee label: `b`,\n",
/// #     " containee debug: `\"lf\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_not_contains`](macro@crate::assert_not_contains)
/// * [`assert_not_contains_as_result`](macro@crate::assert_not_contains_as_result)
/// * [`debug_assert_not_contains`](macro@crate::debug_assert_not_contains)
///
#[macro_export]
macro_rules! assert_not_contains {
    ($container:expr, $containee:expr $(,)?) => {{
        match assert_not_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($container:expr, $containee:expr, $($message:tt)+) => {{
        match assert_not_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an expression (such as a string) does not contain an expression (such as a substring).
///
/// Pseudocode:<br>
/// ¬ a.contains(b)
///
/// This macro provides the same statements as [`assert_not_contains`](macro.assert_not_contains.html),
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
/// * [`assert_not_contains`](macro@crate::assert_not_contains)
/// * [`assert_not_contains`](macro@crate::assert_not_contains)
/// * [`debug_assert_not_contains`](macro@crate::debug_assert_not_contains)
///
#[macro_export]
macro_rules! debug_assert_not_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_not_contains!($($arg)*);
        }
    };
}
